use siglus_assets::keys::{GAMEEXE_KEY, SCENE_KEY};
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;

const SCENE_PACK_HEADER_SIZE: usize = 23 * 4;
const SCENE_HEADER_SIZE: usize = 33 * 4;
const MAX_SYMBOLIC_STATES: usize = 80_000;
const MAX_SYMBOLIC_SRC: usize = 512;
const MAX_BRUTE_MISSING_KEY_BYTES: usize = 2;
const MAX_ORG_TUPLE_ENUM: usize = 2_000_000;
const MAX_ORG_SPLIT_DOMAIN: usize = 8;
const MAX_ORG_SPLIT_VARIANTS: usize = 32;
const MAX_REASONABLE_DECOMPRESSED: usize = 512 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum SymByte {
    Const(u8),
    KeyXor { key_idx: u8, masked: u8 },
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct CrackState {
    key: [Option<u8>; 16],
    src: usize,
    out: Vec<SymByte>,
}

#[derive(Clone, Copy, Debug)]
struct BruteJob {
    state_idx: usize,
    start: u32,
    end: u32,
}

#[derive(Clone, Debug)]
struct SceneBlob {
    scene_no: usize,
    masked: Vec<u8>,
}

#[derive(Clone, Debug)]
struct ParsedScenePack {
    blobs: Vec<SceneBlob>,
}

/// Return whether either resource header says that the per-executable
/// 16-byte angou element is enabled.
///
/// This only inspects the clear-text resource headers. It does not attempt to
/// recover or validate a key.
pub fn resources_require_exe_key(game: &[u8], scene: &[u8]) -> Result<bool, String> {
    let game_mode = read_u32(game, 4).ok_or("Gameexe.dat header is truncated")?;
    let scene_mode = read_u32(scene, 21 * 4).ok_or("Scene.pck header is truncated")?;
    Ok(game_mode != 0 || scene_mode != 0)
}

/// Cheaply test a configured key against the same structural invariants used
/// by the resource cracker. No full Scene.pck decompression is performed.
///
/// A `false` result means the configured key is not compatible with this
/// normal Siglus resource pair and is a signal to try full recovery. Errors
/// mean the pair is outside the resource-crack format (for example plaintext
/// resources or a truncated file).
pub fn validate_key_quick(game: &[u8], scene: &[u8], key: &[u8; 16]) -> Result<bool, String> {
    let game_masked = parse_gameexe_masked(game)?;
    let scene_pack = parse_scene_pack(scene)?;
    if scene_pack.blobs.is_empty() {
        return Err("Scene.pck contains no non-empty scene blob".to_string());
    }
    if !validate_gameexe_lz_header(key, &game_masked) {
        return Ok(false);
    }
    Ok(scene_pack
        .blobs
        .iter()
        .take(4)
        .all(|blob| validate_scene_prefix(key, &blob.masked)))
}

/// Verdict for a key that is already configured for a resource pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyStatus {
    /// The key demonstrably decrypts at least one of the two resources.
    Accepted,
    /// The scene chunks are stored uncompressed (the easy-link form), so no
    /// compressed-resource structure exists to check the key against. Callers
    /// must keep the configured key and must not run the resource crack.
    Unverifiable,
    /// The pack declares compressed scene chunks, but this key does not
    /// decrypt them.
    Mismatch,
}

/// Number of leading scene chunks a configured key is checked against.
const KEY_CHECK_BLOBS: usize = 4;

/// Whether the loader LZSS-decompresses this pack's scene chunks.
///
/// This mirrors the `original_source_header_size` test that
/// `siglus_assets::scene_pck` uses to pick between decompressing a chunk and
/// keeping its bytes as they are (the easy-link form), so a pack reported here
/// as uncompressed is also decoded without LZSS at runtime.
pub fn scene_pack_is_compressed(scene: &[u8]) -> Result<bool, String> {
    Ok(read_u32(scene, 22 * 4).ok_or("Scene.pck missing original_source_header_size")? != 0)
}

/// Check a configured key against a resource pair without requiring the pack
/// to match the original compiler's layout byte for byte.
///
/// [`validate_key_quick`] tests the crack invariants exactly and consequently
/// rejects packs that the runtime loads happily:
///
/// * A text-repacking tool appends its new string table to a scene object and
///   retargets `str_list_ofs` (and leaves the trailing table end pointing at
///   the pre-append size). The runtime reads both offsets from the header, but
///   the crack relations `str_list_ofs == 132 + str_index_cnt * 8` and
///   `scn_ofs >= str_list_ofs` no longer hold.
/// * Easy-link packs store their scene chunks uncompressed.
///
/// A configured key is accepted as soon as it decrypts one resource, which
/// cannot happen by accident. A mismatch is only reported when the pack
/// declares compressed chunks and still refuses the key.
pub fn check_key(game: &[u8], scene: &[u8], key: &[u8; 16]) -> Result<KeyStatus, String> {
    let game_state = check_gameexe_state(game, key);
    let scene_state = check_scene_pack_state(scene, key);

    if matches!(scene_state, Ok(ResourceState::Decrypted))
        || matches!(game_state, Ok(ResourceState::Decrypted))
    {
        return Ok(KeyStatus::Accepted);
    }
    if matches!(scene_state, Ok(ResourceState::Mismatch)) {
        return Ok(KeyStatus::Mismatch);
    }
    // Anything else leaves the key unproven: either the scene chunks are not
    // compressed (the easy-link form) or a resource could not be inspected.
    match (game_state, scene_state) {
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
        _ => Ok(KeyStatus::Unverifiable),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResourceState {
    Decrypted,
    Unverifiable,
    Mismatch,
}

fn check_gameexe_state(game: &[u8], key: &[u8; 16]) -> Result<ResourceState, String> {
    if game.len() < 16 {
        return Err("Gameexe.dat is too small".to_string());
    }
    let exe_angou_mode = read_u32(game, 4).ok_or("Gameexe.dat header is truncated")?;
    if exe_angou_mode == 0 {
        // Nothing in this file is encrypted with the executable key.
        return Ok(ResourceState::Unverifiable);
    }
    let masked = game[8..]
        .iter()
        .enumerate()
        .map(|(i, b)| *b ^ GAMEEXE_KEY[i & 0xff])
        .collect::<Vec<u8>>();
    if masked.len() < 12 {
        return Ok(ResourceState::Unverifiable);
    }
    if validate_gameexe_lz_header(key, &masked) {
        Ok(ResourceState::Decrypted)
    } else {
        Ok(ResourceState::Mismatch)
    }
}

fn check_scene_pack_state(scene: &[u8], key: &[u8; 16]) -> Result<ResourceState, String> {
    let ranges = scene_pack_blob_ranges(scene)?;
    if ranges.is_empty() {
        return Err("Scene.pck contains no non-empty scene blob".to_string());
    }
    let compressed = scene_pack_is_compressed(scene)?;

    for (_, start, end) in ranges.iter().take(KEY_CHECK_BLOBS) {
        let chunk = &scene[*start..*end];

        if !compressed {
            // Easy-link layout: the chunk is the compiled scene object itself,
            // masked at most with the executable key and the easy angou code.
            let raw = xor_cycle(chunk, key);
            let easy = xor_cycle(&xor_cycle(chunk, &SCENE_KEY), key);
            if [raw, easy]
                .iter()
                .any(|plain| scene_object_at(plain, plain.len()))
            {
                return Ok(ResourceState::Decrypted);
            }
            continue;
        }

        // Normal layout: the chunk is an LZSS container masked with the fixed
        // scene key and then with the executable key.
        let masked = xor_cycle(chunk, &SCENE_KEY);
        let plain = decrypt_masked(&masked, key);
        if read_u32(&plain, 0) != Some(plain.len() as u32) {
            continue;
        }
        let org_len = read_u32(&plain, 4).unwrap_or(0) as usize;
        let max_org = max_lz_output_bound(plain.len()).min(MAX_REASONABLE_DECOMPRESSED);
        if org_len < SCENE_HEADER_SIZE || org_len > max_org {
            continue;
        }
        let header = match decompress_lz_prefix_partial(&plain, plain.len(), SCENE_HEADER_SIZE) {
            Some(header) => header,
            None => continue,
        };
        if scene_object_layout_ok(&header, org_len) {
            return Ok(ResourceState::Decrypted);
        }
    }

    Ok(if compressed {
        ResourceState::Mismatch
    } else {
        ResourceState::Unverifiable
    })
}

/// Whether `data` starts with a scene object of `total_len` bytes.
fn scene_object_at(data: &[u8], total_len: usize) -> bool {
    data.len() >= SCENE_HEADER_SIZE
        && read_u32(data, 0) == Some(SCENE_HEADER_SIZE as u32)
        && scene_object_layout_ok(&data[..SCENE_HEADER_SIZE], total_len)
}

/// Recover the unique 16-byte Siglus EXE encryption key from an encrypted
/// `Gameexe.dat` + `Scene.pck` pair.
///
/// This is the resource-only algorithm from `siglus-static-key-tool`: no PE
/// parsing, executable scanning, or packer analysis is involved. A key is
/// returned only after hard validation against Gameexe and Scene resources.
pub fn recover_key_from_resources(game: &[u8], scene: &[u8]) -> Result<[u8; 16], String> {
    let verbose = 0u8;
    let game_masked = parse_gameexe_masked(game)?;
    let scene_pack = parse_scene_pack(scene)?;
    if scene_pack.blobs.is_empty() {
        return Err("Scene.pck contains no non-empty scene blob".to_string());
    }

    let game_prefix = recover_size_prefix(&game_masked, game_masked.len())?;
    let mut base_key = [None; 16];
    for i in 0..4 {
        base_key[i] = Some(game_prefix[i]);
    }

    let mut scene_prefix_votes = Vec::new();
    for blob in &scene_pack.blobs {
        if blob.masked.len() < 12 {
            continue;
        }
        scene_prefix_votes.push((
            blob.scene_no,
            recover_size_prefix(&blob.masked, blob.masked.len())?,
        ));
    }
    if scene_prefix_votes.is_empty() {
        return Err(
            "Scene.pck has no encrypted scene blob large enough for LZSS header recovery"
                .to_string(),
        );
    }
    for (scene_no, prefix) in &scene_prefix_votes {
        if prefix != &game_prefix {
            return Err(format!(
                "resource key mismatch before cracking: Gameexe gives {:02X?}, scene {} gives {:02X?}",
                game_prefix, scene_no, prefix
            ));
        }
    }

    let org_bootstrap = bootstrap_org_size_keys(&scene_pack, &game_masked, base_key, verbose)?;
    let mut bootstrap_keys = Vec::new();
    for key in org_bootstrap {
        bootstrap_keys.extend(bootstrap_scene_header_keys(&scene_pack, key)?);
    }
    dedup_partial_keys(&mut bootstrap_keys);

    let mut by_size = (0..scene_pack.blobs.len()).collect::<Vec<_>>();
    by_size.sort_by_key(|&i| scene_pack.blobs[i].masked.len());
    let n = by_size.len();
    let mut seed_indices = Vec::<usize>::new();
    for rank in [
        0usize,
        n / 8,
        n / 4,
        n / 2,
        (n * 3) / 4,
        n.saturating_sub(1),
    ] {
        if let Some(&idx) = by_size.get(rank.min(n.saturating_sub(1)))
            && !seed_indices.contains(&idx)
        {
            seed_indices.push(idx);
        }
    }
    for &idx in by_size.iter().take(4) {
        if !seed_indices.contains(&idx) {
            seed_indices.push(idx);
        }
    }
    seed_indices.truncate(8);

    let mut full_candidates = Vec::<[u8; 16]>::new();
    'seeds: for &seed_idx in &seed_indices {
        let blob = &scene_pack.blobs[seed_idx];
        let cross_scene = scene_pack
            .blobs
            .iter()
            .filter(|other| other.scene_no != blob.scene_no)
            .map(|other| other.masked.as_slice())
            .collect::<Vec<_>>();
        for initial_key in bootstrap_keys.iter().copied() {
            let mut found = solve_scene_prefix(
                &blob.masked,
                initial_key,
                &game_masked,
                &cross_scene,
                verbose,
            )?;
            full_candidates.append(&mut found);
            dedup_full_keys(&mut full_candidates);
            if !full_candidates.is_empty() {
                break 'seeds;
            }
        }
    }

    if full_candidates.is_empty() {
        return Err(
            "resource crack did not converge to a full 16-byte key from the Scene.pck header constraints"
                .to_string(),
        );
    }

    let verify_scene_count = scene_pack.blobs.len().min(16);
    full_candidates.retain(|candidate| {
        scene_pack
            .blobs
            .iter()
            .take(verify_scene_count)
            .all(|blob| validate_scene_prefix(candidate, &blob.masked))
    });
    full_candidates.retain(|candidate| validate_gameexe_key(candidate, game));
    full_candidates.retain(|candidate| validate_scene_pack_key(candidate, &scene_pack, verbose));
    dedup_full_keys(&mut full_candidates);

    match full_candidates.as_slice() {
        [] => Err("all resource-crack candidates failed hard Gameexe/Scene validation".to_string()),
        [key] => Ok(*key),
        many => Err(format!(
            "resource crack was not unique: {} hard-valid candidates remain",
            many.len()
        )),
    }
}

fn parse_gameexe_masked(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("Gameexe.dat is too small".to_string());
    }
    let exe_angou_mode = read_u32(data, 4).ok_or("Gameexe.dat header is truncated")?;
    if exe_angou_mode == 0 {
        return Err(
            "Gameexe.dat says exe_angou_mode=0; crack mode requires normal EXE-key encryption"
                .to_string(),
        );
    }
    let payload = &data[8..];
    if payload.len() < 12 {
        return Err("Gameexe.dat encrypted payload is too small".to_string());
    }
    Ok(payload
        .iter()
        .enumerate()
        .map(|(i, b)| *b ^ GAMEEXE_KEY[i & 0xff])
        .collect())
}

