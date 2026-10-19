//! UK2 `MES` program container and byte reader.

use std::path::Path;

use anyhow::{Context, Result, bail};

pub const MES_MAGIC: &[u8; 23] = b"<< UK2 TEXT Ver1.00 >>\0";
pub const MES_CODE_OFFSET: usize = MES_MAGIC.len();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MesProgram {
    bytes: Vec<u8>,
}

impl MesProgram {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read UK2 MES {}", path.display()))?;
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        if bytes.len() < MES_CODE_OFFSET {
            bail!("uk2: MES is smaller than the 0x17-byte header");
        }
        if bytes.get(..MES_CODE_OFFSET) != Some(MES_MAGIC.as_slice()) {
            bail!("uk2: expected << UK2 TEXT Ver1.00 >> MES header");
        }
        Ok(Self { bytes })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn code(&self) -> &[u8] {
        &self.bytes[MES_CODE_OFFSET..]
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code().is_empty()
    }

    pub(crate) fn reader_at(&self, at: usize) -> Result<MesReader<'_>> {
        MesReader::new(&self.bytes, at)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MesReader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> MesReader<'a> {
    pub(crate) fn new(bytes: &'a [u8], at: usize) -> Result<Self> {
        if at > bytes.len() {
            bail!(
                "uk2: MES offset {at:#x} exceeds file length {:#x}",
                bytes.len()
            );
        }
        Ok(Self { bytes, at })
    }

    pub(crate) fn position(&self) -> usize {
        self.at
    }

    pub(crate) fn byte(&mut self) -> Result<u8> {
        let byte = self
            .bytes
            .get(self.at)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("uk2: MES ends at offset {:#x}", self.at))?;
        self.at += 1;
        Ok(byte)
    }

    pub(crate) fn u16(&mut self) -> Result<u16> {
        let lo = self.byte()?;
        let hi = self.byte()?;
        Ok(u16::from_le_bytes([lo, hi]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_reference_header() {
        let mut bytes = MES_MAGIC.to_vec();
        bytes.extend_from_slice(&[0, 1, 2]);
        let mes = MesProgram::from_bytes(bytes).unwrap();
        assert_eq!(MES_CODE_OFFSET, 0x17);
        assert_eq!(mes.code(), [0, 1, 2]);
    }
}
