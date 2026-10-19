use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use siglus_compiler_common::{lzss_pack, put_i32, put_index, utf16le, xor_cycle};

pub const PACK_HEADER_FIELDS: usize = 23;
pub const PACK_HEADER_SIZE: usize = PACK_HEADER_FIELDS * 4;
pub const SCENE_HEADER_FIELDS: usize = 33;
pub const SCENE_HEADER_SIZE: usize = SCENE_HEADER_FIELDS * 4;

/// Field order copied from `S_tnm_scn_header` (132 bytes, little-endian i32).
pub const SCENE_HEADER_FIELD_NAMES: [&str; SCENE_HEADER_FIELDS] = [
    "header_size",
    "scn_ofs",
    "scn_size",
    "str_index_list_ofs",
    "str_index_cnt",
    "str_list_ofs",
    "str_cnt",
    "label_list_ofs",
    "label_cnt",
    "z_label_list_ofs",
    "z_label_cnt",
    "cmd_label_list_ofs",
    "cmd_label_cnt",
    "scn_prop_list_ofs",
    "scn_prop_cnt",
    "scn_prop_name_index_list_ofs",
    "scn_prop_name_index_cnt",
    "scn_prop_name_list_ofs",
    "scn_prop_name_cnt",
    "scn_cmd_list_ofs",
    "scn_cmd_cnt",
    "scn_cmd_name_index_list_ofs",
    "scn_cmd_name_index_cnt",
    "scn_cmd_name_list_ofs",
    "scn_cmd_name_cnt",
    "call_prop_name_index_list_ofs",
    "call_prop_name_index_cnt",
    "call_prop_name_list_ofs",
    "call_prop_name_cnt",
    "namae_list_ofs",
    "namae_cnt",
    "read_flag_list_ofs",
    "read_flag_cnt",
];

