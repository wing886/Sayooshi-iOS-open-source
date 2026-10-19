//! One compiled scenario (`SEENnnnn.TXT`): header and decoded code.
//!
//! Header layout (all fields little-endian `i32`):
//!
//! | offset | field |
//! | --- | --- |
//! | 0x00 | header size, always 0x1d0 |
//! | 0x04 | compiler version: 10002, or 110002/1110002 with second-level XOR |
//! | 0x08 / 0x0c | kidoku table offset / count |
//! | 0x14 / 0x18 | dramatis personae offset / count |
//! | 0x1c | dramatis personae byte length |
//! | 0x20 | compressed code offset |
//! | 0x24 | decompressed code length |
//! | 0x28 | compressed code length |
//! | 0x2c / 0x30 | debug entrypoints `Z-1`, `Z-2` |
//! | 0x1c4 / 0x1c8 / 0x1cc | savepoint attributes: message, selcom, seentop (0 default, 1 on, 2 off) |
//!
//! Between the dramatis personae and the code, RLdev stores a metadata
//! block whose byte at `id_len + 16` names the text encoding.

use anyhow::{Result, bail};

use crate::bytecode::Script;
use crate::compression::{self, Xor2Key};
use crate::nls::Nls;

pub const HEADER_SIZE: usize = 0x1d0;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Header {
    pub compiler_version: i32,
    pub uses_xor2: bool,
    pub kidoku_table: Vec<i32>,
    pub dramatis_personae: Vec<Vec<u8>>,
    pub z_minus_one: i32,
    pub z_minus_two: i32,
    pub savepoint_message: i32,
    pub savepoint_selcom: i32,
    pub savepoint_seentop: i32,
    /// The encoding named by RLdev metadata, if present.
    pub rldev_encoding: Option<Nls>,
    code_offset: usize,
    code_packed: usize,
    code_len: usize,
}

fn i32_at(data: &[u8], offset: usize) -> Result<i32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| anyhow::anyhow!("reallive: scenario header is truncated at 0x{offset:x}"))?;
    Ok(i32::from_le_bytes(bytes.try_into().expect("four bytes")))
}

impl Header {
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < HEADER_SIZE {
            bail!("reallive: not a RealLive scenario ({} bytes)", data.len());
        }
        if i32_at(data, 0)? != HEADER_SIZE as i32 {
            bail!(
                "reallive: unsupported scenario header size {}",
                i32_at(data, 0)?
            );
        }
        let compiler_version = i32_at(data, 4)?;
        let uses_xor2 = match compiler_version {
            10002 => false,
            110002 | 1110002 => true,
            other => bail!("reallive: unsupported compiler version {other}"),
        };
        let kidoku_offset = i32_at(data, 0x08)? as usize;
        let kidoku_count = i32_at(data, 0x0c)?.max(0) as usize;
        let kidoku_table = (0..kidoku_count)
            .map(|i| i32_at(data, kidoku_offset + i * 4))
            .collect::<Result<Vec<_>>>()?;
        let mut dramatis_personae = Vec::new();
        let mut offset = i32_at(data, 0x14)? as usize;
        for _ in 0..i32_at(data, 0x18)?.max(0) {
            let length = i32_at(data, offset)?.max(0) as usize;
            let name = data
                .get(offset + 4..offset + 4 + length.saturating_sub(1))
                .unwrap_or(&[])
                .to_vec();
            dramatis_personae.push(name);
            offset += length + 4;
        }
        let code_offset = i32_at(data, 0x20)? as usize;
        let metadata_offset = (i32_at(data, 0x14)? + i32_at(data, 0x1c)?) as usize;
        let rldev_encoding = if metadata_offset != code_offset {
            (|| {
                let meta_len = i32_at(data, metadata_offset).ok()?;
                let id_len = i32_at(data, metadata_offset + 4).ok()? + 1;
                if meta_len < id_len + 17 {
                    return None;
                }
                let code = *data.get(metadata_offset + id_len as usize + 16)?;
                Nls::from_rldev(code)
            })()
        } else {
            None
        };
        Ok(Self {
            compiler_version,
            uses_xor2,
            kidoku_table,
            dramatis_personae,
            z_minus_one: i32_at(data, 0x2c)?,
            z_minus_two: i32_at(data, 0x30)?,
            savepoint_message: i32_at(data, 0x1c4)?,
            savepoint_selcom: i32_at(data, 0x1c8)?,
            savepoint_seentop: i32_at(data, 0x1cc)?,
            rldev_encoding,
            code_offset,
            code_packed: i32_at(data, 0x28)?.max(0) as usize,
            code_len: i32_at(data, 0x24)?.max(0) as usize,
        })
    }

    /// The first-level-decompressed code (second-level XOR not applied).
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let packed = data
            .get(self.code_offset..(self.code_offset + self.code_packed).min(data.len()))
            .ok_or_else(|| anyhow::anyhow!("reallive: scenario code lies outside the file"))?;
        compression::decompress(packed, self.code_len)
    }
}

