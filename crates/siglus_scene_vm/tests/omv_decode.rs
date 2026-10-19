use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

fn project_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../testcase")
        .canonicalize()
        .expect("canonical testcase dir")
}

fn open_theora_from_omv(path: &Path) -> Result<siglus_omv_decoder::TheoraFile> {
    let ogg = siglus_assets::omv::OmvFile::read_embedded_ogg(path)
        .with_context(|| format!("extract ogg from {}", path.display()))?;
    siglus_omv_decoder::TheoraFile::open_from_memory(ogg)
        .with_context(|| format!("open theora from {}", path.display()))
}

#[test]
fn omv_header_and_theora_stream_match_known_sample() -> Result<()> {
    let path = project_dir().join("mov/ny_mv_lucia12aw.omv");
    let omv = siglus_assets::omv::OmvFile::open(&path)?;
    let tf = open_theora_from_omv(&path)?;
    let info = tf.info();

    assert_eq!(omv.header.display_width, 640);
    assert_eq!(omv.header.display_height, 360);
    assert_eq!(omv.header.packet_count_hint, 65);
    assert_eq!(info.width, 640);
    assert_eq!(info.height, 480);
    assert_eq!(info.fmt, siglus_omv_decoder::TH_PF_444);
    assert!((info.fps - 30.0).abs() < 0.01);

    Ok(())
}

#[test]
fn movie_manager_decodes_known_omv_samples_without_losing_frames() -> Result<()> {
    let project_dir = project_dir();
    let mut movie = siglus_scene_vm::movie::MovieManager::new(project_dir.clone());
    let cases = [
        ("mov/ny_mv_lucia12aw.omv", 640u32, 360u32, 65usize),
        ("mov/mn_tt_rpa_sz00.omv", 1280u32, 150u32, 60usize),
        ("mov/ef_ch_sks_mh00.omv", 854u32, 480u32, 210usize),
    ];

    for (rel, width, height, frames) in cases {
        let path = project_dir.join(rel);
        let (asset, _) = movie.ensure_asset(path.to_string_lossy().as_ref())?;
        assert_eq!(asset.info.width, Some(width), "{}", rel);
        assert_eq!(asset.info.height, Some(height), "{}", rel);
        assert_eq!(asset.info.decoded_frames, Some(frames), "{}", rel);
        assert_eq!(asset.frames.len(), 1, "{}", rel);
    }

    Ok(())
}

#[test]
fn alpha_packed_omv_preserves_transparency_range() -> Result<()> {
    let project_dir = project_dir();
    let mut movie = siglus_scene_vm::movie::MovieManager::new(project_dir.clone());
    let path = project_dir.join("mov/ny_mv_lucia12aw.omv");
    let (asset, _) = movie.ensure_asset(path.to_string_lossy().as_ref())?;
    let first = asset.frames.first().context("decoded first frame")?;

    let mut alpha_min = u8::MAX;
    let mut alpha_max = u8::MIN;
    for px in first.rgba.as_chunks::<4>().0 {
        alpha_min = alpha_min.min(px[3]);
        alpha_max = alpha_max.max(px[3]);
    }

    assert_eq!(alpha_min, 0);
    assert_eq!(alpha_max, 255);
    Ok(())
}

#[test]
fn embedded_ogg_starts_with_ogg_magic() -> Result<()> {
    let path = project_dir().join("mov/ny_mv_lucia12aw.omv");
    let ogg = siglus_assets::omv::OmvFile::read_embedded_ogg(&path)?;
    assert_eq!(&ogg[..4], b"OggS");
    assert!(fs::metadata(path)?.len() as usize > ogg.len());
    Ok(())
}

/// Supply the original file externally; do not redistribute game assets.
#[test]
#[ignore = "set SIGLUS_OMV_SEEK_TEST_FILE to Summer Pockets RB mov/__sys_day_filter02.omv"]
fn indexed_seek_discards_back_page_packets_and_matches_sequential_decode() -> Result<()> {
    use siglus_assets::omv::OmvFile;
    use siglus_omv_decoder::TheoraVideoStream;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn fingerprint(frame: &[u8]) -> (usize, u64) {
        let mut hash = DefaultHasher::new();
        frame.hash(&mut hash);
        (frame.len(), hash.finish())
    }

    let path = PathBuf::from(std::env::var("SIGLUS_OMV_SEEK_TEST_FILE")?);
    let omv = OmvFile::open(&path)?;
    let point = omv.seek_point_for_frame(40)?;
    assert_eq!(point.key_frame_packet_no, 32);
    assert!(point.seek_page_no < point.key_frame_page_no);
    assert!(
        omv.pages[point.seek_page_no..point.key_frame_page_no]
            .iter()
            .any(|page| page.packet_count > 0)
    );

    let mut sequential = TheoraVideoStream::open(omv.open_embedded_ogg_reader(&path)?)?;
    let mut expected = Vec::new();
    for _ in 0..66 {
        let frame = sequential.read_video_frame()?.context("sequential frame")?;
        expected.push(fingerprint(&frame));
    }

    // Start with the reported seek, then cover both sides of key frames,
    // backwards seeks, repeated seeks and sequential playback after each seek.
    let mut indexed = TheoraVideoStream::open(omv.open_embedded_ogg_reader(&path)?)?;
    for target in [40, 0, 15, 16, 31, 32, 47, 48, 63, 64, 40, 32, 0] {
        let point = omv.seek_point_for_frame(target)?;
        let frame = indexed
            .seek_to_indexed_frame(
                point.file_offset,
                point.key_page_file_offset,
                point.first_packet_no,
                point.key_frame_packet_no,
                point.target_packet_no,
            )?
            .context("indexed frame")?;
        assert_eq!(fingerprint(&frame), expected[target], "seek frame {target}");
        let next = indexed.read_video_frame()?.context("frame after seek")?;
        assert_eq!(
            fingerprint(&next),
            expected[target + 1],
            "after seek {target}"
        );
    }
    Ok(())
}