/// Clear-text `Scene.pck` header fields and the file ranges of every non-empty
/// scene chunk, as `(scene_no, start, end)`.
///
/// This deliberately ignores the encryption-mode words so that callers can
/// inspect packs that are not in the crackable form.
fn scene_pack_blob_ranges(data: &[u8]) -> Result<Vec<(usize, usize, usize)>, String> {
    if data.len() < SCENE_PACK_HEADER_SIZE {
        return Err("Scene.pck is too small for S_tnm_pack_scn_header".to_string());
    }
    let header_size = read_u32(data, 0).ok_or("Scene.pck header is truncated")? as usize;
    if header_size != SCENE_PACK_HEADER_SIZE {
        return Err(format!(
            "Scene.pck header_size mismatch: got 0x{header_size:X}, expected 0x{SCENE_PACK_HEADER_SIZE:X}"
        ));
    }

    let index_ofs =
        read_u32(data, 17 * 4).ok_or("Scene.pck missing scn_data_index_list_ofs")? as usize;
    let index_cnt = read_u32(data, 18 * 4).ok_or("Scene.pck missing scn_data_index_cnt")? as usize;
    let data_ofs = read_u32(data, 19 * 4).ok_or("Scene.pck missing scn_data_list_ofs")? as usize;
    let data_cnt = read_u32(data, 20 * 4).ok_or("Scene.pck missing scn_data_cnt")? as usize;
    if index_cnt != data_cnt {
        return Err(format!(
            "Scene.pck index/data count mismatch: index_cnt={index_cnt}, data_cnt={data_cnt}"
        ));
    }
    let index_bytes = index_cnt
        .checked_mul(8)
        .and_then(|n| index_ofs.checked_add(n))
        .ok_or("Scene.pck index table overflows")?;
    if index_bytes > data.len() || data_ofs > data.len() {
        return Err("Scene.pck index/data offsets are out of range".to_string());
    }

    let mut ranges = Vec::new();
    for scene_no in 0..index_cnt {
        let base = index_ofs + scene_no * 8;
        let rel = read_u32(data, base).ok_or("Scene.pck index offset truncated")? as usize;
        let size = read_u32(data, base + 4).ok_or("Scene.pck index size truncated")? as usize;
        if size == 0 {
            continue;
        }
        let start = data_ofs
            .checked_add(rel)
            .ok_or("Scene.pck scene offset overflows")?;
        let end = start
            .checked_add(size)
            .ok_or("Scene.pck scene size overflows")?;
        if start < data_ofs || end > data.len() || size < 12 {
            return Err(format!(
                "Scene.pck scene {scene_no} has an invalid encrypted blob range"
            ));
        }
        ranges.push((scene_no, start, end));
    }
    Ok(ranges)
}

fn parse_scene_pack(data: &[u8]) -> Result<ParsedScenePack, String> {
    let _original_source_header_size =
        read_u32(data, 22 * 4).ok_or("Scene.pck missing original_source_header_size")? as usize;
    let exe_angou_mode =
        read_u32(data, 21 * 4).ok_or("Scene.pck missing scn_data_exe_angou_mod")?;
    if exe_angou_mode == 0 {
        return Err("Scene.pck says scn_data_exe_angou_mod=0; crack mode requires normal EXE-key encryption".to_string());
    }
    // Do not use original_source_header_size to infer LZSS mode.  In the
    // original compiler that field is zero whenever ORIGINAL_SOURCE_LINK is
    // disabled, even though the normal scene payloads are still LZSS-packed.
    // The crack path proves LZSS independently from each blob's encrypted
    // [arc_size, org_size] header and from successful decompression.
    let mut blobs = Vec::new();
    for (scene_no, start, end) in scene_pack_blob_ranges(data)? {
        let masked = data[start..end]
            .iter()
            .enumerate()
            .map(|(i, b)| *b ^ SCENE_KEY[i & 0xff])
            .collect::<Vec<_>>();
        blobs.push(SceneBlob { scene_no, masked });
    }
    Ok(ParsedScenePack { blobs })
}

fn recover_size_prefix(masked: &[u8], compressed_size: usize) -> Result<[u8; 4], String> {
    if masked.len() < 4 || compressed_size > u32::MAX as usize {
        return Err(
            "encrypted LZSS blob cannot provide the compressed-size key prefix".to_string(),
        );
    }
    let size = (compressed_size as u32).to_le_bytes();
    Ok([
        masked[0] ^ size[0],
        masked[1] ^ size[1],
        masked[2] ^ size[2],
        masked[3] ^ size[3],
    ])
}

