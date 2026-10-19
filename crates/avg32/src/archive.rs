//! AVG32 `PACL` archive reader.

use std::path::Path;

use anyhow::{Context, Result, bail};

const HEADER_SIZE: usize = 0x20;
const ENTRY_SIZE: usize = 0x20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaclEntry {
    pub name: String,
    pub offset: u32,
    pub packed_size: u32,
    pub unpacked_size: u32,
}

#[derive(Debug, Clone)]
pub struct PaclArchive {
    bytes: Vec<u8>,
    entries: Vec<PaclEntry>,
}

impl PaclArchive {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = game_fs::read(path)
            .with_context(|| format!("failed to read AVG32 PACL archive {}", path.display()))?;
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        if bytes.len() < HEADER_SIZE {
            bail!("avg32: PACL archive is smaller than its header");
        }
        if &bytes[..4] != b"PACL" {
            bail!("avg32: expected PACL archive magic");
        }

        let count = read_u32(&bytes, 0x10)? as usize;
        let table_end = HEADER_SIZE
            .checked_add(
                count
                    .checked_mul(ENTRY_SIZE)
                    .ok_or_else(|| anyhow::anyhow!("avg32: PACL entry table overflows"))?,
            )
            .ok_or_else(|| anyhow::anyhow!("avg32: PACL entry table overflows"))?;
        if table_end > bytes.len() {
            bail!("avg32: PACL entry table exceeds archive size");
        }

        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let at = HEADER_SIZE + index * ENTRY_SIZE;
            let name_bytes = &bytes[at..at + 0x10];
            let name_end = name_bytes
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(name_bytes.len());
            if name_end == 0 {
                bail!("avg32: PACL entry {index} has an empty name");
            }
            let name = crate::nls::decode_name(&name_bytes[..name_end]);
            let offset = read_u32(&bytes, at + 0x10)?;
            let packed_size = read_u32(&bytes, at + 0x14)?;
            let unpacked_size = read_u32(&bytes, at + 0x18)?;
            let end = (offset as usize)
                .checked_add(packed_size as usize)
                .ok_or_else(|| anyhow::anyhow!("avg32: PACL entry {name} range overflows"))?;
            if (offset as usize) < table_end || end > bytes.len() {
                bail!("avg32: PACL entry {name} is outside archive data");
            }
            entries.push(PaclEntry {
                name,
                offset,
                packed_size,
                unpacked_size,
            });
        }

        Ok(Self { bytes, entries })
    }

    pub fn entries(&self) -> &[PaclEntry] {
        &self.entries
    }

    pub fn entry(&self, name: &str) -> Option<&PaclEntry> {
        let find = |name: &str| {
            self.entries
                .iter()
                .find(|entry| entry.name.eq_ignore_ascii_case(name))
        };
        find(name).or_else(|| find(&crate::nls::sjis_fallback(name)?))
    }

    /// Reads an entry and expands its `PACK` payload when it is compressed.
    pub fn read(&self, name: &str) -> Result<Vec<u8>> {
        let entry = self
            .entry(name)
            .ok_or_else(|| anyhow::anyhow!("avg32: PACL entry {name:?} was not found"))?;
        self.read_entry(entry)
    }

    pub fn read_entry(&self, entry: &PaclEntry) -> Result<Vec<u8>> {
        let start = entry.offset as usize;
        let end = start + entry.packed_size as usize;
        let packed = &self.bytes[start..end];
        if entry.packed_size == entry.unpacked_size {
            return Ok(packed.to_vec());
        }
        if packed.len() < 0x10 || &packed[..4] != b"PACK" {
            bail!(
                "avg32: compressed PACL entry {} has no PACK header",
                entry.name
            );
        }
        let declared_unpacked = read_u32(packed, 8)? as usize;
        let declared_packed = read_u32(packed, 12)? as usize;
        if declared_unpacked != entry.unpacked_size as usize || declared_packed != packed.len() {
            bail!("avg32: PACK sizes disagree with PACL entry {}", entry.name);
        }
        unpack_lz(&packed[0x10..], declared_unpacked)
            .with_context(|| format!("failed to decompress AVG32 PACL entry {}", entry.name))
    }
}

/// Expands AVG32's MSB-first LZ stream.  It is also used for `PACK` payloads.
pub fn unpack_lz(input: &[u8], output_len: usize) -> Result<Vec<u8>> {
    let mut input_at = 0usize;
    let mut output = Vec::with_capacity(output_len);

    while output.len() < output_len {
        let flags = *input
            .get(input_at)
            .ok_or_else(|| anyhow::anyhow!("avg32: LZ stream ended before flags"))?;
        input_at += 1;
        for bit in 0..8 {
            if output.len() == output_len {
                break;
            }
            if flags & (0x80 >> bit) != 0 {
                output.push(
                    *input
                        .get(input_at)
                        .ok_or_else(|| anyhow::anyhow!("avg32: LZ literal runs past input"))?,
                );
                input_at += 1;
                continue;
            }

            let lo = *input
                .get(input_at)
                .ok_or_else(|| anyhow::anyhow!("avg32: LZ match runs past input"))?;
            let hi = *input
                .get(input_at + 1)
                .ok_or_else(|| anyhow::anyhow!("avg32: LZ match runs past input"))?;
            input_at += 2;
            let token = u16::from_le_bytes([lo, hi]);
            let count = usize::from((token & 0x0f) + 2);
            let distance = usize::from(token >> 4) + 1;
            if distance > output.len() {
                bail!("avg32: LZ match refers before output start");
            }
            for _ in 0..count {
                if output.len() == output_len {
                    break;
                }
                let byte = output[output.len() - distance];
                output.push(byte);
            }
        }
    }
    Ok(output)
}

fn read_u32(bytes: &[u8], at: usize) -> Result<u32> {
    let slice = bytes
        .get(at..at + 4)
        .ok_or_else(|| anyhow::anyhow!("avg32: truncated u32 at {at:#x}"))?;
    Ok(u32::from_le_bytes(
        slice.try_into().expect("slice has four bytes"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_lz_supports_literals_and_overlapping_matches() {
        // Five literals (HELLO), then a five-byte match from five bytes back.
        let bytes = [0b1111_1000, b'H', b'E', b'L', b'L', b'O', 0x43, 0x00];
        assert_eq!(unpack_lz(&bytes, 10).unwrap(), b"HELLOHELLO");
    }

    #[test]
    fn parses_and_reads_a_pacl_archive() {
        let mut bytes = vec![0u8; HEADER_SIZE + ENTRY_SIZE];
        bytes[..4].copy_from_slice(b"PACL");
        bytes[0x10..0x14].copy_from_slice(&1u32.to_le_bytes());
        bytes[HEADER_SIZE..HEADER_SIZE + 8].copy_from_slice(b"A.TXT\0\0\0");
        let offset = (HEADER_SIZE + ENTRY_SIZE) as u32;
        bytes[HEADER_SIZE + 0x10..HEADER_SIZE + 0x14].copy_from_slice(&offset.to_le_bytes());
        bytes[HEADER_SIZE + 0x14..HEADER_SIZE + 0x18].copy_from_slice(&3u32.to_le_bytes());
        bytes[HEADER_SIZE + 0x18..HEADER_SIZE + 0x1c].copy_from_slice(&3u32.to_le_bytes());
        bytes.extend_from_slice(b"yes");
        let archive = PaclArchive::from_bytes(bytes).unwrap();
        assert_eq!(archive.read("a.txt").unwrap(), b"yes");
    }
}