#[derive(Debug, Clone)]
pub struct Scenario {
    pub number: i32,
    pub header: Header,
    pub script: Script,
    /// Encoding of this scenario's text.
    pub nls: Nls,
}

impl Scenario {
    pub fn parse(number: i32, data: &[u8], xor2: Option<&Xor2Key>, nls: Nls) -> Result<Self> {
        let header = Header::parse(data)?;
        let mut code = header.decompress(data)?;
        if header.uses_xor2 {
            match xor2 {
                Some(key) => key.apply(&mut code),
                None => bail!(
                    "reallive: SEEN{number:04} needs a second-level key that is not known for this title"
                ),
            }
        }
        let nls = header.rldev_encoding.unwrap_or(nls);
        let script = Script::parse(&code, &header.kidoku_table, nls)
            .map_err(|error| error.context(format!("in SEEN{number:04}")))?;
        Ok(Self {
            number,
            header,
            script,
            nls,
        })
    }

    pub fn entrypoint(&self, number: i32) -> Option<usize> {
        self.script.entrypoint(number)
    }
}

/// Assembles a scenario file around `code` (tests and tools).
pub fn build(code: &[u8], kidoku_table: &[i32], dramatis_personae: &[&[u8]]) -> Vec<u8> {
    let mut out = vec![0u8; HEADER_SIZE];
    let put = |out: &mut Vec<u8>, offset: usize, value: i32| {
        out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    };
    put(&mut out, 0, HEADER_SIZE as i32);
    put(&mut out, 4, 10002);
    let kidoku_offset = out.len() as i32;
    put(&mut out, 0x08, kidoku_offset);
    put(&mut out, 0x0c, kidoku_table.len() as i32);
    for value in kidoku_table {
        out.extend_from_slice(&value.to_le_bytes());
    }
    let dramatis_offset = out.len();
    put(&mut out, 0x14, dramatis_offset as i32);
    put(&mut out, 0x18, dramatis_personae.len() as i32);
    for name in dramatis_personae {
        out.extend_from_slice(&(name.len() as i32 + 1).to_le_bytes());
        out.extend_from_slice(name);
        out.push(0);
    }
    let dramatis_len = (out.len() - dramatis_offset) as i32;
    put(&mut out, 0x1c, dramatis_len);
    let packed = compression::compress_literal(code);
    let code_offset = out.len() as i32;
    put(&mut out, 0x20, code_offset);
    put(&mut out, 0x24, code.len() as i32);
    put(&mut out, 0x28, packed.len() as i32);
    out.extend(packed);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::{Element, asm};

    #[test]
    fn builds_and_parses_a_scenario() {
        let mut code = asm::marker(0);
        code.extend(b"\x82\xa0");
        let data = build(&code, &[1_000_000], &[b"\x88\xa2"]);
        let scenario = Scenario::parse(1, &data, None, Nls::Sjis).unwrap();
        assert_eq!(
            scenario.header.dramatis_personae,
            vec![b"\x88\xa2".to_vec()]
        );
        assert_eq!(scenario.entrypoint(0), Some(0));
        assert!(matches!(scenario.script.elements[1], Element::Textout(_)));
    }
}