fn max_lz_output_bound(comp_len: usize) -> usize {
    // Siglus LZSS stores an 8-byte [arc_size, org_size] header, then one flag
    // byte per up-to-eight tokens. A literal costs one source byte and emits
    // one byte; a back-reference costs two source bytes and emits at most 17.
    // The first token of a compressor-produced stream must be a literal
    // because there is no history yet.
    if comp_len < 10 {
        return 0;
    }

    let mut rem = comp_len - 8;
    let mut out = 0usize;

    rem -= 1; // first flag
    if rem == 0 {
        return 0;
    }
    rem -= 1; // mandatory first literal
    out = 1;

    let mut slots = 7usize;
    while slots > 0 && rem > 0 {
        if rem >= 2 {
            rem -= 2;
            out = out.saturating_add(17);
        } else {
            rem -= 1;
            out = out.saturating_add(1);
        }
        slots -= 1;
    }

    while rem >= 2 {
        rem -= 1; // flag
        let mut slots = 8usize;
        while slots > 0 && rem > 0 {
            if rem >= 2 {
                rem -= 2;
                out = out.saturating_add(17);
            } else {
                rem -= 1;
                out = out.saturating_add(1);
            }
            slots -= 1;
        }
    }

    out
}

fn decoded_org_size_from_key4(masked: &[u8], key4: &[u8; 4]) -> Option<usize> {
    if masked.len() < 8 {
        return None;
    }
    let mut org = [0u8; 4];
    for i in 0..4 {
        org[i] = masked[4 + i] ^ key4[i];
    }
    Some(u32::from_le_bytes(org) as usize)
}

fn partial_key4(key: &[Option<u8>; 16]) -> Option<[u8; 4]> {
    Some([key[4]?, key[5]?, key[6]?, key[7]?])
}

fn org_key_tuple(masked: &[u8], org_size: usize) -> Option<[u8; 4]> {
    if masked.len() < 8 || org_size > u32::MAX as usize {
        return None;
    }
    let org = (org_size as u32).to_le_bytes();
    Some([
        masked[4] ^ org[0],
        masked[5] ^ org[1],
        masked[6] ^ org[2],
        masked[7] ^ org[3],
    ])
}

fn scene_org_tuple_plausible(masked: &[u8], key4: &[u8; 4]) -> bool {
    let Some(org) = decoded_org_size_from_key4(masked, key4) else {
        return false;
    };
    let max_org = max_lz_output_bound(masked.len()).min(MAX_REASONABLE_DECOMPRESSED);
    org >= SCENE_HEADER_SIZE && org <= max_org
}

fn game_org_tuple_plausible(masked: &[u8], key4: &[u8; 4]) -> bool {
    let Some(org) = decoded_org_size_from_key4(masked, key4) else {
        return false;
    };
    let max_org = max_lz_output_bound(masked.len()).min(MAX_REASONABLE_DECOMPRESSED);
    org >= 2 && org <= max_org && (org & 1) == 0
}

fn bootstrap_org_size_keys(
    pack: &ParsedScenePack,
    game_masked: &[u8],
    base: [Option<u8>; 16],
    verbose: u8,
) -> Result<Vec<[Option<u8>; 16]>, String> {
    let seed = pack
        .blobs
        .iter()
        .min_by_key(|blob| max_lz_output_bound(blob.masked.len()))
        .ok_or("Scene.pck contains no scene blob for org-size key recovery")?;
    let max_org = max_lz_output_bound(seed.masked.len()).min(MAX_REASONABLE_DECOMPRESSED);
    if max_org < SCENE_HEADER_SIZE {
        return Err(format!(
            "Scene.pck scene {} is too small to encode a 132-byte scene header",
            seed.scene_no
        ));
    }

    let org_count = max_org - SCENE_HEADER_SIZE + 1;
    if org_count > MAX_ORG_TUPLE_ENUM {
        if verbose > 0 {
            eprintln!(
                "org bootstrap   : skipped tuple enumeration; shortest scene={} allows {} org sizes",
                seed.scene_no, org_count
            );
        }
        return Ok(vec![base]);
    }

    let mut tuples = Vec::<[u8; 4]>::with_capacity(org_count);
    for org in SCENE_HEADER_SIZE..=max_org {
        if let Some(tuple) = org_key_tuple(&seed.masked, org) {
            tuples.push(tuple);
        }
    }

    // Every scene and Gameexe.dat uses the same K4..K7. A tuple is possible
    // only if it decodes each resource's org_size into the exact size range
    // that its compressed length can represent. Gameexe is UTF-16LE, so its
    // decompressed byte length must additionally be even.
    tuples.retain(|tuple| {
        pack.blobs
            .iter()
            .all(|blob| scene_org_tuple_plausible(&blob.masked, tuple))
            && game_org_tuple_plausible(game_masked, tuple)
    });
    tuples.sort_unstable();
    tuples.dedup();

    if tuples.is_empty() {
        return Err(
            "Gameexe.dat/Scene.pck org-size constraints reject every K4..K7 tuple".to_string(),
        );
    }

    let mut domains = (0..4).map(|_| HashSet::<u8>::new()).collect::<Vec<_>>();
    for tuple in &tuples {
        for i in 0..4 {
            domains[i].insert(tuple[i]);
        }
    }

    if verbose > 0 {
        eprintln!(
            "org bootstrap   : scene={} max_org={} tuple_candidates={}",
            seed.scene_no,
            max_org,
            tuples.len()
        );
        eprintln!(
            "org domains     : K4={} K5={} K6={} K7={}",
            domains[0].len(),
            domains[1].len(),
            domains[2].len(),
            domains[3].len()
        );
    }

    // Keep large domains symbolic, but materialize small domains. Projection
    // preserves correlations between the selected K4..K7 bytes.
    let split = (0..4)
        .filter(|&i| domains[i].len() <= MAX_ORG_SPLIT_DOMAIN)
        .collect::<Vec<_>>();
    let mut variants = Vec::<[Option<u8>; 16]>::new();
    let mut seen = HashSet::<[Option<u8>; 16]>::new();
    for tuple in &tuples {
        let mut key = base;
        let mut ok = true;
        for &i in &split {
            ok &= assign_key(&mut key, 4 + i, tuple[i]);
        }
        if ok && seen.insert(key) {
            variants.push(key);
            if variants.len() > MAX_ORG_SPLIT_VARIANTS {
                break;
            }
        }
    }

    if variants.len() > MAX_ORG_SPLIT_VARIANTS || variants.is_empty() {
        let mut key = base;
        for i in 0..4 {
            if domains[i].len() == 1 {
                let value = *domains[i].iter().next().unwrap();
                if !assign_key(&mut key, 4 + i, value) {
                    return Err(format!("org-size bootstrap disagrees on K{}", 4 + i));
                }
            }
        }
        variants.clear();
        variants.push(key);
    }

    if verbose > 0 {
        eprintln!("org key split  : {} partial-key variant(s)", variants.len());
    }
    Ok(variants)
}

fn bootstrap_scene_header_keys(
    pack: &ParsedScenePack,
    mut base: [Option<u8>; 16],
) -> Result<Vec<[Option<u8>; 16]>, String> {
    let mut k11_allowed = [true; 256];
    let mut saw = false;

    for blob in &pack.blobs {
        if blob.masked.len() < 13 {
            continue;
        }
        saw = true;

        // S_tnm_scn_header starts with 84 00 00 00.  At output position 0
        // there is no history, so token 0 is literal 0x84.  Token 1 is also
        // necessarily literal 0x00.  At position 2 the compiler has a zero
        // run and emits a distance-1 back-reference.  For normal PE32-sized
        // scene data the initial zero run is 2..5 bytes, so the raw token is
        // 0x0010..0x0013.
        let exact = [
            (9usize, 9usize, 0x84u8),
            (10usize, 10usize, 0x00u8),
            (12usize, 12usize, 0x00u8),
        ];
        for (src, key_idx, plain) in exact {
            let value = blob.masked[src] ^ plain;
            if !assign_key(&mut base, key_idx, value) {
                return Err(format!(
                    "Scene.pck scene {} disagrees on directly recoverable key byte K{}",
                    blob.scene_no, key_idx
                ));
            }
        }

        let mut local = [false; 256];
        for raw_lo in 0x10u8..=0x13u8 {
            local[(blob.masked[11] ^ raw_lo) as usize] = true;
        }
        for i in 0..256 {
            k11_allowed[i] &= local[i];
        }
    }

    if !saw {
        return Err(
            "Scene.pck has no scene blob long enough for deterministic header bootstrap"
                .to_string(),
        );
    }

    let k11 = (0..256)
        .filter(|&v| k11_allowed[v])
        .map(|v| v as u8)
        .collect::<Vec<_>>();
    if k11.is_empty() {
        return Err(
            "Scene.pck scenes disagree on the mandatory initial distance-1 LZSS back-reference"
                .to_string(),
        );
    }

    let mut variants = Vec::new();
    for value in k11 {
        let mut key = base;
        if assign_key(&mut key, 11, value) {
            variants.push(key);
        }
    }
    Ok(variants)
}

