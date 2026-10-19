//! Tone curves (`#TONECURVE_FILENAME`, a `.tcc` in `dat/`): per-channel
//! lookup tables that recolour images, e.g. for evening and night scenes.
//! The file starts with 1000 and the table count; the tables follow from
//! 0xFA8, each a 0x40-byte header and 256 red, green and blue entries.

use std::collections::HashMap;
use std::rc::{Rc, Weak};

use crate::image::Image;

pub type RgbMap = [[u8; 256]; 3];

#[derive(Debug, Clone, Default)]
pub struct ToneCurves {
    tables: Vec<RgbMap>,
    /// Recoloured copies: (image address, table) → (original, copy); the
    /// original is checked so a new image at a freed address is not
    /// mistaken for it.
    cache: HashMap<(usize, usize), (Weak<Image>, Rc<Image>)>,
}

impl ToneCurves {
    pub fn parse(data: &[u8]) -> Self {
        let word = |at: usize| {
            data.get(at..at + 4)
                .map_or(0, |b| i32::from_le_bytes(b.try_into().expect("4 bytes")))
        };
        let mut tables = Vec::new();
        if word(0) == 1000 {
            let count = word(4).max(0) as usize;
            let mut at = 0xFA8 + 0x40;
            for _ in 0..count {
                let Some(bytes) = data.get(at..at + 768) else {
                    break;
                };
                let mut map = [[0u8; 256]; 3];
                for (channel, table) in map.iter_mut().enumerate() {
                    table.copy_from_slice(&bytes[channel * 256..channel * 256 + 256]);
                }
                tables.push(map);
                at += 768 + 0x40;
            }
        }
        Self {
            tables,
            cache: HashMap::new(),
        }
    }

    /// The table a script number names. Scripts count from 100 (LB passes
    /// 100 and 101 for its two times of day); smaller numbers are taken
    /// as table indices.
    pub fn table(&self, number: i32) -> Option<usize> {
        let index = if number >= 100 { number - 100 } else { number };
        usize::try_from(index)
            .ok()
            .filter(|&i| i < self.tables.len())
    }

    /// `image` recoloured with table `table` (cached).
    pub fn apply(&mut self, image: &Rc<Image>, table: usize) -> Rc<Image> {
        let key = (Rc::as_ptr(image) as usize, table);
        if let Some((original, done)) = self.cache.get(&key)
            && original.upgrade().is_some_and(|o| Rc::ptr_eq(&o, image))
        {
            return done.clone();
        }
        let mut toned = (**image).clone();
        self.apply_to(&mut toned.surface, table);
        let toned = Rc::new(toned);
        if self.cache.len() > 64 {
            self.cache.clear();
        }
        self.cache
            .insert(key, (Rc::downgrade(image), toned.clone()));
        toned
    }

    /// Recolours `surface` in place.
    pub fn apply_to(&self, surface: &mut crate::surface::Surface, table: usize) {
        let Some(map) = self.tables.get(table) else {
            return;
        };
        for pixel in surface.rgba.chunks_exact_mut(4) {
            for channel in 0..3 {
                pixel[channel] = map[channel][usize::from(pixel[channel])];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_tone_curves() {
        let Some(root) = std::env::var_os("REALLIVE_TEST_GAME") else {
            return;
        };
        let Ok(data) = game_fs::read(std::path::Path::new(&root).join("dat/tcdata.tcc")) else {
            return;
        };
        let mut curves = ToneCurves::parse(&data);
        assert_eq!(curves.tables.len(), 5);
        let table = curves.table(101).expect("LB's night curve");
        let image = Rc::new(Image::new(2, 1));
        let toned = curves.apply(&image, table);
        // Black stays black in LB's curves; the copy is cached.
        assert_eq!(&toned.surface.rgba[..3], &[0, 35, 13]);
        assert!(Rc::ptr_eq(&toned, &curves.apply(&image, table)));
    }

    #[test]
    fn not_a_tcc() {
        assert_eq!(ToneCurves::parse(b"junk").table(100), None);
    }
}
