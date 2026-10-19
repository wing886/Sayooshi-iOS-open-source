//! AVG32 `.ARD` click-area maps.

use anyhow::{Result, bail};

/// A byte-per-pixel area-id map, scaled by AVG32 from its authored dimensions
/// into the fixed 640×480 scenario coordinate space.
#[derive(Debug, Clone)]
pub struct AreaMap {
    width: usize,
    height: usize,
    ids: Vec<u8>,
}

impl AreaMap {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 0x120 {
            bail!("avg32: ARD file is shorter than its header");
        }
        let width = u32::from_le_bytes(bytes[8..12].try_into().expect("four bytes")) as usize;
        let height = u32::from_le_bytes(bytes[12..16].try_into().expect("four bytes")) as usize;
        let length = width
            .checked_mul(height)
            .ok_or_else(|| anyhow::anyhow!("avg32: ARD dimensions overflow"))?;
        let ids = bytes
            .get(0x120..0x120 + length)
            .ok_or_else(|| anyhow::anyhow!("avg32: ARD map is truncated"))?
            .to_vec();
        Ok(Self { width, height, ids })
    }

    pub fn area_at(&self, x: i32, y: i32) -> u8 {
        if !(0..640).contains(&x) || !(0..480).contains(&y) {
            return 0;
        }
        let x = x as usize * self.width / 640;
        let y = y as usize * self.height / 480;
        self.ids[y * self.width + x]
    }

    pub fn area_ids(&self) -> Vec<u8> {
        let mut ids = self.ids.clone();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    pub fn first_point_for(&self, wanted: u8) -> Option<(i32, i32)> {
        let index = self.ids.iter().position(|id| *id == wanted)?;
        let x = index % self.width;
        let y = index / self.width;
        // `area_at` uses floor division. Its inverse must round upward or a
        // point on an authored-cell boundary can land in the preceding cell.
        Some((
            (x * 640).div_ceil(self.width).min(639) as i32,
            (y * 480).div_ceil(self.height).min(479) as i32,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_fixed_scenario_coordinates() {
        let mut bytes = vec![0; 0x120 + 4];
        bytes[8..12].copy_from_slice(&2u32.to_le_bytes());
        bytes[12..16].copy_from_slice(&2u32.to_le_bytes());
        bytes[0x120..].copy_from_slice(&[1, 2, 3, 4]);
        let map = AreaMap::parse(&bytes).unwrap();
        assert_eq!(map.area_at(0, 0), 1);
        assert_eq!(map.area_at(639, 0), 2);
        assert_eq!(map.area_at(0, 479), 3);
        assert_eq!(map.area_at(639, 479), 4);
        for id in 1..=4 {
            let point = map.first_point_for(id).unwrap();
            assert_eq!(map.area_at(point.0, point.1), id);
        }
    }
}