fn print_known_key(keys: &[[Option<u8>; 16]]) {
    let mut parts = Vec::with_capacity(16);
    for i in 0..16 {
        let first = keys.first().and_then(|k| k[i]);
        let same = first.is_some() && keys.iter().all(|k| k[i] == first);
        if same {
            parts.push(format!("{:02X}", first.unwrap()));
        } else {
            parts.push("??".to_string());
        }
    }
    println!("known key      : {}", parts.join(" "));
}

fn solve_scene_prefix(
    masked: &[u8],
    initial_key: [Option<u8>; 16],
    game_masked: &[u8],
    cross_scene: &[&[u8]],
    verbose: u8,
) -> Result<Vec<[u8; 16]>, String> {
    if masked.len() < 24 {
        return Ok(Vec::new());
    }

    let initial = CrackState {
        key: initial_key,
        src: 8,
        out: Vec::new(),
    };

    // The normal Siglus compiler emits a very constrained beginning for every
    // scene stream.  Expand only the first two flag groups first.  They cover
    // enough of S_tnm_scn_header to recover almost all key bytes without
    // symbolically decoding the complete 132-byte header.
    let mut first = expand_flag_group(masked, initial)?;
    first.retain(|s| {
        partial_scene_header_plausible(&s.key, masked)
            && partial_game_header_plausible(&s.key, game_masked)
    });
    dedup_states(&mut first);
    prune_states(&mut first);
    if verbose >= 2 {
        eprintln!("symbolic group : 1, states={}", first.len());
    }

    let mut second = expand_states_parallel(masked, &first, game_masked)?;
    dedup_states(&mut second);
    prune_states(&mut second);
    if verbose >= 2 {
        eprintln!("symbolic group : 2, states={}", second.len());
    }

    let mut full = complete_near_keys(masked, game_masked, cross_scene, &second, verbose);
    dedup_full_keys(&mut full);
    if !full.is_empty() {
        return Ok(full);
    }

    // Unusual headers can make the second flag group end before all useful
    // key positions have reappeared.  A bounded third group is a fallback,
    // not an open-ended symbolic decompressor.
    let mut third_inputs = second
        .into_iter()
        .filter(|s| missing_key_count(&s.key) <= 4)
        .collect::<Vec<_>>();
    prune_states(&mut third_inputs);
    let mut third = expand_states_parallel(masked, &third_inputs, game_masked)?;
    dedup_states(&mut third);
    prune_states(&mut third);
    if verbose >= 2 {
        eprintln!("symbolic group : 3, states={}", third.len());
    }
    full = complete_near_keys(masked, game_masked, cross_scene, &third, verbose);
    dedup_full_keys(&mut full);
    Ok(full)
}

fn worker_count() -> usize {
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .max(1)
}

fn expand_states_parallel(
    masked: &[u8],
    states: &[CrackState],
    game_masked: &[u8],
) -> Result<Vec<CrackState>, String> {
    if states.is_empty() {
        return Ok(Vec::new());
    }

    let workers = worker_count().min(states.len());
    if workers <= 1 {
        let mut out = Vec::<CrackState>::new();
        for st in states {
            for next in expand_flag_group(masked, st.clone())? {
                if partial_game_header_plausible(&next.key, game_masked)
                    && partial_scene_header_plausible(&next.key, masked)
                {
                    out.push(next);
                }
            }
            if out.len() > MAX_SYMBOLIC_STATES * 2 {
                dedup_states(&mut out);
                prune_states(&mut out);
            }
        }
        return Ok(out);
    }

    const RESULT_BATCH: usize = 1024;
    let next_state = AtomicUsize::new(0);
    thread::scope(|scope| -> Result<Vec<CrackState>, String> {
        // A bounded channel keeps parallel symbolic expansion from multiplying
        // the 80k-state memory ceiling by the number of CPU cores.
        let (tx, rx) = mpsc::sync_channel::<Result<Vec<CrackState>, String>>(workers * 2);
        for _ in 0..workers {
            let tx = tx.clone();
            let next_state = &next_state;
            scope.spawn(move || {
                let mut batch = Vec::<CrackState>::with_capacity(RESULT_BATCH);
                loop {
                    let idx = next_state.fetch_add(1, Ordering::Relaxed);
                    if idx >= states.len() {
                        break;
                    }
                    let expanded = match expand_flag_group(masked, states[idx].clone()) {
                        Ok(v) => v,
                        Err(e) => {
                            let _ = tx.send(Err(e));
                            return;
                        }
                    };
                    for next in expanded {
                        if partial_game_header_plausible(&next.key, game_masked)
                            && partial_scene_header_plausible(&next.key, masked)
                        {
                            batch.push(next);
                            if batch.len() >= RESULT_BATCH {
                                if tx.send(Ok(std::mem::take(&mut batch))).is_err() {
                                    return;
                                }
                                batch = Vec::with_capacity(RESULT_BATCH);
                            }
                        }
                    }
                }
                if !batch.is_empty() {
                    let _ = tx.send(Ok(batch));
                }
            });
        }
        drop(tx);

        let mut merged = Vec::<CrackState>::new();
        for message in rx {
            let mut batch = message?;
            merged.append(&mut batch);
            if merged.len() > MAX_SYMBOLIC_STATES * 2 {
                dedup_states(&mut merged);
                prune_states(&mut merged);
            }
        }
        Ok(merged)
    })
}

fn prune_states(states: &mut Vec<CrackState>) {
    if states.len() <= MAX_SYMBOLIC_STATES {
        return;
    }
    states.sort_by_key(|s| {
        let assigned = s.key.iter().filter(|v| v.is_some()).count();
        (
            std::cmp::Reverse(assigned),
            std::cmp::Reverse(s.out.len()),
            s.src,
        )
    });
    states.truncate(MAX_SYMBOLIC_STATES);
}

fn missing_key_count(key: &[Option<u8>; 16]) -> usize {
    key.iter().filter(|v| v.is_none()).count()
}

fn partial_game_header_plausible(key: &[Option<u8>; 16], game_masked: &[u8]) -> bool {
    if game_masked.len() < 8 {
        return false;
    }
    let mut org = [0u8; 4];
    for i in 0..4 {
        let Some(k) = key[4 + i] else {
            return true;
        };
        org[i] = game_masked[4 + i] ^ k;
    }
    let org = u32::from_le_bytes(org) as usize;
    let max_org = max_lz_output_bound(game_masked.len()).min(MAX_REASONABLE_DECOMPRESSED);
    org >= 2 && org <= max_org && (org & 1) == 0
}

fn partial_scene_header_plausible(key: &[Option<u8>; 16], scene_masked: &[u8]) -> bool {
    let Some(key4) = partial_key4(key) else {
        return true;
    };
    scene_org_tuple_plausible(scene_masked, &key4)
}

fn complete_near_keys(
    seed_masked: &[u8],
    game_masked: &[u8],
    cross_scene: &[&[u8]],
    states: &[CrackState],
    verbose: u8,
) -> Vec<[u8; 16]> {
    const BRUTE_CHUNK: u32 = 2048;

    let mut jobs = Vec::<BruteJob>::new();
    for (state_idx, st) in states.iter().enumerate() {
        let missing = missing_key_count(&st.key);
        if missing > MAX_BRUTE_MISSING_KEY_BYTES {
            continue;
        }
        let combinations = match missing {
            0 => 1u32,
            1 => 256u32,
            2 => 65_536u32,
            _ => unreachable!(),
        };
        let mut start = 0u32;
        while start < combinations {
            let end = (start + BRUTE_CHUNK).min(combinations);
            jobs.push(BruteJob {
                state_idx,
                start,
                end,
            });
            start = end;
        }
    }

    if jobs.is_empty() {
        return Vec::new();
    }

    let workers = worker_count().min(jobs.len());
    if verbose >= 2 {
        eprintln!(
            "near-key work  : {} chunk(s), workers={}",
            jobs.len(),
            workers
        );
    }

    let mut merged = Vec::<[u8; 16]>::new();
    if workers <= 1 {
        for job in &jobs {
            brute_job(
                *job,
                states,
                seed_masked,
                game_masked,
                cross_scene,
                &mut merged,
            );
            if merged.len() > 64 {
                merged.clear();
                break;
            }
        }
    } else {
        let next_job = AtomicUsize::new(0);
        thread::scope(|scope| {
            let mut handles = Vec::new();
            let jobs_ref = &jobs;
            for _ in 0..workers {
                let next_job = &next_job;
                handles.push(scope.spawn(move || {
                    let mut local = Vec::<[u8; 16]>::new();
                    loop {
                        let idx = next_job.fetch_add(1, Ordering::Relaxed);
                        if idx >= jobs_ref.len() {
                            break;
                        }
                        brute_job(
                            jobs_ref[idx],
                            states,
                            seed_masked,
                            game_masked,
                            cross_scene,
                            &mut local,
                        );
                        if local.len() > 64 {
                            // This seed/state family is too weak.  Keep the
                            // existing bounded-candidate behavior instead of
                            // letting parallel brute force explode memory.
                            break;
                        }
                    }
                    local
                }));
            }
            for handle in handles {
                let mut local = handle.join().expect("resource-crack worker panicked");
                merged.append(&mut local);
                if merged.len() > 64 {
                    merged.clear();
                    break;
                }
            }
        });
    }

    dedup_full_keys(&mut merged);
    if merged.len() > 64 {
        merged.clear();
    }
    if verbose >= 2 {
        eprintln!(
            "near-key brute : {} prefix-valid candidate(s)",
            merged.len()
        );
    }
    merged
}