/// Field order copied from `S_tnm_pack_scn_header` (92 bytes, little-endian i32).
pub const PACK_HEADER_FIELD_NAMES: [&str; PACK_HEADER_FIELDS] = [
    "header_size",
    "inc_prop_list_ofs",
    "inc_prop_cnt",
    "inc_prop_name_index_list_ofs",
    "inc_prop_name_index_cnt",
    "inc_prop_name_list_ofs",
    "inc_prop_name_cnt",
    "inc_cmd_list_ofs",
    "inc_cmd_cnt",
    "inc_cmd_name_index_list_ofs",
    "inc_cmd_name_index_cnt",
    "inc_cmd_name_list_ofs",
    "inc_cmd_name_cnt",
    "scn_name_index_list_ofs",
    "scn_name_index_cnt",
    "scn_name_list_ofs",
    "scn_name_cnt",
    "scn_data_index_list_ofs",
    "scn_data_index_cnt",
    "scn_data_list_ofs",
    "scn_data_cnt",
    "scn_data_exe_angou_mod",
    "original_source_header_size",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncProperty {
    pub name: String,
    pub form: i32,
    pub size: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncCommand {
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Definitions {
    pub properties: Vec<IncProperty>,
    pub commands: Vec<IncCommand>,
}

#[derive(Debug, Clone)]
pub struct SceneInput {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SourceInput {
    /// Relative Windows source name embedded in the encrypted object.
    pub name: String,
    /// Original source bytes (the C++ writer encrypts bytes, not decoded text).
    pub data: Vec<u8>,
}

/// Values declared by the absent `tnm_source_angou.h`. The source-tail writer
/// itself is fully implemented; only these unrecovered constants are external.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEncryptionParameters {
    pub easy_key: Vec<u8>,
    pub easy_index: usize,
    pub mask_key: Vec<u8>,
    pub mask_index: usize,
    pub garbage_key: Vec<u8>,
    pub garbage_index: usize,
    pub final_key: Vec<u8>,
    pub final_index: usize,
    pub name_key: Vec<u8>,
    pub name_index: usize,
    pub mask_width_md5_index: usize,
    pub mask_width_modulus: usize,
    pub mask_width_add: usize,
    pub mask_height_md5_index: usize,
    pub mask_height_modulus: usize,
    pub mask_height_add: usize,
    pub mask_md5_index: usize,
    pub map_width_md5_index: usize,
    pub map_width_modulus: usize,
    pub map_width_add: usize,
    pub garbage_md5_index: usize,
    pub tile_repeat_x: i32,
    pub tile_repeat_y: i32,
    pub tile_limit: u8,
}

#[derive(Debug, Clone)]
pub struct LinkOptions {
    /// `false` is the original compiler's easy-link representation.
    pub compress: bool,
    /// Missing recovered `TNM_EASY_ANGOU_CODE`, with an override required by
    /// callers that do not accept the workspace-recovered default.
    pub fixed_scene_key: Vec<u8>,
    pub exe_key: Option<[u8; 16]>,
    /// Original files in the exact category/sort order used by the C++ linker.
    pub original_sources: Vec<SourceInput>,
    /// Constants from the absent `tnm_source_angou.h`.
    pub source_encryption: Option<SourceEncryptionParameters>,
}

#[derive(Debug, Clone, Copy)]
struct CommandLabel {
    id: i32,
    offset: i32,
}

#[derive(Debug, Clone)]
struct ParsedScene {
    labels: Vec<CommandLabel>,
}

pub fn link(
    scenes: &[SceneInput],
    definitions: &Definitions,
    options: &LinkOptions,
) -> Result<Vec<u8>> {
    if scenes.is_empty() {
        bail!("at least one compiled scene is required");
    }
    if options.compress {
        if options.fixed_scene_key.is_empty() {
            bail!("fixed scene XOR key cannot be empty in compressed mode");
        }
        if options.source_encryption.is_none() {
            bail!(
                "compressed Scene.pck requires source-encryption parameters from the absent tnm_source_angou.h"
            );
        }
    } else if options.source_encryption.is_some() || !options.original_sources.is_empty() {
        bail!("original sources are only valid in compressed mode");
    }

    validate_definitions(definitions)?;
    let mut seen_scene_names = HashSet::new();
    let mut parsed = Vec::with_capacity(scenes.len());
    for scene in scenes {
        let normalized = scene.name.to_ascii_lowercase();
        if normalized.is_empty() {
            bail!("scene name cannot be empty");
        }
        if !seen_scene_names.insert(normalized.clone()) {
            bail!("duplicate scene name: {normalized}");
        }
        parsed.push(parse_scene(&scene.data).with_context(|| format!("scene {normalized}"))?);
    }

    let mut resolved: Vec<Option<(i32, i32)>> = vec![None; definitions.commands.len()];
    for (scene_index, scene) in parsed.iter().enumerate() {
        for label in &scene.labels {
            if label.id < 0 {
                bail!(
                    "scene {} contains negative command id {}",
                    scenes[scene_index].name,
                    label.id
                );
            }
            let id = label.id as usize;
            // Scene-local command labels follow the global inc range and are
            // deliberately ignored by the original linker.
            if id >= definitions.commands.len() {
                continue;
            }
            if resolved[id].is_some() {
                bail!(
                    "command {} is defined more than once",
                    definitions.commands[id].name
                );
            }
            resolved[id] = Some((scene_index as i32, label.offset));
        }
    }
    for (index, command) in definitions.commands.iter().enumerate() {
        if resolved[index].is_none() {
            bail!("command {} is declared but unresolved", command.name);
        }
    }

    let scene_names: Vec<String> = scenes.iter().map(|s| s.name.to_ascii_lowercase()).collect();
    let property_names: Vec<String> = definitions
        .properties
        .iter()
        .map(|p| p.name.clone())
        .collect();
    let command_names: Vec<String> = definitions
        .commands
        .iter()
        .map(|c| c.name.clone())
        .collect();

    // A zero executable key is the explicit development/default value, not an
    // encrypted package key. Keep the pack header and payload consistent with
    // Gameexe.dat's zero-key handling.
    let exe_key = options
        .exe_key
        .filter(|key| key.iter().any(|&byte| byte != 0));
    let mut chunks = Vec::with_capacity(scenes.len());
    for scene in scenes {
        let mut chunk = if options.compress {
            let mut packed = lzss_pack(&scene.data);
            xor_cycle(&mut packed, &options.fixed_scene_key);
            packed
        } else {
            scene.data.clone()
        };
        if let Some(exe_key) = exe_key {
            xor_cycle(&mut chunk, &exe_key);
        }
        chunks.push(chunk);
    }

    let mut out = vec![0u8; PACK_HEADER_SIZE];
    let mut header = [0i32; PACK_HEADER_FIELDS];
    header[0] = PACK_HEADER_SIZE as i32;

    header[1] = offset(&out)?;
    header[2] = definitions.properties.len() as i32;
    for property in &definitions.properties {
        put_i32(&mut out, property.form);
        put_i32(&mut out, property.size);
    }
    header[3] = offset(&out)?;
    header[4] = property_names.len() as i32;
    append_indices(&mut out, &property_names)?;
    header[5] = offset(&out)?;
    header[6] = property_names.len() as i32;
    append_strings(&mut out, &property_names);

    header[7] = offset(&out)?;
    header[8] = definitions.commands.len() as i32;
    for item in &resolved {
        let (scene_no, command_offset) = item.expect("resolution checked above");
        put_i32(&mut out, scene_no);
        put_i32(&mut out, command_offset);
    }
    header[9] = offset(&out)?;
    header[10] = command_names.len() as i32;
    append_indices(&mut out, &command_names)?;
    header[11] = offset(&out)?;
    header[12] = command_names.len() as i32;
    append_strings(&mut out, &command_names);

    header[13] = offset(&out)?;
    header[14] = scene_names.len() as i32;
    append_indices(&mut out, &scene_names)?;
    header[15] = offset(&out)?;
    header[16] = scene_names.len() as i32;
    append_strings(&mut out, &scene_names);

    header[17] = offset(&out)?;
    header[18] = chunks.len() as i32;
    let mut chunk_offset = 0i32;
    for chunk in &chunks {
        put_index(&mut out, chunk_offset, i32::try_from(chunk.len())?);
        chunk_offset = chunk_offset
            .checked_add(i32::try_from(chunk.len())?)
            .ok_or_else(|| anyhow!("combined scene data exceeds i32"))?;
    }
    header[19] = offset(&out)?;
    header[20] = chunks.len() as i32;
    header[21] = exe_key.is_some() as i32;
    let source_archive = if options.compress {
        Some(build_source_archive(
            &options.original_sources,
            options.source_encryption.as_ref().expect("checked above"),
        )?)
    } else {
        None
    };
    header[22] = source_archive
        .as_ref()
        .map_or(0, |archive| archive.header_size);
    for chunk in &chunks {
        out.extend_from_slice(chunk);
    }
    if let Some(archive) = source_archive {
        out.extend_from_slice(&archive.data);
    }
    for (i, field) in header.into_iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&field.to_le_bytes());
    }
    Ok(out)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceArchive {
    pub header_size: i32,
    pub object_sizes: Vec<u32>,
    pub data: Vec<u8>,
}

/// Builds `[encrypted size-list object][encrypted source object ...]` exactly
/// as `main_proc_link.cpp` does.
pub fn build_source_archive(
    sources: &[SourceInput],
    parameters: &SourceEncryptionParameters,
) -> Result<SourceArchive> {
    validate_source_parameters(parameters)?;
    let mut objects = Vec::with_capacity(sources.len());
    let mut object_sizes = Vec::with_capacity(sources.len());
    for source in sources {
        let object = encrypt_source(&source.data, &source.name, parameters)
            .with_context(|| format!("encrypt original source {}", source.name))?;
        object_sizes.push(u32::try_from(object.len())?);
        objects.push(object);
    }
    let mut size_bytes = Vec::with_capacity(object_sizes.len() * 4);
    for size in &object_sizes {
        size_bytes.extend_from_slice(&size.to_le_bytes());
    }
    let size_object = encrypt_source(&size_bytes, "__DummyName__", parameters)?;
    let header_size = i32::try_from(size_object.len())?;
    let total = size_object.len() + objects.iter().map(Vec::len).sum::<usize>();
    let mut data = Vec::with_capacity(total);
    data.extend_from_slice(&size_object);
    for object in objects {
        data.extend_from_slice(&object);
    }
    Ok(SourceArchive {
        header_size,
        object_sizes,
        data,
    })
}

/// Implements `C_tnms_source_angou::encryption` from `source_angou.cpp`.
pub fn encrypt_source(
    source: &[u8],
    name: &str,
    parameters: &SourceEncryptionParameters,
) -> Result<Vec<u8>> {
    validate_source_parameters(parameters)?;
    let mut packed = if source.is_empty() {
        Vec::new()
    } else {
        lzss_pack(source)
    };
    xor_from(&mut packed, &parameters.easy_key, parameters.easy_index);
    let digest = md5::compute(&packed).0;

    let mut name_data = utf16le(name);
    xor_from(&mut name_data, &parameters.name_key, parameters.name_index);

    let mask_width = usize::from(digest[parameters.mask_width_md5_index])
        % parameters.mask_width_modulus
        + parameters.mask_width_add;
    let mask_height = usize::from(digest[parameters.mask_height_md5_index])
        % parameters.mask_height_modulus
        + parameters.mask_height_add;
    let mut mask = vec![0; mask_width * mask_height];
    for (index, byte) in mask.iter_mut().enumerate() {
        *byte = parameters.mask_key[(parameters.mask_index + index) % parameters.mask_key.len()]
            ^ digest[(parameters.mask_md5_index + index) % digest.len()];
    }

    let map_width = usize::from(digest[parameters.map_width_md5_index])
        % parameters.map_width_modulus
        + parameters.map_width_add;
    let byte_half_size = packed.len().div_ceil(2);
    let dword_half_size = byte_half_size.div_ceil(4);
    let map_height = dword_half_size.div_ceil(map_width);
    let map_size = map_width
        .checked_mul(map_height)
        .and_then(|value| value.checked_mul(4))
        .ok_or_else(|| anyhow!("source map size overflow"))?;
    let original_packed_size = packed.len();
    packed.resize(map_size * 2, 0);
    for index in original_packed_size..packed.len() {
        let garbage_index = parameters.garbage_index + index - original_packed_size;
        let md5_index = parameters.garbage_md5_index + index - original_packed_size;
        packed[index] = parameters.garbage_key[garbage_index % parameters.garbage_key.len()]
            ^ digest[md5_index % digest.len()];
    }

    // Stns_source_angou_header is two little-endian i32s followed by MD5_CODE_CNT bytes.
    let mut out = Vec::with_capacity(24 + 4 + name_data.len() + map_size * 2);
    put_i32(&mut out, 1);
    put_i32(&mut out, i32::try_from(original_packed_size)?);
    out.extend_from_slice(&digest);
    put_i32(&mut out, i32::try_from(name_data.len())?);
    out.extend_from_slice(&name_data);
    let maps_start = out.len();
    out.resize(maps_start + map_size * 2, 0);

    tile_copy(
        &mut out[maps_start..maps_start + map_size],
        &packed[..map_size],
        map_width,
        map_height,
        &mask,
        mask_width,
        mask_height,
        parameters,
        false,
    );
    tile_copy(
        &mut out[maps_start..maps_start + map_size],
        &packed[byte_half_size..byte_half_size + map_size],
        map_width,
        map_height,
        &mask,
        mask_width,
        mask_height,
        parameters,
        true,
    );
    tile_copy(
        &mut out[maps_start + map_size..maps_start + map_size * 2],
        &packed[..map_size],
        map_width,
        map_height,
        &mask,
        mask_width,
        mask_height,
        parameters,
        true,
    );
    tile_copy(
        &mut out[maps_start + map_size..maps_start + map_size * 2],
        &packed[byte_half_size..byte_half_size + map_size],
        map_width,
        map_height,
        &mask,
        mask_width,
        mask_height,
        parameters,
        false,
    );
    xor_from(&mut out, &parameters.final_key, parameters.final_index);
    Ok(out)
}

fn validate_source_parameters(parameters: &SourceEncryptionParameters) -> Result<()> {
    for (name, key) in [
        ("easy_key", &parameters.easy_key),
        ("mask_key", &parameters.mask_key),
        ("garbage_key", &parameters.garbage_key),
        ("final_key", &parameters.final_key),
        ("name_key", &parameters.name_key),
    ] {
        if key.is_empty() {
            bail!("source-encryption {name} cannot be empty");
        }
    }
    for (name, index) in [
        ("mask_width_md5_index", parameters.mask_width_md5_index),
        ("mask_height_md5_index", parameters.mask_height_md5_index),
        ("mask_md5_index", parameters.mask_md5_index),
        ("map_width_md5_index", parameters.map_width_md5_index),
        ("garbage_md5_index", parameters.garbage_md5_index),
    ] {
        if index >= 16 {
            bail!("source-encryption {name} must be below 16");
        }
    }
    for (name, value) in [
        ("mask_width_modulus", parameters.mask_width_modulus),
        ("mask_height_modulus", parameters.mask_height_modulus),
        ("map_width_modulus", parameters.map_width_modulus),
        ("mask_width_add", parameters.mask_width_add),
        ("mask_height_add", parameters.mask_height_add),
        ("map_width_add", parameters.map_width_add),
    ] {
        if value == 0 {
            bail!("source-encryption {name} cannot be zero");
        }
    }
    Ok(())
}

fn xor_from(data: &mut [u8], key: &[u8], start: usize) {
    for (index, byte) in data.iter_mut().enumerate() {
        *byte ^= key[(start + index) % key.len()];
    }
}

#[allow(clippy::too_many_arguments)]
fn tile_copy(
    destination: &mut [u8],
    source: &[u8],
    map_width: usize,
    map_height: usize,
    mask: &[u8],
    mask_width: usize,
    mask_height: usize,
    parameters: &SourceEncryptionParameters,
    reverse: bool,
) {
    let start_x = repeat_start(parameters.tile_repeat_x, mask_width);
    let start_y = repeat_start(parameters.tile_repeat_y, mask_height);
    for y in 0..map_height {
        let mask_y = (start_y + y) % mask_height;
        for x in 0..map_width {
            let mask_x = (start_x + x) % mask_width;
            let selected = mask[mask_y * mask_width + mask_x] >= parameters.tile_limit;
            if selected != reverse {
                let offset = (y * map_width + x) * 4;
                destination[offset..offset + 4].copy_from_slice(&source[offset..offset + 4]);
            }
        }
    }
}

fn repeat_start(repeat: i32, size: usize) -> usize {
    if repeat <= 0 {
        repeat.unsigned_abs() as usize % size
    } else {
        (size - repeat as usize % size) % size
    }
}

fn validate_definitions(definitions: &Definitions) -> Result<()> {
    let mut names = HashSet::new();
    for property in &definitions.properties {
        if !names.insert(property.name.clone()) {
            bail!("duplicate definition name: {}", property.name);
        }
    }
    for command in &definitions.commands {
        if !names.insert(command.name.clone()) {
            bail!("duplicate definition name: {}", command.name);
        }
    }
    Ok(())
}

fn parse_scene(data: &[u8]) -> Result<ParsedScene> {
    if data.len() < SCENE_HEADER_SIZE {
        bail!("compiled scene is shorter than its 132-byte header");
    }
    let field = |i: usize| i32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
    if field(0) != SCENE_HEADER_SIZE as i32 {
        bail!(
            "scene header_size is {}, expected {}",
            field(0),
            SCENE_HEADER_SIZE
        );
    }
    let list_offset = nonnegative(field(11), "cmd_label_list_ofs")?;
    let count = nonnegative(field(12), "cmd_label_cnt")?;
    let byte_count = count
        .checked_mul(8)
        .ok_or_else(|| anyhow!("command label size overflow"))?;
    if list_offset
        .checked_add(byte_count)
        .filter(|&end| end <= data.len())
        .is_none()
    {
        bail!("command label table is out of bounds");
    }
    let mut labels = Vec::with_capacity(count);
    for i in 0..count {
        let base = list_offset + i * 8;
        labels.push(CommandLabel {
            id: i32::from_le_bytes(data[base..base + 4].try_into().unwrap()),
            offset: i32::from_le_bytes(data[base + 4..base + 8].try_into().unwrap()),
        });
    }
    Ok(ParsedScene { labels })
}

fn nonnegative(value: i32, name: &str) -> Result<usize> {
    usize::try_from(value).with_context(|| format!("negative {name}: {value}"))
}

fn offset(out: &[u8]) -> Result<i32> {
    Ok(i32::try_from(out.len())?)
}

fn append_indices(out: &mut Vec<u8>, strings: &[String]) -> Result<()> {
    let mut string_offset = 0i32;
    for string in strings {
        let units = i32::try_from(string.encode_utf16().count())?;
        put_index(out, string_offset, units);
        string_offset = string_offset
            .checked_add(units)
            .ok_or_else(|| anyhow!("string table exceeds i32"))?;
    }
    Ok(())
}

fn append_strings(out: &mut Vec<u8>, strings: &[String]) {
    for string in strings {
        out.extend_from_slice(&utf16le(string));
    }
}

/// Parses the `#property` and `#command` declarations consumed by the C++ IA
/// pass. Macro expansion is intentionally outside the linker: callers should
/// pass the same expanded declarations used for scene compilation.
pub fn parse_inc_declarations(text: &str, forms: &HashMap<String, i32>) -> Result<Definitions> {
    let clean = strip_comments_and_lower(text)?;
    let mut definitions = Definitions::default();
    let mut positions = Vec::new();
    for needle in ["#property", "#command"] {
        let mut start = 0;
        while let Some(found) = clean[start..].find(needle) {
            let at = start + found;
            positions.push((at, needle));
            start = at + needle.len();
        }
    }
    positions.sort_by_key(|x| x.0);
    for (index, (position, kind)) in positions.iter().enumerate() {
        let start = position + kind.len();
        let end = positions.get(index + 1).map(|x| x.0).unwrap_or(clean.len());
        let declaration = clean[start..end].trim();
        if *kind == "#property" {
            definitions
                .properties
                .push(parse_property(declaration, forms)?);
        } else {
            let name = declaration
                .split(|c: char| c.is_whitespace() || c == '(' || c == ':')
                .next()
                .unwrap_or("");
            if name.is_empty() {
                bail!("#command is missing a name");
            }
            definitions.commands.push(IncCommand { name: name.into() });
        }
    }
    validate_definitions(&definitions)?;
    Ok(definitions)
}

fn parse_property(declaration: &str, forms: &HashMap<String, i32>) -> Result<IncProperty> {
    let name_end = declaration
        .find(|c: char| c.is_whitespace() || c == ':')
        .unwrap_or(declaration.len());
    let name = declaration[..name_end].trim();
    if name.is_empty() {
        bail!("#property is missing a name");
    }
    let mut form = 10;
    let mut size = 0;
    if let Some(colon) = declaration.find(':') {
        let rest = declaration[colon + 1..].trim_start();
        let form_name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        form = *forms
            .get(&form_name)
            .ok_or_else(|| anyhow!("unknown property form {form_name}"))?;
        if let Some(open) = rest.find('[') {
            let close = rest[open + 1..]
                .find(']')
                .map(|p| p + open + 1)
                .ok_or_else(|| anyhow!("unterminated property array size"))?;
            size = rest[open + 1..close].trim().parse()?;
            if form != 11 && form != 21 {
                bail!("only intlist and strlist properties may specify array size");
            }
        }
    }
    if form == 0 {
        bail!("void property is invalid");
    }
    Ok(IncProperty {
        name: name.into(),
        form,
        size,
    })
}

fn strip_comments_and_lower(text: &str) -> Result<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let mut quote = None;
    let mut line_comment = false;
    let mut block_comment = false;
    while i < chars.len() {
        let ch = chars[i];
        let next = chars.get(i + 1).copied();
        if ch == '\n' {
            line_comment = false;
            out.push(ch);
            i += 1;
            continue;
        }
        if line_comment {
            i += 1;
        } else if block_comment {
            if ch == '*' && next == Some('/') {
                block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
        } else if let Some(q) = quote {
            out.push(ch);
            if let ('\\', Some(next)) = (ch, next) {
                out.push(next);
                i += 2;
            } else {
                if ch == q {
                    quote = None;
                }
                i += 1;
            }
        } else if ch == '\'' || ch == '"' {
            quote = Some(ch);
            out.push(ch);
            i += 1;
        } else if ch == ';' || (ch == '/' && next == Some('/')) {
            line_comment = true;
            i += if ch == ';' { 1 } else { 2 };
        } else if ch == '/' && next == Some('*') {
            block_comment = true;
            i += 2;
        } else {
            out.extend(ch.to_lowercase());
            i += 1;
        }
    }
    if quote.is_some() {
        bail!("unterminated quote in inc definitions");
    }
    if block_comment {
        bail!("unterminated block comment in inc definitions");
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use siglus_compiler_common::{RECOVERED_SCENE_KEY, lzss_unpack};

    fn fake_scene(labels: &[(i32, i32)]) -> Vec<u8> {
        let mut data = vec![0u8; SCENE_HEADER_SIZE];
        data[0..4].copy_from_slice(&(SCENE_HEADER_SIZE as i32).to_le_bytes());
        data[44..48].copy_from_slice(&(SCENE_HEADER_SIZE as i32).to_le_bytes());
        data[48..52].copy_from_slice(&(labels.len() as i32).to_le_bytes());
        for (id, offset) in labels {
            put_i32(&mut data, *id);
            put_i32(&mut data, *offset);
        }
        data
    }

    fn source_parameters() -> SourceEncryptionParameters {
        SourceEncryptionParameters {
            easy_key: vec![1, 2, 3],
            easy_index: 1,
            mask_key: vec![4, 5, 6],
            mask_index: 2,
            garbage_key: vec![7, 8, 9],
            garbage_index: 0,
            final_key: vec![10, 11, 12],
            final_index: 1,
            name_key: vec![13, 14, 15],
            name_index: 2,
            mask_width_md5_index: 0,
            mask_width_modulus: 5,
            mask_width_add: 2,
            mask_height_md5_index: 1,
            mask_height_modulus: 5,
            mask_height_add: 2,
            mask_md5_index: 2,
            map_width_md5_index: 3,
            map_width_modulus: 5,
            map_width_add: 2,
            garbage_md5_index: 4,
            tile_repeat_x: 0,
            tile_repeat_y: 0,
            tile_limit: 128,
        }
    }

    #[test]
    fn links_and_resolves_commands() {
        let result = link(
            &[SceneInput {
                name: "Start".into(),
                data: fake_scene(&[(0, 123)]),
            }],
            &Definitions {
                properties: vec![IncProperty {
                    name: "$x".into(),
                    form: 10,
                    size: 0,
                }],
                commands: vec![IncCommand {
                    name: "$boot".into(),
                }],
            },
            &LinkOptions {
                compress: false,
                fixed_scene_key: RECOVERED_SCENE_KEY.to_vec(),
                exe_key: None,
                original_sources: vec![],
                source_encryption: None,
            },
        )
        .unwrap();
        assert_eq!(i32::from_le_bytes(result[0..4].try_into().unwrap()), 92);
        // Header (92) + one property (8) + its CIndex (8) + UTF-16 "$x" (4).
        assert_eq!(i32::from_le_bytes(result[28..32].try_into().unwrap()), 112);
    }

    #[test]
    fn zero_exe_key_is_unencrypted_development_mode() {
        let result = link(
            &[SceneInput {
                name: "Start".into(),
                data: fake_scene(&[]),
            }],
            &Definitions::default(),
            &LinkOptions {
                compress: false,
                fixed_scene_key: RECOVERED_SCENE_KEY.to_vec(),
                exe_key: Some([0; 16]),
                original_sources: vec![],
                source_encryption: None,
            },
        )
        .unwrap();
        assert_eq!(&result[21 * 4..22 * 4], &0i32.to_le_bytes());
    }

    #[test]
    fn compressed_encrypted_chunk_round_trips() {
        let scene = fake_scene(&[]);
        let exe_key = [0x5au8; 16];
        let result = link(
            &[SceneInput {
                name: "Compressed".into(),
                data: scene.clone(),
            }],
            &Definitions::default(),
            &LinkOptions {
                compress: true,
                fixed_scene_key: RECOVERED_SCENE_KEY.to_vec(),
                exe_key: Some(exe_key),
                original_sources: vec![SourceInput {
                    name: "start.ss".into(),
                    data: b"#z00\n".to_vec(),
                }],
                source_encryption: Some(source_parameters()),
            },
        )
        .unwrap();
        let data_ofs = i32::from_le_bytes(result[19 * 4..20 * 4].try_into().unwrap()) as usize;
        let index_ofs = i32::from_le_bytes(result[17 * 4..18 * 4].try_into().unwrap()) as usize;
        let chunk_size =
            i32::from_le_bytes(result[index_ofs + 4..index_ofs + 8].try_into().unwrap()) as usize;
        let mut chunk = result[data_ofs..data_ofs + chunk_size].to_vec();
        xor_cycle(&mut chunk, &exe_key);
        xor_cycle(&mut chunk, &RECOVERED_SCENE_KEY);
        assert_eq!(lzss_unpack(&chunk).unwrap(), scene);
        assert!(i32::from_le_bytes(result[88..92].try_into().unwrap()) > 0);
    }

    #[test]
    fn source_archive_contains_encrypted_size_header_and_objects() {
        let archive = build_source_archive(
            &[SourceInput {
                name: "start.ss".into(),
                data: b"abcabcabc".to_vec(),
            }],
            &source_parameters(),
        )
        .unwrap();
        assert!(archive.header_size > 28);
        assert_eq!(archive.object_sizes.len(), 1);
        assert_eq!(
            archive.data.len(),
            archive.header_size as usize + archive.object_sizes[0] as usize
        );
    }

    #[test]
    fn header_maps_match_binary_sizes() {
        assert_eq!(SCENE_HEADER_FIELD_NAMES.len(), SCENE_HEADER_FIELDS);
        assert_eq!(PACK_HEADER_FIELD_NAMES.len(), PACK_HEADER_FIELDS);
        assert_eq!(SCENE_HEADER_SIZE, 132);
        assert_eq!(PACK_HEADER_SIZE, 92);
    }
}
