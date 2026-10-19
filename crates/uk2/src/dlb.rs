//! UK2 `DLB` archive reader.

use std::path::Path;

use anyhow::{Context, Result, bail};
use encoding_rs::SHIFT_JIS;

const DLB_MAGIC: &[u8; 22] = b"<< dlb file Ver1.00>>\0";
const DLB_ENTRY_SIZE: usize = 0x15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DlbEntry {
    pub name: String,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct DlbArchive {
    bytes: Vec<u8>,
    entries: Vec<DlbEntry>,
}

impl DlbArchive {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read UK2 DLB {}", path.display()))?;
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        if bytes.len() < DLB_MAGIC.len() + 2 {
            bail!("uk2: DLB is smaller than its header");
        }
        if bytes.get(..DLB_MAGIC.len()) != Some(DLB_MAGIC.as_slice()) {
            bail!("uk2: expected DLB Ver1.00 header");
        }
        let count_at = DLB_MAGIC.len();
        let count = u16::from_le_bytes([bytes[count_at], bytes[count_at + 1]]) as usize;
        let table_at = count_at + 2;
        let table_len = count
            .checked_mul(DLB_ENTRY_SIZE)
            .ok_or_else(|| anyhow::anyhow!("uk2: DLB entry table overflows"))?;
        let table_end = table_at
            .checked_add(table_len)
            .ok_or_else(|| anyhow::anyhow!("uk2: DLB entry table overflows"))?;
        if table_end > bytes.len() {
            bail!("uk2: DLB entry table exceeds archive length");
        }

        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let at = table_at + index * DLB_ENTRY_SIZE;
            let raw_name = &bytes[at..at + 0x0d];
            let name_end = raw_name
                .iter()
                .position(|&byte| byte == 0)
                .unwrap_or(raw_name.len());
            let (name, _, had_errors) = SHIFT_JIS.decode(&raw_name[..name_end]);
            if had_errors {
                bail!("uk2: DLB entry {index} has invalid Shift-JIS name");
            }
            let offset = read_u32(&bytes, at + 0x0d)?;
            let size = read_u32(&bytes, at + 0x11)?;
            let end = (offset as usize)
                .checked_add(size as usize)
                .ok_or_else(|| anyhow::anyhow!("uk2: DLB entry range overflows"))?;
            if (offset as usize) < table_end || end > bytes.len() {
                bail!("uk2: DLB entry {name:?} lies outside archive data");
            }
            entries.push(DlbEntry {
                name: name.into_owned(),
                offset,
                size,
            });
        }
        Ok(Self { bytes, entries })
    }

    pub fn entries(&self) -> &[DlbEntry] {
        &self.entries
    }

    pub fn entry(&self, name: &str) -> Option<&DlbEntry> {
        self.entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
    }

    pub fn read(&self, name: &str) -> Result<&[u8]> {
        let entry = self
            .entry(name)
            .ok_or_else(|| anyhow::anyhow!("uk2: DLB entry {name:?} was not found"))?;
        let start = entry.offset as usize;
        Ok(&self.bytes[start..start + entry.size as usize])
    }
}

fn read_u32(bytes: &[u8], at: usize) -> Result<u32> {
    let raw = bytes
        .get(at..at + 4)
        .ok_or_else(|| anyhow::anyhow!("uk2: truncated u32 at {at:#x}"))?;
    Ok(u32::from_le_bytes(raw.try_into().expect("four-byte slice")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_reference_dlb_layout() {
        let table_end = DLB_MAGIC.len() + 2 + DLB_ENTRY_SIZE;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(DLB_MAGIC);
        bytes.extend_from_slice(&1u16.to_le_bytes());
        let mut name = [0u8; 0x0d];
        name[..5].copy_from_slice(b"A.MES");
        bytes.extend_from_slice(&name);
        bytes.extend_from_slice(&(table_end as u32).to_le_bytes());
        bytes.extend_from_slice(&3u32.to_le_bytes());
        bytes.extend_from_slice(b"abc");
        let dlb = DlbArchive::from_bytes(bytes).unwrap();
        assert_eq!(dlb.read("a.mes").unwrap(), b"abc");
    }
}