fn brute_job(
    job: BruteJob,
    states: &[CrackState],
    seed_masked: &[u8],
    game_masked: &[u8],
    cross_scene: &[&[u8]],
    out: &mut Vec<[u8; 16]>,
) {
    let st = &states[job.state_idx];
    let missing = st
        .key
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.is_none().then_some(i))
        .collect::<Vec<_>>();

    for code in job.start..job.end {
        let mut partial = st.key;
        match missing.as_slice() {
            [] => {}
            [a] => {
                partial[*a] = Some(code as u8);
            }
            [a, b] => {
                partial[*a] = Some((code >> 8) as u8);
                partial[*b] = Some(code as u8);
            }
            _ => return,
        }
        let Some(key) = full_key(&partial) else {
            continue;
        };
        try_completed_key(&key, seed_masked, game_masked, cross_scene, out);
        if out.len() > 64 {
            return;
        }
    }
}

fn try_completed_key(
    key: &[u8; 16],
    seed_masked: &[u8],
    game_masked: &[u8],
    cross_scene: &[&[u8]],
    out: &mut Vec<[u8; 16]>,
) {
    if !validate_scene_prefix(key, seed_masked) || !validate_gameexe_lz_header(key, game_masked) {
        return;
    }
    if !cross_scene
        .iter()
        .take(3)
        .all(|scene| validate_scene_prefix(key, scene))
    {
        return;
    }
    out.push(*key);
}

fn validate_gameexe_lz_header(key: &[u8; 16], masked: &[u8]) -> bool {
    if masked.len() < 8 {
        return false;
    }
    let mut h = [0u8; 8];
    for i in 0..8 {
        h[i] = masked[i] ^ key[i & 0x0f];
    }
    let comp = u32::from_le_bytes(h[0..4].try_into().unwrap()) as usize;
    let org = u32::from_le_bytes(h[4..8].try_into().unwrap()) as usize;
    let max_org = max_lz_output_bound(masked.len()).min(MAX_REASONABLE_DECOMPRESSED);
    comp == masked.len() && org >= 2 && org <= max_org && (org & 1) == 0
}

fn expand_flag_group(masked: &[u8], state: CrackState) -> Result<Vec<CrackState>, String> {
    if state.src >= masked.len() {
        return Ok(Vec::new());
    }
    let flag_pos = state.src;
    let parallel_first_group = flag_pos == 8 && state.out.is_empty();
    let flag_key_idx = flag_pos & 0x0f;
    let mut flag_states = Vec::<(CrackState, u8)>::new();

    if let Some(k) = state.key[flag_key_idx] {
        let flag = masked[flag_pos] ^ k;
        if flag_pos == 8 && state.out.is_empty() && (flag & 0x07) != 0x03 {
            return Ok(Vec::new());
        }
        let mut st = state;
        st.src += 1;
        flag_states.push((st, flag));
    } else if flag_pos == 8 && state.out.is_empty() {
        // Source-exact first three tokens for S_tnm_scn_header:
        // 0x84 literal, 0x00 literal, then a back-reference for the repeated zeros.
        for flag in 0u16..=255 {
            let flag = flag as u8;
            if (flag & 0x07) != 0x03 {
                continue;
            }
            let mut st = state.clone();
            if assign_key(&mut st.key, flag_key_idx, masked[flag_pos] ^ flag) {
                st.src += 1;
                flag_states.push((st, flag));
            }
        }
    } else {
        for flag in 0u16..=255 {
            let flag = flag as u8;
            let mut st = state.clone();
            if assign_key(&mut st.key, flag_key_idx, masked[flag_pos] ^ flag) {
                st.src += 1;
                flag_states.push((st, flag));
            }
        }
    }

    let mut active = flag_states;
    for bit in 0..8 {
        let mut next = expand_flag_bit(masked, active, bit, parallel_first_group);
        dedup_flag_states(&mut next);
        if next.len() > MAX_SYMBOLIC_STATES {
            next.sort_by_key(|(s, _)| {
                let assigned = s.key.iter().filter(|v| v.is_some()).count();
                (
                    std::cmp::Reverse(assigned),
                    std::cmp::Reverse(s.out.len()),
                    s.src,
                )
            });
            next.truncate(MAX_SYMBOLIC_STATES);
        }
        active = next;
        if active.is_empty() {
            break;
        }
    }

    Ok(active.into_iter().map(|(s, _)| s).collect())
}

fn expand_flag_bit(
    masked: &[u8],
    active: Vec<(CrackState, u8)>,
    bit: u32,
    parallel: bool,
) -> Vec<(CrackState, u8)> {
    fn expand_one(
        masked: &[u8],
        st: CrackState,
        flag: u8,
        bit: u32,
        out: &mut Vec<(CrackState, u8)>,
    ) {
        if st.out.len() >= SCENE_HEADER_SIZE {
            out.push((st, flag));
            return;
        }
        if ((flag >> bit) & 1) != 0 {
            if let Some(st2) = expand_literal(masked, st) {
                out.push((st2, flag));
            }
        } else {
            for st2 in expand_backref(masked, st) {
                out.push((st2, flag));
            }
        }
    }

    let workers = if parallel {
        worker_count().min(active.len())
    } else {
        1
    };
    if workers <= 1 {
        let mut next = Vec::<(CrackState, u8)>::new();
        for (st, flag) in active {
            expand_one(masked, st, flag, bit, &mut next);
        }
        return next;
    }

    let mut buckets = (0..workers)
        .map(|_| Vec::<(CrackState, u8)>::new())
        .collect::<Vec<_>>();
    for (i, item) in active.into_iter().enumerate() {
        buckets[i % workers].push(item);
    }

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for bucket in buckets {
            handles.push(scope.spawn(move || {
                let mut local = Vec::<(CrackState, u8)>::new();
                for (st, flag) in bucket {
                    expand_one(masked, st, flag, bit, &mut local);
                }
                local
            }));
        }
        let mut next = Vec::<(CrackState, u8)>::new();
        for handle in handles {
            let mut local = handle.join().expect("resource-crack flag worker panicked");
            next.append(&mut local);
        }
        next
    })
}

fn expand_literal(masked: &[u8], mut st: CrackState) -> Option<CrackState> {
    if st.src >= masked.len() || st.src >= MAX_SYMBOLIC_SRC {
        return None;
    }
    let idx = st.src & 0x0f;
    let sym = match st.key[idx] {
        Some(k) => SymByte::Const(masked[st.src] ^ k),
        None => SymByte::KeyXor {
            key_idx: idx as u8,
            masked: masked[st.src],
        },
    };
    st.src += 1;
    if !append_symbolic(&mut st, sym) {
        return None;
    }
    Some(st)
}

fn expand_backref(masked: &[u8], st: CrackState) -> Vec<CrackState> {
    if st.src + 1 >= masked.len() || st.src + 1 >= MAX_SYMBOLIC_SRC || st.out.is_empty() {
        return Vec::new();
    }

    let p0 = st.src;
    let p1 = st.src + 1;
    let k0 = p0 & 0x0f;
    let k1 = p1 & 0x0f;
    let b0_known = st.key[k0].map(|k| masked[p0] ^ k);
    let b1_known = st.key[k1].map(|k| masked[p1] ^ k);

    if let (Some(lo), Some(hi)) = (b0_known, b1_known) {
        let raw = u16::from_le_bytes([lo, hi]);
        return apply_backref_raw(st, raw).into_iter().collect();
    }

    let max_dist = st.out.len().min(4095);
    let mut out = Vec::new();
    for dist in 1..=max_dist {
        for len in 2..=17usize {
            let raw = (((dist as u16) << 4) | ((len - 2) as u16)).to_le_bytes();
            if let Some(known) = b0_known
                && known != raw[0]
            {
                continue;
            }
            if let Some(known) = b1_known
                && known != raw[1]
            {
                continue;
            }
            let mut candidate = st.clone();
            if !assign_key(&mut candidate.key, k0, masked[p0] ^ raw[0])
                || !assign_key(&mut candidate.key, k1, masked[p1] ^ raw[1])
            {
                continue;
            }
            if let Some(done) = apply_backref_raw(candidate, u16::from_le_bytes(raw)) {
                out.push(done);
            }
        }
    }
    out
}

fn apply_backref_raw(mut st: CrackState, raw: u16) -> Option<CrackState> {
    let dist = (raw >> 4) as usize;
    let len = ((raw & 0x0f) as usize) + 2;
    if dist == 0 || dist > st.out.len() {
        return None;
    }
    st.src += 2;
    for _ in 0..len {
        if st.out.len() >= SCENE_HEADER_SIZE {
            break;
        }
        let src_idx = st.out.len().checked_sub(dist)?;
        let sym = *st.out.get(src_idx)?;
        if !append_symbolic(&mut st, sym) {
            return None;
        }
    }
    Some(st)
}

