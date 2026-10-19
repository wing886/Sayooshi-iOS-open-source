//! A lazily loaded pack must give every scene byte for byte as the rebuilt
//! pack does. Uses a local game (`SIGLUS_TEST_PROJECT`, or RewriteHF when
//! present); skipped otherwise.

use siglus_assets::scene_pck::{ScenePck, ScenePckDecodeOptions, find_scene_pck_in_project};

#[test]
fn lazy_scenes_match_the_rebuilt_pack() {
    let project = std::env::var_os("SIGLUS_TEST_PROJECT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Users/xmoe/Documents/RewriteHF".into());
    let Ok(path) = find_scene_pck_in_project(&project) else {
        return;
    };
    let options = ScenePckDecodeOptions::from_project_dir(&project).unwrap();
    let rebuilt = ScenePck::load_and_rebuild(&path, &options).unwrap();
    let lazy = ScenePck::load_lazy(&path, &options).unwrap();
    assert!(lazy.is_lazy());
    assert_eq!(rebuilt.scn_name_map, lazy.scn_name_map);
    assert_eq!(rebuilt.string_codec, lazy.string_codec);
    assert_eq!(rebuilt.inc_cmds.len(), lazy.inc_cmds.len());
    let count = rebuilt.header.scn_data_cnt.max(0) as usize;
    assert!(count > 0);
    for scn_no in 0..count {
        let a = rebuilt.scn_data_slice(scn_no).unwrap();
        let (owner, range) = lazy.scn_data_shared(scn_no).unwrap();
        assert_eq!(&*a, &owner[range.clone()], "scene {scn_no}");
        // A scene in use is shared, not decoded again.
        let (again, _) = lazy.scn_data_shared(scn_no).unwrap();
        assert!(range.is_empty() || std::sync::Arc::ptr_eq(&owner, &again));
    }
}
