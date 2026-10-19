//! Immutable scene layout metadata derived from the currently loaded Scene.pck.
//!
//! The original engine keeps this information in the resident lexer and
//! rebuilds it when `tnm_reload_scene_pck()` switches append directories. The
//! Rust cache follows the same lifetime: callers key it by active append.

use anyhow::Result;
use siglus_assets::scene_pck::ScenePck;
use std::collections::HashMap;

use crate::scene_stream::ScnHeader;

#[derive(Debug)]
pub(crate) struct SceneMetadata {
    pub rows: Vec<(String, usize)>,
    names: HashMap<String, usize>,
}

impl SceneMetadata {
    pub fn from_pack(pck: &ScenePck) -> Result<Self> {
        let count = pck.header.scn_data_cnt.max(0) as usize;
        let mut rows = Vec::with_capacity(count);
        for scene_no in 0..count {
            let name = pck
                .find_scene_name(scene_no)
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| scene_no.to_string());
            let chunk = pck.scn_data_slice(scene_no)?;
            let read_flag_count = if chunk.is_empty() {
                0
            } else {
                ScnHeader::read(&chunk)?.read_flag_cnt.max(0) as usize
            };
            rows.push((name, read_flag_count));
        }
        let names = pck.scn_name_map.clone();
        Ok(Self { rows, names })
    }

    #[cfg(test)]
    pub fn from_rows(rows: Vec<(String, usize)>) -> Self {
        let names = rows
            .iter()
            .enumerate()
            .map(|(scene_no, (name, _))| (name.clone(), scene_no))
            .collect();
        Self { rows, names }
    }

    pub fn find_scene_no(&self, name: &str) -> Option<usize> {
        if let Ok(scene_no) = name.parse::<usize>() {
            return Some(scene_no);
        }
        self.names.get(name).copied().or_else(|| {
            self.names
                .iter()
                .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
                .map(|(_, scene_no)| *scene_no)
                .min()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_matches_resident_lexer_case_semantics() {
        let metadata = SceneMetadata::from_rows(vec![
            ("_rb_titlemenu".to_string(), 4),
            ("story".to_string(), 8),
        ]);
        assert_eq!(metadata.find_scene_no("_RB_titlemenu"), Some(0));
        assert_eq!(metadata.find_scene_no("STORY"), Some(1));
        assert_eq!(metadata.find_scene_no("1"), Some(1));
    }
}