fn append_symbolic(st: &mut CrackState, sym: SymByte) -> bool {
    let index = st.out.len();
    st.out.push(sym);
    if let Some(expected) = exact_scene_header_byte(index)
        && !constrain_symbol(&mut st.key, sym, expected)
    {
        return false;
    }
    check_all_exact(st) && check_partial_scene_header(st)
}

fn exact_scene_header_byte(index: usize) -> Option<u8> {
    match index {
        0 => Some(0x84),
        1..=3 => Some(0x00),
        12 => Some(0x84),
        13..=15 => Some(0x00),
        _ => None,
    }
}

fn constrain_symbol(key: &mut [Option<u8>; 16], sym: SymByte, expected: u8) -> bool {
    match sym {
        SymByte::Const(v) => v == expected,
        SymByte::KeyXor { key_idx, masked } => assign_key(key, key_idx as usize, masked ^ expected),
    }
}

fn assign_key(key: &mut [Option<u8>; 16], idx: usize, value: u8) -> bool {
    match key[idx] {
        Some(old) => old == value,
        None => {
            key[idx] = Some(value);
            true
        }
    }
}

fn check_all_exact(st: &CrackState) -> bool {
    for (idx, sym) in st.out.iter().copied().enumerate() {
        if let Some(expected) = exact_scene_header_byte(idx) {
            match resolve_sym(sym, &st.key) {
                Some(v) if v == expected => {}
                Some(_) => return false,
                None => {}
            }
        }
    }
    true
}

fn resolve_sym(sym: SymByte, key: &[Option<u8>; 16]) -> Option<u8> {
    match sym {
        SymByte::Const(v) => Some(v),
        SymByte::KeyXor { key_idx, masked } => key[key_idx as usize].map(|k| masked ^ k),
    }
}

fn sym_dword(st: &CrackState, dword_idx: usize) -> Option<u32> {
    let base = dword_idx.checked_mul(4)?;
    if base + 4 > st.out.len() {
        return None;
    }
    let mut b = [0u8; 4];
    for i in 0..4 {
        b[i] = resolve_sym(st.out[base + i], &st.key)?;
    }
    Some(u32::from_le_bytes(b))
}

fn check_partial_scene_header(st: &CrackState) -> bool {
    let d = |i| sym_dword(st, i);

    if let Some(v) = d(0)
        && v != SCENE_HEADER_SIZE as u32
    {
        return false;
    }
    if let Some(v) = d(3)
        && v != SCENE_HEADER_SIZE as u32
    {
        return false;
    }
    if let (Some(c4), Some(c6)) = (d(4), d(6))
        && c4 != c6
    {
        return false;
    }
    if let (Some(c14), Some(c16)) = (d(14), d(16))
        && c14 != c16
    {
        return false;
    }
    if let (Some(c14), Some(c18)) = (d(14), d(18))
        && c14 != c18
    {
        return false;
    }
    if let (Some(c20), Some(c22)) = (d(20), d(22))
        && c20 != c22
    {
        return false;
    }
    if let (Some(c20), Some(c24)) = (d(20), d(24))
        && c20 != c24
    {
        return false;
    }
    if let (Some(c26), Some(c28)) = (d(26), d(28))
        && c26 != c28
    {
        return false;
    }

    if let (Some(cnt), Some(ofs)) = (d(4), d(5))
        && (cnt > 16_000_000 || ofs != 132u32.saturating_add(cnt.saturating_mul(8)))
    {
        return false;
    }
    if let (Some(scn_ofs), Some(str_list_ofs)) = (d(1), d(5))
        && (scn_ofs < str_list_ofs || ((scn_ofs - str_list_ofs) & 1) != 0)
    {
        return false;
    }
    if let (Some(scn_ofs), Some(scn_size), Some(label_ofs)) = (d(1), d(2), d(7))
        && (scn_size == 0 || label_ofs != scn_ofs.saturating_add(scn_size))
    {
        return false;
    }
    if !check_offset_relation(d(7), d(8), d(9), 4) {
        return false;
    }
    if !check_offset_relation(d(9), d(10), d(11), 4) {
        return false;
    }
    if !check_offset_relation(d(11), d(12), d(13), 8) {
        return false;
    }
    if !check_offset_relation(d(13), d(14), d(15), 8) {
        return false;
    }
    if !check_offset_relation(d(15), d(16), d(17), 8) {
        return false;
    }
    if let (Some(a), Some(b)) = (d(17), d(19))
        && (b < a || ((b - a) & 1) != 0)
    {
        return false;
    }
    if !check_offset_relation(d(19), d(20), d(21), 4) {
        return false;
    }
    if !check_offset_relation(d(21), d(22), d(23), 8) {
        return false;
    }
    if let (Some(a), Some(b)) = (d(23), d(25))
        && (b < a || ((b - a) & 1) != 0)
    {
        return false;
    }
    if !check_offset_relation(d(25), d(26), d(27), 8) {
        return false;
    }
    if let (Some(a), Some(b)) = (d(27), d(29))
        && (b < a || ((b - a) & 1) != 0)
    {
        return false;
    }
    if !check_offset_relation(d(29), d(30), d(31), 4) {
        return false;
    }

    true
}

fn check_offset_relation(a: Option<u32>, cnt: Option<u32>, b: Option<u32>, elem_size: u32) -> bool {
    match (a, cnt, b) {
        (Some(a), Some(cnt), Some(b)) => {
            cnt <= 16_000_000 && b == a.saturating_add(cnt.saturating_mul(elem_size))
        }
        _ => true,
    }
}

fn full_key(key: &[Option<u8>; 16]) -> Option<[u8; 16]> {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = key[i]?;
    }
    Some(out)
}

fn validate_scene_prefix(key: &[u8; 16], masked: &[u8]) -> bool {
    if masked.len() < 12 {
        return false;
    }
    // 132 output bytes can never require more than 8 + 17 flag bytes +
    // 2*132 token bytes.  512 encrypted bytes is therefore ample, and keeps
    // the <=65536 completion loop independent of full scene size.
    let take = masked.len().min(MAX_SYMBOLIC_SRC);
    let mut plain = Vec::with_capacity(take);
    for (i, b) in masked[..take].iter().enumerate() {
        plain.push(*b ^ key[i & 0x0f]);
    }
    let comp_len = read_u32(&plain, 0).unwrap_or(0) as usize;
    let org_len = read_u32(&plain, 4).unwrap_or(0) as usize;
    let max_org = max_lz_output_bound(masked.len()).min(MAX_REASONABLE_DECOMPRESSED);
    if comp_len != masked.len() || org_len < SCENE_HEADER_SIZE || org_len > max_org {
        return false;
    }
    let decoded = match decompress_lz_prefix_partial(&plain, masked.len(), SCENE_HEADER_SIZE) {
        Some(v) => v,
        None => return false,
    };
    validate_scene_header(&decoded, Some(org_len))
}

fn validate_scene_pack_key(key: &[u8; 16], pack: &ParsedScenePack, verbose: u8) -> bool {
    let mut checked = 0usize;
    for blob in &pack.blobs {
        let plain = decrypt_masked(&blob.masked, key);
        let comp_len = match read_u32(&plain, 0) {
            Some(v) => v as usize,
            None => return false,
        };
        let org_len = match read_u32(&plain, 4) {
            Some(v) => v as usize,
            None => return false,
        };
        let max_org = max_lz_output_bound(plain.len()).min(MAX_REASONABLE_DECOMPRESSED);
        if comp_len != plain.len() || org_len < SCENE_HEADER_SIZE || org_len > max_org {
            return false;
        }
        let decoded = match decompress_siglus_lz(&plain) {
            Some(v) => v,
            None => return false,
        };
        if decoded.len() != org_len || !validate_scene_header(&decoded, Some(org_len)) {
            return false;
        }
        checked += 1;
        if checked >= 16 {
            break;
        }
    }
    if verbose > 0 {
        eprintln!("scene verify   : {checked} scene(s) fully decompressed and validated");
    }
    checked > 0
}

fn validate_gameexe_key(key: &[u8; 16], data: &[u8]) -> bool {
    if data.len() < 16 || read_u32(data, 4) == Some(0) {
        return false;
    }
    let mut plain = Vec::with_capacity(data.len() - 8);
    for (i, b) in data[8..].iter().enumerate() {
        plain.push(*b ^ GAMEEXE_KEY[i & 0xff] ^ key[i & 0x0f]);
    }
    let comp_len = match read_u32(&plain, 0) {
        Some(v) => v as usize,
        None => return false,
    };
    let org_len = match read_u32(&plain, 4) {
        Some(v) => v as usize,
        None => return false,
    };
    let max_org = max_lz_output_bound(plain.len()).min(MAX_REASONABLE_DECOMPRESSED);
    if comp_len != plain.len() || org_len == 0 || org_len > max_org || (org_len & 1) != 0 {
        return false;
    }
    let decoded = match decompress_siglus_lz(&plain) {
        Some(v) => v,
        None => return false,
    };
    if decoded.len() != org_len || (decoded.len() & 1) != 0 {
        return false;
    }
    let words = decoded
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u16::from_le_bytes([p[0], p[1]]))
        .collect::<Vec<_>>();
    let text = match String::from_utf16(&words) {
        Ok(v) => v,
        Err(_) => return false,
    };
    looks_like_gameexe_text(&text)
}

