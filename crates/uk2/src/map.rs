//! UK2 `MAP` resource reader.
//!
//! Sorcer Kingdom `MAP` files act as scene/resource layout tables.  The first
//! four bytes are the map dimensions, followed by two 16-byte NUL-terminated
//! Shift-JIS resource names. Map records begin at offset 0x24.

use anyhow::{Context, Result, bail};
use encoding_rs::SHIFT_JIS;
use std::path::Path;

pub const UK2_MAP_HEADER_SIZE: usize = 0x24;
const RESOURCE_NAME_SIZE: usize = 16;
const MAP_DATA_HEADER_SIZE: usize = 22;
const ENTITY_SIZE: usize = 0x11;
const TRIGGER_SIZE: usize = 0x0a;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uk2MapEntity {
    raw: [u8; ENTITY_SIZE],
}

impl Uk2MapEntity {
    pub fn id(&self) -> u16 {
        u16::from_le_bytes([self.raw[6], self.raw[7]])
    }

    pub fn position(&self) -> (u16, u16) {
        (
            u16::from_le_bytes([self.raw[0], self.raw[1]]),
            u16::from_le_bytes([self.raw[2], self.raw[3]]),
        )
    }

    /// `sub_1BD89`: 1000 leaves an existing field unchanged. The final value
    /// replaces the low attribute nibble; every matching update toggles flag 1.
    pub fn apply_fa(&mut self, values: [u16; 5]) {
        for (offset, value) in [(0, values[0]), (2, values[1])] {
            if value != 1000 {
                self.raw[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
            }
        }
        if values[2] != 1000 {
            self.raw[0x0c] = values[2] as u8;
        }
        self.raw[0x0e] = (self.raw[0x0e] & 0xf0) | ((values[4] as u8) & 0x0f);
        let flags = u16::from_le_bytes([self.raw[4], self.raw[5]]) ^ 2;
        self.raw[4..6].copy_from_slice(&flags.to_le_bytes());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uk2MapLayout {
    pub width: u16,
    pub height: u16,
    pub base: String,
    pub overlay: String,
    map_data_header: [u8; MAP_DATA_HEADER_SIZE],
    tiles: Vec<u16>,
    entities: Vec<Uk2MapEntity>,
    triggers: Vec<[u8; TRIGGER_SIZE]>,
    payload: Vec<u8>,
}

impl Uk2MapLayout {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read UK2 MAP {}", path.display()))?;
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        if bytes.len() < UK2_MAP_HEADER_SIZE + MAP_DATA_HEADER_SIZE {
            bail!("uk2: MAP is smaller than its header");
        }
        let width = u16::from_le_bytes([bytes[0], bytes[1]]);
        let height = u16::from_le_bytes([bytes[2], bytes[3]]);
        let base = decode_name(&bytes[4..4 + RESOURCE_NAME_SIZE])?;
        let overlay = decode_name(&bytes[4 + RESOURCE_NAME_SIZE..UK2_MAP_HEADER_SIZE])?;
        let map_data_header: [u8; MAP_DATA_HEADER_SIZE] = bytes
            [UK2_MAP_HEADER_SIZE..UK2_MAP_HEADER_SIZE + MAP_DATA_HEADER_SIZE]
            .try_into()
            .expect("slice length checked");
        let tile_data_bytes =
            usize::from(u16::from_le_bytes([map_data_header[0], map_data_header[1]]));
        let expected_tile_data_bytes = usize::from(width)
            .checked_mul(usize::from(height))
            .and_then(|cells| cells.checked_mul(2))
            .context("uk2: MAP dimensions overflow tile data size")?;
        if tile_data_bytes != expected_tile_data_bytes {
            bail!(
                "uk2: MAP tile byte count is {tile_data_bytes}, expected {expected_tile_data_bytes} for {width}x{height}"
            );
        }
        let tile_start = UK2_MAP_HEADER_SIZE + MAP_DATA_HEADER_SIZE;
        let tile_end = tile_start
            .checked_add(tile_data_bytes)
            .context("uk2: MAP tile data end overflows")?;
        if tile_end > bytes.len() {
            bail!(
                "uk2: MAP declares {tile_data_bytes} tile bytes but only {} remain",
                bytes.len() - tile_start
            );
        }
        let tiles = bytes[tile_start..tile_end]
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        let entity_count = usize::from(map_data_header[2]);
        let trigger_count = usize::from(map_data_header[4]);
        let expected_tail = entity_count * ENTITY_SIZE + trigger_count * TRIGGER_SIZE;
        if bytes.len() - tile_end != expected_tail {
            bail!(
                "uk2: MAP has {} tail bytes, expected {expected_tail} for {entity_count} entities and {trigger_count} triggers",
                bytes.len() - tile_end
            );
        }
        let entity_end = tile_end + entity_count * ENTITY_SIZE;
        let entities = bytes[tile_end..entity_end]
            .chunks_exact(ENTITY_SIZE)
            .map(|chunk| Uk2MapEntity {
                raw: chunk.try_into().expect("entity chunk size"),
            })
            .collect();
        let triggers = bytes[entity_end..]
            .chunks_exact(TRIGGER_SIZE)
            .map(|chunk| chunk.try_into().expect("trigger chunk size"))
            .collect();
        Ok(Self {
            width,
            height,
            base,
            overlay,
            map_data_header,
            tiles,
            entities,
            triggers,
            payload: bytes[UK2_MAP_HEADER_SIZE..].to_vec(),
        })
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// The 22-byte per-map header immediately following the shared MAP header.
    /// Its first little-endian word is the tile-grid byte count.
    pub fn map_data_header(&self) -> &[u8; MAP_DATA_HEADER_SIZE] {
        &self.map_data_header
    }

    /// Row-major little-endian tile values, one `u16` per map cell.
    pub fn tiles(&self) -> &[u16] {
        &self.tiles
    }

    pub fn entities(&self) -> &[Uk2MapEntity] {
        &self.entities
    }

    pub fn triggers(&self) -> &[[u8; TRIGGER_SIZE]] {
        &self.triggers
    }

    /// Returns whether an entity in this MAP matched. The original first
    /// searches a separate global entity list, then the loaded MAP objects.
    pub fn apply_fa(&mut self, values: [u16; 5]) -> bool {
        if let Some((index, entity)) = self
            .entities
            .iter_mut()
            .enumerate()
            .find(|(_, entity)| entity.id() == values[3])
        {
            entity.apply_fa(values);
            let start = MAP_DATA_HEADER_SIZE + self.tiles.len() * 2 + index * ENTITY_SIZE;
            self.payload[start..start + ENTITY_SIZE].copy_from_slice(&entity.raw);
            return true;
        }
        false
    }

    /// `sub_1C2E3` changes one map cell. Out-of-range chip or layer values
    /// preserve that component of the existing word, as in the executable.
    pub fn apply_fb(&mut self, x: u16, y: u16, chip: u16, layer: u16) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = usize::from(y) * usize::from(self.width) + usize::from(x);
        let previous = self.tiles[index];
        let chip = if chip <= 0x01ff {
            chip
        } else {
            previous & 0x01ff
        };
        let layer = if layer <= 3 {
            layer
        } else {
            (previous >> 9) & 3
        };
        let updated = (previous & 0xf800) | (layer << 9) | chip;
        self.tiles[index] = updated;
        let start = MAP_DATA_HEADER_SIZE + index * 2;
        self.payload[start..start + 2].copy_from_slice(&updated.to_le_bytes());
        true
    }

    /// `FC` and `FD` find the first 10-byte trigger whose word at +4 matches
    /// the supplied ID, then set byte +8 to zero or one respectively.
    pub fn set_trigger_enabled(&mut self, id: u16, enabled: bool) -> bool {
        if let Some((index, trigger)) = self
            .triggers
            .iter_mut()
            .enumerate()
            .find(|(_, trigger)| u16::from_le_bytes([trigger[4], trigger[5]]) == id)
        {
            trigger[8] = u8::from(enabled);
            let start = MAP_DATA_HEADER_SIZE
                + self.tiles.len() * 2
                + self.entities.len() * ENTITY_SIZE
                + index * TRIGGER_SIZE;
            self.payload[start + 8] = trigger[8];
            return true;
        }
        false
    }

    pub fn cell_count(&self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }
}

fn decode_name(bytes: &[u8]) -> Result<String> {
    let end = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    let end = bytes[..end]
        .iter()
        .rposition(|&byte| byte != b' ')
        .map(|index| index + 1)
        .unwrap_or(0);
    let (name, _, had_errors) = SHIFT_JIS.decode(&bytes[..end]);
    if had_errors {
        bail!("uk2: MAP contains invalid Shift-JIS");
    }
    Ok(name.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_layout_header() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        let mut base = [0u8; RESOURCE_NAME_SIZE];
        base[..11].copy_from_slice(b"chip06.pdt1");
        bytes.extend_from_slice(&base);
        let mut overlay = [0u8; RESOURCE_NAME_SIZE];
        overlay[..13].copy_from_slice(b"chara03a.pdt1");
        bytes.extend_from_slice(&overlay);
        bytes.extend_from_slice(&4u16.to_le_bytes());
        bytes.extend_from_slice(&[0; MAP_DATA_HEADER_SIZE - 2]);
        bytes.extend_from_slice(&0x1234u16.to_le_bytes());
        bytes.extend_from_slice(&0x5678u16.to_le_bytes());
        let map = Uk2MapLayout::from_bytes(bytes).unwrap();
        assert_eq!(map.width, 2);
        assert_eq!(map.height, 1);
        assert_eq!(map.base, "chip06.pdt1");
        assert_eq!(map.overlay, "chara03a.pdt1");
        assert_eq!(map.tiles(), [0x1234, 0x5678]);
        assert_eq!(map.payload().len(), MAP_DATA_HEADER_SIZE + 4);
    }
}