fn looks_like_gameexe_text(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    let mut useful = 0usize;
    let mut total = 0usize;
    for ch in text.chars().take(8192) {
        total += 1;
        if !ch.is_control() || matches!(ch, '\r' | '\n' | '\t') {
            useful += 1;
        }
    }
    total > 0 && useful * 100 >= total * 90
}

fn runtime_i32_usize(v: u32) -> usize {
    (v as i32).max(0) as usize
}

fn runtime_range_fits(ofs: u32, cnt: u32, elem_size: usize, chunk_len: usize) -> bool {
    let ofs = runtime_i32_usize(ofs);
    let cnt = runtime_i32_usize(cnt);
    match cnt.checked_mul(elem_size).and_then(|n| ofs.checked_add(n)) {
        Some(end) => end <= chunk_len,
        None => false,
    }
}

fn validate_scene_runtime_bounds(d: &[u32; 33], chunk_len: usize) -> bool {
    // Mirror the hard construction checks in the Rust SceneStream: negative
    // i32 fields clamp to zero, and the scene/string-index/label/z-label
    // ranges must stay inside the decompressed chunk. Name maps are
    // intentionally not hard failures in SceneStream and are not checked here.
    let scn_ofs = runtime_i32_usize(d[1]);
    let scn_size = runtime_i32_usize(d[2]);
    match scn_ofs.checked_add(scn_size) {
        Some(end) if end <= chunk_len => {}
        _ => return false,
    }
    if !runtime_range_fits(d[3], d[4], 8, chunk_len) {
        return false;
    }
    if runtime_i32_usize(d[5]) > chunk_len {
        return false;
    }
    if !runtime_range_fits(d[7], d[8], 4, chunk_len) {
        return false;
    }
    if !runtime_range_fits(d[9], d[10], 4, chunk_len) {
        return false;
    }
    true
}

fn validate_scene_header(decoded: &[u8], expected_org_size: Option<usize>) -> bool {
    if decoded.len() < SCENE_HEADER_SIZE {
        return false;
    }
    let mut d = [0u32; 33];
    for (i, slot) in d.iter_mut().enumerate() {
        *slot = match read_u32(decoded, i * 4) {
            Some(v) => v,
            None => return false,
        };
    }
    let org_size = expected_org_size.unwrap_or(decoded.len());
    if !validate_scene_runtime_bounds(&d, org_size) {
        return false;
    }

    // z_label_cnt is dynamic. The original compiler assigns it from
    // out_scn.z_label_list.size(), and the Rust SceneStream treats it as a
    // normal count used only for bounds/jump checks.
    if d[0] != SCENE_HEADER_SIZE as u32 || d[3] != SCENE_HEADER_SIZE as u32 {
        return false;
    }
    if d[4] != d[6]
        || d[14] != d[16]
        || d[14] != d[18]
        || d[20] != d[22]
        || d[20] != d[24]
        || d[26] != d[28]
    {
        return false;
    }
    if d[5] != 132u32.saturating_add(d[4].saturating_mul(8)) {
        return false;
    }
    if d[1] < d[5] || ((d[1] - d[5]) & 1) != 0 || d[2] == 0 || d[7] != d[1].saturating_add(d[2]) {
        return false;
    }
    if !check_offset_relation(Some(d[7]), Some(d[8]), Some(d[9]), 4)
        || !check_offset_relation(Some(d[9]), Some(d[10]), Some(d[11]), 4)
        || !check_offset_relation(Some(d[11]), Some(d[12]), Some(d[13]), 8)
        || !check_offset_relation(Some(d[13]), Some(d[14]), Some(d[15]), 8)
        || !check_offset_relation(Some(d[15]), Some(d[16]), Some(d[17]), 8)
        || !check_offset_relation(Some(d[19]), Some(d[20]), Some(d[21]), 4)
        || !check_offset_relation(Some(d[21]), Some(d[22]), Some(d[23]), 8)
        || !check_offset_relation(Some(d[25]), Some(d[26]), Some(d[27]), 8)
        || !check_offset_relation(Some(d[29]), Some(d[30]), Some(d[31]), 4)
    {
        return false;
    }
    for (a, b) in [(17usize, 19usize), (23, 25), (27, 29)] {
        if d[b] < d[a] || ((d[b] - d[a]) & 1) != 0 {
            return false;
        }
    }
    if d[31].saturating_add(d[32].saturating_mul(4)) as usize != org_size {
        return false;
    }
    for &idx in &[
        1usize, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31,
    ] {
        if d[idx] as usize > org_size {
            return false;
        }
    }
    true
}

fn decompress_lz_prefix_partial(
    buffer: &[u8],
    full_comp_len: usize,
    target: usize,
) -> Option<Vec<u8>> {
    if buffer.len() < 8 {
        return None;
    }
    let comp_len = read_u32(buffer, 0)? as usize;
    let org_len = read_u32(buffer, 4)? as usize;
    let max_org = max_lz_output_bound(full_comp_len).min(MAX_REASONABLE_DECOMPRESSED);
    if comp_len != full_comp_len || org_len < target || org_len > max_org {
        return None;
    }
    let mut out = Vec::<u8>::with_capacity(target);
    let mut src = 8usize;
    while out.len() < target {
        let mut flags = *buffer.get(src)?;
        src += 1;
        for _ in 0..8 {
            if out.len() >= target {
                break;
            }
            if (flags & 1) != 0 {
                out.push(*buffer.get(src)?);
                src += 1;
            } else {
                let lo = *buffer.get(src)?;
                let hi = *buffer.get(src + 1)?;
                src += 2;
                let raw = u16::from_le_bytes([lo, hi]);
                let dist = (raw >> 4) as usize;
                let len = ((raw & 0x0f) as usize) + 2;
                if dist == 0 || dist > out.len() {
                    return None;
                }
                for _ in 0..len {
                    if out.len() >= target {
                        break;
                    }
                    let b = out[out.len() - dist];
                    out.push(b);
                }
            }
            flags >>= 1;
        }
    }
    Some(out)
}

fn decrypt_masked(masked: &[u8], key: &[u8; 16]) -> Vec<u8> {
    masked
        .iter()
        .enumerate()
        .map(|(i, b)| *b ^ key[i & 0x0f])
        .collect()
}

/// XOR `data` with a cyclically repeated `key` of any non-zero length.
fn xor_cycle(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, b)| *b ^ key[i % key.len()])
        .collect()
}

/// Whether a 132-byte scene header describes an object of `org_size` bytes.
///
/// This keeps every relation the runtime needs — `validate_scene_runtime_bounds`
/// plus the header's own size, table and offset relations — but drops the two
/// relations that only describe where the *original compiler* placed the
/// string table:
///
/// * `str_list_ofs == 132 + str_index_cnt * 8`
/// * `scn_ofs >= str_list_ofs`
///
/// A tool that appends a new string table to a compiled scene and retargets
/// `str_list_ofs` breaks both while the runtime still loads the scene, because
/// the runtime reads `str_list_ofs` from the header instead of deriving it.
fn scene_object_layout_ok(header: &[u8], org_size: usize) -> bool {
    if header.len() < SCENE_HEADER_SIZE {
        return false;
    }
    let mut d = [0u32; 33];
    for (i, slot) in d.iter_mut().enumerate() {
        match read_u32(header, i * 4) {
            Some(v) => *slot = v,
            None => return false,
        }
    }
    if !validate_scene_runtime_bounds(&d, org_size) {
        return false;
    }
    if d[0] != SCENE_HEADER_SIZE as u32 || d[3] != SCENE_HEADER_SIZE as u32 {
        return false;
    }
    if d[4] != d[6]
        || d[14] != d[16]
        || d[14] != d[18]
        || d[20] != d[22]
        || d[20] != d[24]
        || d[26] != d[28]
    {
        return false;
    }
    if d[2] == 0 || d[7] != d[1].saturating_add(d[2]) {
        return false;
    }
    if !check_offset_relation(Some(d[7]), Some(d[8]), Some(d[9]), 4)
        || !check_offset_relation(Some(d[9]), Some(d[10]), Some(d[11]), 4)
        || !check_offset_relation(Some(d[11]), Some(d[12]), Some(d[13]), 8)
        || !check_offset_relation(Some(d[13]), Some(d[14]), Some(d[15]), 8)
        || !check_offset_relation(Some(d[15]), Some(d[16]), Some(d[17]), 8)
        || !check_offset_relation(Some(d[19]), Some(d[20]), Some(d[21]), 4)
        || !check_offset_relation(Some(d[21]), Some(d[22]), Some(d[23]), 8)
        || !check_offset_relation(Some(d[25]), Some(d[26]), Some(d[27]), 8)
        || !check_offset_relation(Some(d[29]), Some(d[30]), Some(d[31]), 4)
    {
        return false;
    }
    for (a, b) in [(17usize, 19usize), (23, 25), (27, 29)] {
        if d[b] < d[a] || ((d[b] - d[a]) & 1) != 0 {
            return false;
        }
    }
    if d[31].saturating_add(d[32].saturating_mul(4)) as usize > org_size {
        return false;
    }
    for &idx in &[
        1usize, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31,
    ] {
        if d[idx] as usize > org_size {
            return false;
        }
    }
    true
}

fn read_u32(data: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(off..off + 4)?.try_into().ok()?))
}

fn dedup_states(states: &mut Vec<CrackState>) {
    let mut seen = HashSet::new();
    states.retain(|s| seen.insert(s.clone()));
}

fn dedup_flag_states(states: &mut Vec<(CrackState, u8)>) {
    let mut seen = HashSet::new();
    states.retain(|s| seen.insert(s.clone()));
}

fn dedup_partial_keys(keys: &mut Vec<[Option<u8>; 16]>) {
    let mut seen = HashSet::new();
    keys.retain(|k| seen.insert(*k));
}

fn dedup_full_keys(keys: &mut Vec<[u8; 16]>) {
    let mut seen = HashSet::new();
    keys.retain(|k| seen.insert(*k));
}

fn hex16(key: &[u8; 16]) -> String {
    key.iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn decompress_siglus_lz(buffer: &[u8]) -> Option<Vec<u8>> {
    if buffer.len() < 8 {
        return None;
    }
    let comp_len = u32::from_le_bytes(buffer[0..4].try_into().ok()?) as usize;
    let decomp_len = u32::from_le_bytes(buffer[4..8].try_into().ok()?) as usize;
    if comp_len != buffer.len() || decomp_len == 0 {
        return None;
    }

    let mut out = vec![0u8; decomp_len];
    let mut src = 8usize;
    let mut dst = 0usize;

    while dst < decomp_len {
        if src >= buffer.len() {
            return None;
        }
        let mut flags = buffer[src];
        src += 1;
        for _ in 0..8 {
            if dst >= decomp_len {
                break;
            }
            if (flags & 1) != 0 {
                if src >= buffer.len() {
                    return None;
                }
                out[dst] = buffer[src];
                dst += 1;
                src += 1;
            } else {
                if src + 1 >= buffer.len() {
                    return None;
                }
                let raw = u16::from_le_bytes([buffer[src], buffer[src + 1]]);
                src += 2;
                let dist = (raw >> 4) as usize;
                let len = ((raw & 0x0F) as usize) + 2;
                if dist == 0 || dist > dst {
                    return None;
                }
                for _ in 0..len {
                    if dst >= decomp_len {
                        break;
                    }
                    let byte = out[dst - dist];
                    out[dst] = byte;
                    dst += 1;
                }
            }
            flags >>= 1;
        }
    }

    Some(out)
}

#[cfg(test)]
mod configured_key_tests {
    use super::*;

    const TEST_KEY: [u8; 16] = [
        0x36, 0x0F, 0xC9, 0x37, 0x2E, 0xBA, 0x09, 0xDC, 0xE4, 0x0D, 0xF2, 0x00, 0x23, 0xA3, 0x6E,
        0x94,
    ];
    const WRONG_KEY: [u8; 16] = [0x5A; 16];

    /// A valid Siglus LZSS stream that stores every byte as a literal.
    fn literal_lzss(src: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; 8];
        for group in src.chunks(8) {
            out.push(0xFF);
            out.extend_from_slice(group);
        }
        let arc = out.len() as u32;
        let org = src.len() as u32;
        out[0..4].copy_from_slice(&arc.to_le_bytes());
        out[4..8].copy_from_slice(&org.to_le_bytes());
        out
    }

    fn write_u32(buf: &mut [u8], off: usize, value: u32) {
        buf[off..off + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn utf16le(text: &str) -> Vec<u8> {
        text.encode_utf16().flat_map(|w| w.to_le_bytes()).collect()
    }

    /// Bytes a text repack appends when it moves the string table.
    const APPENDED_STRING_TABLE: usize = 64;

    /// A compiled scene object in the original compiler's layout.
    ///
    /// With `repacked` the object models a text repack: the new string table is
    /// appended at the original end, `str_list_ofs` is retargeted to it, and
    /// the trailing table end keeps pointing at the pre-append size.
    fn scene_object(repacked: bool) -> Vec<u8> {
        let mut d = [0u32; 33];
        d[0] = SCENE_HEADER_SIZE as u32;
        d[1] = 1000;
        d[2] = 8;
        d[3] = SCENE_HEADER_SIZE as u32;
        d[4] = 0;
        d[5] = SCENE_HEADER_SIZE as u32;
        let end = d[1] + d[2];
        // Every table is empty, so every list starts where the previous ended.
        for idx in [7usize, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31] {
            d[idx] = end;
        }

        let mut bytes = vec![0u8; end as usize];
        for (i, value) in d.iter().enumerate() {
            write_u32(&mut bytes, i * 4, *value);
        }
        if repacked {
            write_u32(&mut bytes, 4 * 4, 1);
            write_u32(&mut bytes, 5 * 4, end);
            write_u32(&mut bytes, 6 * 4, 1);
            bytes.extend(std::iter::repeat(0u8).take(APPENDED_STRING_TABLE));
        }
        bytes
    }

    /// `compressed` mirrors `original_source_header_size`, which is what the
    /// loader uses to decide whether to LZSS-decompress a chunk.
    fn scene_pack(objects: &[Vec<u8>], compressed: bool, key: &[u8; 16]) -> Vec<u8> {
        const HEADER: usize = SCENE_PACK_HEADER_SIZE;
        let index_ofs = HEADER;
        let data_ofs = HEADER + objects.len() * 8;

        let chunks = objects
            .iter()
            .map(|object| {
                let chunk = if compressed {
                    xor_cycle(&literal_lzss(object), &SCENE_KEY)
                } else {
                    object.clone()
                };
                xor_cycle(&chunk, key)
            })
            .collect::<Vec<_>>();

        let mut out = vec![0u8; data_ofs];
        write_u32(&mut out, 0, HEADER as u32);
        write_u32(&mut out, 17 * 4, index_ofs as u32);
        write_u32(&mut out, 18 * 4, objects.len() as u32);
        write_u32(&mut out, 19 * 4, data_ofs as u32);
        write_u32(&mut out, 20 * 4, objects.len() as u32);
        write_u32(&mut out, 21 * 4, 1);
        write_u32(&mut out, 22 * 4, if compressed { 870 } else { 0 });

        let mut rel = 0u32;
        for (i, chunk) in chunks.iter().enumerate() {
            write_u32(&mut out, index_ofs + i * 8, rel);
            write_u32(&mut out, index_ofs + i * 8 + 4, chunk.len() as u32);
            rel += chunk.len() as u32;
        }
        for chunk in &chunks {
            out.extend_from_slice(chunk);
        }
        out
    }

    fn gameexe(text: &str, key: &[u8; 16], exe_angou_mode: u32) -> Vec<u8> {
        let body = xor_cycle(&xor_cycle(&literal_lzss(&utf16le(text)), &GAMEEXE_KEY), key);
        let mut out = Vec::new();
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&exe_angou_mode.to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn fixtures() -> (Vec<u8>, Vec<u8>) {
        let game = gameexe("Rewrite+ configured key test\n", &TEST_KEY, 1);
        (game, scene_pack(&[scene_object(false)], true, &TEST_KEY))
    }

    #[test]
    fn accepts_a_compressed_pack() {
        let (game, scene) = fixtures();
        assert_eq!(check_key(&game, &scene, &TEST_KEY), Ok(KeyStatus::Accepted));
        assert_eq!(validate_key_quick(&game, &scene, &TEST_KEY), Ok(true));
    }

    #[test]
    fn accepts_a_repacked_pack_whose_string_table_moved() {
        let game = gameexe("Rewrite+ configured key test\n", &TEST_KEY, 1);
        let scene = scene_pack(
            &[scene_object(false), scene_object(true), scene_object(true)],
            true,
            &TEST_KEY,
        );
        assert_eq!(check_key(&game, &scene, &TEST_KEY), Ok(KeyStatus::Accepted));
        // The crack invariants still reject this layout, which is why the port
        // used to fall through to a full - and for such packs futile - recovery.
        assert_eq!(validate_key_quick(&game, &scene, &TEST_KEY), Ok(false));
    }

    #[test]
    fn rejects_a_wrong_key_for_a_compressed_pack() {
        let (game, scene) = fixtures();
        assert_eq!(
            check_key(&game, &scene, &WRONG_KEY),
            Ok(KeyStatus::Mismatch)
        );
    }

    #[test]
    fn accepts_an_easy_link_pack() {
        let game = gameexe("Rewrite+ configured key test\n", &TEST_KEY, 1);
        let scene = scene_pack(&[scene_object(false), scene_object(true)], false, &TEST_KEY);
        assert_eq!(scene_pack_is_compressed(&scene), Ok(false));
        assert_eq!(check_key(&game, &scene, &TEST_KEY), Ok(KeyStatus::Accepted));
    }

    #[test]
    fn an_easy_link_pack_cannot_disprove_a_key() {
        // Uncompressed chunks carry no compressed-resource structure, and the
        // loader keeps their bytes as they are, so the caller must keep the
        // configured key instead of running the resource crack.
        let game = gameexe("Rewrite+ configured key test\n", &TEST_KEY, 1);
        let scene = scene_pack(&[scene_object(false)], false, &TEST_KEY);
        assert_eq!(
            check_key(&game, &scene, &WRONG_KEY),
            Ok(KeyStatus::Unverifiable)
        );
    }
}
