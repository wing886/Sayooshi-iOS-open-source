//! Gameexe-driven table loading.
//!
//! This module records asset-table load failures in `UnknownOpRecorder`
//! and do not crash runtime bring-up.
//!
//! NOTE: `TCHAR` in the original engine is UTF-16LE.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use siglus_assets::{
    cgm::CgTableData,
    dbs::DbsDatabase,
    gameexe::{
        GameexeConfig, GameexeDecodeOptions, GameexeDecodeReport, decode_gameexe_dat_bytes,
        normalize_gameexe_key,
    },
    thumb_table::ThumbTable,
};

use super::unknown::UnknownOpRecorder;
const INIDEF_BTN_SE_CNT: usize = 16;
const INIMAX_BTN_SE_CNT: usize = 256;
const INIDEF_BTN_ACTION_CNT: usize = 16;
const INIMAX_BTN_ACTION_CNT: usize = 256;
const TNM_BTN_STATE_MAX: usize = 5;
const INIDEF_SE_CNT: usize = 16;
const INIMIN_SE_CNT: usize = 8;
const INIMAX_SE_CNT: usize = 256;
const INIDEF_MWND_CNT: usize = 2;
const INIMAX_MWND_CNT: usize = 256;
const INIDEF_WAKU_CNT: usize = 4;
const INIMAX_WAKU_CNT: usize = 256;
const INIDEF_MWND_WAKU_BTN_CNT: usize = 8;
const INIMAX_MWND_WAKU_BTN_CNT: usize = 256;
const INIDEF_MWND_WAKU_FACE_CNT: usize = 1;
const INIMAX_MWND_WAKU_FACE_CNT: usize = 16;
const INIDEF_MWND_WAKU_OBJECT_CNT: usize = 1;
const INIMAX_MWND_WAKU_OBJECT_CNT: usize = 16;
const INIDEF_ICON_CNT: usize = 16;
const INIMAX_ICON_CNT: usize = 256;
const INIDEF_SEL_BTN_CNT: usize = 16;
const INIMIN_SEL_BTN_CNT: usize = 0;
const INIMAX_SEL_BTN_CNT: usize = 256;
const INIDEF_SHAKE_CNT: usize = 16;
const INIMAX_SHAKE_CNT: usize = 256;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShakeStep {
    pub x: i32,
    pub y: i32,
    pub time_ms: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonSeTemplate {
    pub hit_no: i64,
    pub push_no: i64,
    pub decide_no: i64,
}

impl Default for ButtonSeTemplate {
    fn default() -> Self {
        Self {
            hit_no: 0,
            push_no: -1,
            decide_no: 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonActionPattern {
    pub rep_pat_no: i64,
    pub rep_pos_x: i64,
    pub rep_pos_y: i64,
    pub rep_tr: i64,
    pub rep_bright: i64,
    pub rep_dark: i64,
}

impl Default for ButtonActionPattern {
    fn default() -> Self {
        Self {
            rep_pat_no: 0,
            rep_pos_x: 0,
            rep_pos_y: 0,
            rep_tr: 255,
            rep_bright: 0,
            rep_dark: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonActionTemplate {
    pub state: [ButtonActionPattern; TNM_BTN_STATE_MAX],
}

impl Default for ButtonActionTemplate {
    fn default() -> Self {
        let mut state = [ButtonActionPattern::default(); TNM_BTN_STATE_MAX];
        state[1].rep_bright = 32;
        state[2].rep_bright = 32;
        state[2].rep_pos_x = 1;
        state[2].rep_pos_y = 1;
        Self { state }
    }
}

#[derive(Debug, Clone)]
pub struct MwndTemplate {
    pub novel_mode: i64,
    pub extend_type: i64,
    pub window_pos: (i64, i64),
    pub window_size: (i64, i64),
    pub message_pos: (i64, i64),
    pub message_margin: (i64, i64, i64, i64),
    pub moji_cnt: (i64, i64),
    pub moji_size: i64,
    pub moji_space: (i64, i64),
    pub moji_color: i64,
    pub shadow_color: i64,
    pub fuchi_color: i64,
    pub ruby_size: i64,
    pub ruby_space: i64,
    pub waku_no: i64,
    pub waku_pos: (i64, i64),
    pub name_disp_mode: i64,
    pub name_newline: i64,
    pub name_bracket: i64,
    pub name_moji_size: i64,
    pub name_moji_space: (i64, i64),
    pub name_moji_cnt: i64,
    pub name_window_pos: (i64, i64),
    pub name_window_size: (i64, i64),
    pub name_msg_pos: (i64, i64),
    pub name_msg_pos_rep: (i64, i64),
    pub name_msg_margin: (i64, i64, i64, i64),
    pub name_extend_type: i64,
    pub name_window_align: i64,
    pub name_moji_color: i64,
    pub name_shadow_color: i64,
    pub name_fuchi_color: i64,
    pub name_waku_no: i64,
    pub face_hide_name: i64,
    pub talk_margin: (i64, i64, i64, i64),
    pub overflow_check_size: i64,
    pub msg_back_insert_nl: i64,
    pub open_anime_type: i64,
    pub open_anime_time: i64,
    pub close_anime_type: i64,
    pub close_anime_time: i64,
}

impl Default for MwndTemplate {
    fn default() -> Self {
        Self {
            novel_mode: 0,
            extend_type: 0,
            window_pos: (50, 400),
            window_size: (700, 150),
            message_pos: (20, 20),
            message_margin: (20, 20, 20, 20),
            moji_cnt: (26, 3),
            moji_size: 25,
            moji_space: (-1, 10),
            moji_color: -1,
            shadow_color: -1,
            fuchi_color: -1,
            ruby_size: 10,
            ruby_space: 1,
            waku_no: 0,
            waku_pos: (0, 0),
            name_disp_mode: 0,
            name_newline: 0,
            name_bracket: 0,
            name_moji_size: 16,
            name_moji_space: (-1, 8),
            name_moji_cnt: 8,
            name_window_pos: (0, -100),
            name_window_size: (300, 100),
            name_msg_pos: (8, 8),
            name_msg_pos_rep: (0, 0),
            name_msg_margin: (8, 8, 8, 8),
            name_extend_type: 0,
            name_window_align: 0,
            name_moji_color: -1,
            name_shadow_color: -1,
            name_fuchi_color: -1,
            name_waku_no: -1,
            face_hide_name: 0,
            talk_margin: (0, 0, 0, 0),
            overflow_check_size: 0,
            msg_back_insert_nl: 0,
            open_anime_type: 0,
            open_anime_time: 0,
            close_anime_type: 0,
            close_anime_time: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MwndRenderTemplate {
    /// #TATEYOKO_MODE: 0 = horizontal, 1 = vertical.
    pub vertical_writing: bool,
    pub default_mwnd_no: i64,
    pub default_sel_mwnd_no: i64,
    pub order: i64,
    pub filter_layer_rep: i64,
    pub waku_layer_rep: i64,
    pub face_layer_rep: i64,
    pub shadow_layer_rep: i64,
    pub fuchi_layer_rep: i64,
    pub moji_layer_rep: i64,
    pub shadow_color: i64,
    pub fuchi_color: i64,
    pub moji_color: i64,
}

impl Default for MwndRenderTemplate {
    fn default() -> Self {
        Self {
            vertical_writing: false,
            default_mwnd_no: 0,
            default_sel_mwnd_no: 1,
            order: 1,
            filter_layer_rep: 0,
            waku_layer_rep: 1,
            face_layer_rep: 2,
            shadow_layer_rep: 3,
            fuchi_layer_rep: 4,
            moji_layer_rep: 5,
            shadow_color: 1,
            fuchi_color: 1,
            moji_color: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmojiTemplate {
    pub file_name: String,
    pub font_size: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageButtonTemplate {
    pub color_no: [i64; TNM_BTN_STATE_MAX],
}

impl Default for MessageButtonTemplate {
    fn default() -> Self {
        let mut color_no = [0; TNM_BTN_STATE_MAX];
        color_no[0] = 0;
        color_no[1] = 2;
        color_no[2] = 2;
        Self { color_no }
    }
}

#[derive(Debug, Clone)]
pub struct IconTemplate {
    pub file_name: String,
    pub anime_pat_cnt: i64,
    pub anime_speed: i64,
}

impl Default for IconTemplate {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            anime_pat_cnt: 1,
            anime_speed: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamaeEntry {
    pub source: String,
    pub display: String,
    pub color_mod: i64,
    pub moji_color_no: i64,
    pub shadow_color_no: i64,
    pub fuchi_color_no: i64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FontConfigDefaults {
    pub font_type: i64,
    pub futoku: i64,
    pub shadow: i64,
}

#[derive(Debug, Clone, Default)]
pub struct WakuButtonTemplate {
    pub file_name: String,
    pub cut_no: i64,
    pub pos_base: i64,
    pub pos: (i64, i64),
    pub action_no: i64,
    pub se_no: i64,
    pub sys_type: i64,
    pub sys_type_opt: i64,
    pub btn_mode: i64,
    pub scn_name: String,
    pub cmd_name: String,
    pub z_no: i64,
    pub frame_action_scn_name: String,
    pub frame_action_cmd_name: String,
}

#[derive(Debug, Clone)]
pub struct WakuTemplate {
    pub extend_type: i64,
    pub waku_file: String,
    pub filter_file: String,
    pub filter_margin: (i64, i64, i64, i64),
    pub filter_color: (u8, u8, u8, u8),
    pub filter_config_color: bool,
    pub filter_config_tr: bool,
    pub icon_no: i64,
    pub page_icon_no: i64,
    pub icon_pos_type: i64,
    pub icon_pos_base: i64,
    pub icon_pos: (i64, i64, i64),
    pub buttons: Vec<WakuButtonTemplate>,
    pub face_pos: Vec<(i64, i64)>,
    pub object_cnt: usize,
}

impl Default for WakuTemplate {
    fn default() -> Self {
        Self {
            extend_type: 0,
            waku_file: String::new(),
            filter_file: String::new(),
            filter_margin: (0, 0, 0, 0),
            filter_color: (0, 0, 255, 128),
            filter_config_color: true,
            filter_config_tr: true,
            icon_no: -1,
            page_icon_no: -1,
            icon_pos_type: 0,
            icon_pos_base: 0,
            icon_pos: (0, 0, 0),
            buttons: vec![WakuButtonTemplate::default(); INIDEF_MWND_WAKU_BTN_CNT],
            face_pos: vec![(0, 0); INIDEF_MWND_WAKU_FACE_CNT],
            object_cnt: INIDEF_MWND_WAKU_OBJECT_CNT,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelBtnTemplate {
    pub base_file: String,
    pub filter_file: String,
    pub base_pos: (i64, i64),
    pub rep_pos: (i64, i64),
    pub x_align: i64,
    pub y_align: i64,
    pub max_y_cnt: i64,
    pub line_width: i64,
    pub moji_cnt: i64,
    pub moji_pos: (i64, i64),
    pub moji_size: i64,
    pub moji_space: (i64, i64),
    pub moji_x_align: i64,
    pub moji_y_align: i64,
    pub moji_color: i64,
    pub moji_hit_color: i64,
    pub btn_action_no: i64,
    pub open_anime_type: i64,
    pub open_anime_time: i64,
    pub close_anime_type: i64,
    pub close_anime_time: i64,
    pub decide_anime_type: i64,
    pub decide_anime_time: i64,
}

impl Default for SelBtnTemplate {
    fn default() -> Self {
        Self {
            base_file: String::new(),
            filter_file: String::new(),
            base_pos: (0, 0),
            rep_pos: (0, 0),
            x_align: 0,
            y_align: 0,
            max_y_cnt: 0,
            line_width: 100,
            moji_cnt: 0,
            moji_pos: (0, 0),
            moji_size: 25,
            moji_space: (0, 0),
            moji_x_align: 0,
            moji_y_align: 0,
            moji_color: 0,
            moji_hit_color: 5,
            btn_action_no: 0,
            open_anime_type: 1,
            open_anime_time: 500,
            close_anime_type: 1,
            close_anime_time: 500,
            decide_anime_type: 1,
            decide_anime_time: 500,
        }
    }
}

#[derive(Debug)]
pub struct AssetTables {
    pub gameexe: Option<GameexeConfig>,
    pub gameexe_report: Option<GameexeDecodeReport>,

    pub button_se_templates: Vec<ButtonSeTemplate>,
    pub button_action_templates: Vec<ButtonActionTemplate>,
    pub se_file_names: Vec<Option<String>>,
    pub mwnd_render: MwndRenderTemplate,
    pub mwnd_templates: Vec<MwndTemplate>,
    pub emoji_templates: Vec<EmojiTemplate>,
    pub message_button_templates: Vec<MessageButtonTemplate>,
    pub waku_templates: Vec<WakuTemplate>,
    pub icon_templates: Vec<IconTemplate>,
    pub sel_btn_templates: Vec<SelBtnTemplate>,
    pub namae_entries: Vec<NamaeEntry>,
    pub color_table: Vec<(u8, u8, u8)>,
    pub font_defaults: FontConfigDefaults,
    /// #SHAKE.n = (x,y,time)(...) tables.  SCREEN.SHAKE receives the table
    /// number, not a duration; this mirrors C_tnm_ini::Sshake.
    pub shake_templates: Vec<Vec<ShakeStep>>,

    pub cgtable: Option<CgTableData>,
    pub cgtable_flag_cnt: Option<usize>,
    /// Original C_tnm_cg_table::flag is a fixed C_elm_int_list: every entry is a signed 32-bit int.
    pub cg_flags: Vec<i32>,

    pub databases: Vec<Option<DbsDatabase>>,

    pub thumb_table: Option<ThumbTable>,
}

impl Default for AssetTables {
    fn default() -> Self {
        Self {
            gameexe: None,
            gameexe_report: None,
            button_se_templates: vec![ButtonSeTemplate::default(); INIDEF_BTN_SE_CNT],
            button_action_templates: vec![ButtonActionTemplate::default(); INIDEF_BTN_ACTION_CNT],
            se_file_names: vec![None; INIDEF_SE_CNT],
            mwnd_render: MwndRenderTemplate::default(),
            mwnd_templates: vec![MwndTemplate::default(); INIDEF_MWND_CNT],
            emoji_templates: vec![EmojiTemplate::default(); 16],
            message_button_templates: vec![MessageButtonTemplate::default(); 16],
            waku_templates: vec![WakuTemplate::default(); INIDEF_WAKU_CNT],
            icon_templates: vec![IconTemplate::default(); INIDEF_ICON_CNT],
            sel_btn_templates: vec![SelBtnTemplate::default(); INIDEF_SEL_BTN_CNT],
            namae_entries: Vec::new(),
            color_table: default_color_table(),
            font_defaults: FontConfigDefaults::default(),
            shake_templates: vec![Vec::new(); INIDEF_SHAKE_CNT],
            cgtable: None,
            cgtable_flag_cnt: None,
            cg_flags: Vec::new(),
            databases: Vec::new(),
            thumb_table: None,
        }
    }
}

impl AssetTables {
    pub fn load(project_dir: &Path, unknown: &mut UnknownOpRecorder) -> Self {
        let mut out = Self::default();

        let gameexe_path = match crate::resource::find_initial_gameexe_path(project_dir) {
            Ok(path) => path,
            Err(_) => {
                unknown.record_note("gameexe.missing");
                return out;
            }
        };

        let raw = match crate::resource::read_file_bytes(&gameexe_path) {
            Ok(b) => b,
            Err(e) => {
                unknown.record_note(&format!("gameexe.read.failed:{e}"));
                return out;
            }
        };

        let (text, report): (String, Option<GameexeDecodeReport>) = if gameexe_path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("ini"))
        {
            match String::from_utf8(raw) {
                Ok(text) => (text, None),
                Err(e) => {
                    unknown.record_note(&format!("gameexe.ini.decode.failed:{e}"));
                    return out;
                }
            }
        } else {
            let opt = match load_gameexe_decode_options(project_dir) {
                Ok(v) => v,
                Err(e) => {
                    // Keep going with defaults.
                    unknown.record_note(&format!("gameexe.key_toml.invalid:{e}"));
                    GameexeDecodeOptions::default()
                }
            };

            match decode_gameexe_dat_bytes(&raw, &opt) {
                Ok((text, report)) => (text, Some(report)),
                Err(e) => {
                    unknown.record_note(&format!("gameexe.decode.failed:{e}"));
                    return out;
                }
            }
        };

        let cfg = GameexeConfig::from_text(&text);
        out.gameexe_report = report;
        out.button_se_templates = load_button_se_templates(&cfg);
        out.button_action_templates = load_button_action_templates(&cfg);
        out.se_file_names = load_se_file_names(&cfg);
        out.mwnd_render = load_mwnd_render_template(&cfg);
        out.mwnd_templates = load_mwnd_templates(&cfg);
        out.emoji_templates = load_emoji_templates(&cfg);
        out.message_button_templates = load_message_button_templates(&cfg);
        out.waku_templates = load_waku_templates(&cfg, Some(&text));
        out.icon_templates = load_icon_templates(&cfg);
        out.sel_btn_templates = load_sel_btn_templates(&cfg);
        out.namae_entries = load_namae_entries(Some(&text));
        out.color_table = load_color_table(&cfg);
        out.font_defaults = load_font_config_defaults(&cfg);
        out.shake_templates = load_shake_templates(&cfg);
        out.gameexe = Some(cfg);

        // Drive table loading from the parsed config.
        let dat_dir = project_dir.join("dat");
        if !path_is_dir(&dat_dir) {
            unknown.record_note("dat.dir.missing");
        }

        if let Some(cfg) = out.gameexe.as_ref() {
            // CGTABLE
            if let Some(v) = cfg.get_unquoted("CGTABLE_FILE") {
                if let Some(path) = resolve_table_path(project_dir, &dat_dir, v, Some("cgm")) {
                    match crate::resource::read_file_bytes(&path)
                        .and_then(|bytes| CgTableData::from_bytes(&bytes))
                    {
                        Ok(t) => out.cgtable = Some(t),
                        Err(e) => unknown.record_note(&format!("cgtable.load.failed:{path:?}:{e}")),
                    }
                } else {
                    unknown.record_note(&format!("cgtable.path.missing:{v}"));
                }
            }

            // CGTABLE_FLAG_CNT
            if let Some(n) = cfg.get_usize("CGTABLE_FLAG_CNT") {
                out.cgtable_flag_cnt = Some(n);
                out.cg_flags = vec![0i32; n.max(32)];
            }

            // THUMBTABLE
            if let Some(v) = cfg.get_unquoted("THUMBTABLE_FILE") {
                if let Some(path) = resolve_table_path(project_dir, &dat_dir, v, Some("dat")) {
                    match crate::resource::read_file_bytes(&path)
                        .and_then(|bytes| ThumbTable::from_bytes(&bytes))
                    {
                        Ok(t) => out.thumb_table = Some(t),
                        Err(e) => {
                            unknown.record_note(&format!("thumb_table.load.failed:{path:?}:{e}"))
                        }
                    }
                } else {
                    unknown.record_note(&format!("thumb_table.path.missing:{v}"));
                }
            }

            // DATABASE
            let db_cnt = cfg.indexed_count("DATABASE");
            // The original C_elm_database_list owns a fixed DATABASE[i] slot
            // for every configured index. A missing/unreadable .dbs leaves
            // that slot uninitialized; it must not shift following indices.
            out.databases = vec![None; db_cnt];
            let database_append = crate::resource::initial_select_ini_append(project_dir);
            for i in 0..db_cnt {
                let key = format!("DATABASE.{i}");
                let Some(name) = cfg
                    .get_indexed_unquoted("DATABASE", i)
                    .or_else(|| cfg.get_indexed_item_unquoted("DATABASE", i, 0))
                    .or_else(|| cfg.get_indexed_field_unquoted("DATABASE", i, "FILE"))
                    .or_else(|| cfg.get_indexed_field_unquoted("DATABASE", i, "NAME"))
                else {
                    unknown.record_note(&format!("database.name.missing:{key}"));
                    continue;
                };
                // Original C_elm_database::load_dbs() does not use a generic
                // project/dat lookup. It appends ".dbs" to the configured
                // DATABASE name and resolves the file through tnm_find_dat(),
                // which searches dat/ from the current Select.ini append to
                // later append entries in order.
                let dbs_file_name = format!("{name}.dbs");
                let path = match crate::resource::resolve_dat_file_path(
                    project_dir,
                    &database_append.dir,
                    &dbs_file_name,
                ) {
                    Ok(Some(path)) => path,
                    Ok(None) => {
                        unknown
                            .record_note(&format!("database.path.missing:{key}:{dbs_file_name}"));
                        continue;
                    }
                    Err(e) => {
                        unknown.record_note(&format!(
                            "database.path.resolve.failed:{key}:{dbs_file_name}:{e}"
                        ));
                        continue;
                    }
                };
                match crate::resource::read_file_bytes(&path)
                    .and_then(|bytes| DbsDatabase::from_bytes(&bytes))
                {
                    Ok(db) => out.databases[i] = Some(db),
                    Err(e) => unknown.record_note(&format!("dbs.load.failed:{path:?}:{e}")),
                }
            }
        }

        out
    }
}

fn parse_i64_tuple(raw: Option<&str>) -> Vec<i64> {
    raw.map(parse_button_action_numbers).unwrap_or_default()
}

fn load_mwnd_render_template(cfg: &GameexeConfig) -> MwndRenderTemplate {
    let mut t = MwndRenderTemplate {
        vertical_writing: cfg
            .get_unquoted("TATEYOKO_MODE")
            .and_then(parse_i64_like_local)
            .unwrap_or(0)
            == 1,
        ..Default::default()
    };

    if let Some(v) = cfg
        .get_unquoted("MWND.DEFAULT_MWND_NO")
        .and_then(parse_i64_like_local)
    {
        t.default_mwnd_no = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.DEFAULT_SEL_MWND_NO")
        .and_then(parse_i64_like_local)
    {
        t.default_sel_mwnd_no = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.ORDER")
        .and_then(parse_i64_like_local)
    {
        t.order = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.FILTER_LAYER_REP")
        .and_then(parse_i64_like_local)
    {
        t.filter_layer_rep = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.WAKU_LAYER_REP")
        .and_then(parse_i64_like_local)
    {
        t.waku_layer_rep = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.FACE_LAYER_REP")
        .and_then(parse_i64_like_local)
    {
        t.face_layer_rep = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.SHADOW_LAYER_REP")
        .and_then(parse_i64_like_local)
    {
        t.shadow_layer_rep = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.FUCHI_LAYER_REP")
        .and_then(parse_i64_like_local)
    {
        t.fuchi_layer_rep = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.MOJI_LAYER_REP")
        .and_then(parse_i64_like_local)
    {
        t.moji_layer_rep = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.SHADOW_COLOR")
        .and_then(parse_i64_like_local)
    {
        t.shadow_color = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.FUCHI_COLOR")
        .and_then(parse_i64_like_local)
    {
        t.fuchi_color = v;
    }
    if let Some(v) = cfg
        .get_unquoted("MWND.MOJI_COLOR")
        .and_then(parse_i64_like_local)
    {
        t.moji_color = v;
    }
    t
}

fn load_mwnd_templates(cfg: &GameexeConfig) -> Vec<MwndTemplate> {
    let cnt = cfg
        .get_usize("MWND.CNT")
        .unwrap_or(INIDEF_MWND_CNT)
        .min(INIMAX_MWND_CNT);
    let mut out = vec![MwndTemplate::default(); cnt];

    for i in 0..cnt {
        let mut t = MwndTemplate::default();
        let get_i64 = |field: &str| {
            cfg.get_indexed_field("MWND", i, field)
                .and_then(parse_i64_like_local)
        };
        let get_tuple = |field: &str| parse_i64_tuple(cfg.get_indexed_field("MWND", i, field));

        if let Some(v) = get_i64("NOVEL_MODE") {
            t.novel_mode = v;
        }
        if let Some(v) = get_i64("EXTEND_TYPE") {
            t.extend_type = v;
        }
        let window_pos = get_tuple("WINDOW_POS");
        if window_pos.len() >= 2 {
            t.window_pos = (window_pos[0], window_pos[1]);
        }
        let window_size = get_tuple("WINDOW_SIZE");
        if window_size.len() >= 2 {
            t.window_size = (window_size[0], window_size[1]);
        }
        let message_pos = get_tuple("MESSAGE_POS");
        if message_pos.len() >= 2 {
            t.message_pos = (message_pos[0], message_pos[1]);
        }
        let message_margin = get_tuple("MESSAGE_MARGIN");
        if message_margin.len() >= 4 {
            t.message_margin = (
                message_margin[0],
                message_margin[1],
                message_margin[2],
                message_margin[3],
            );
        }
        let moji_cnt = get_tuple("MOJI_CNT");
        if moji_cnt.len() >= 2 {
            t.moji_cnt = (moji_cnt[0], moji_cnt[1]);
        }
        if let Some(v) = get_i64("MOJI_SIZE") {
            t.moji_size = v;
        }
        let moji_space = get_tuple("MOJI_SPACE");
        if moji_space.len() >= 2 {
            t.moji_space = (moji_space[0], moji_space[1]);
        }
        if let Some(v) = get_i64("MOJI_COLOR") {
            t.moji_color = v;
        }
        if let Some(v) = get_i64("SHADOW_COLOR") {
            t.shadow_color = v;
        }
        if let Some(v) = get_i64("FUCHI_COLOR") {
            t.fuchi_color = v;
        }
        if let Some(v) = get_i64("RUBY_SIZE") {
            t.ruby_size = v;
        }
        if let Some(v) = get_i64("RUBY_SPACE") {
            t.ruby_space = v;
        }
        if let Some(v) = get_i64("WAKU_NO") {
            t.waku_no = v;
        }
        let waku_pos = get_tuple("WAKU_POS");
        if waku_pos.len() >= 2 {
            t.waku_pos = (waku_pos[0], waku_pos[1]);
        }
        if let Some(v) = get_i64("NAME_DISP_MODE") {
            t.name_disp_mode = v;
        }
        if let Some(v) = get_i64("NAME_NEWLINE") {
            t.name_newline = v;
        }
        if let Some(v) = get_i64("NAME_BRACKET") {
            t.name_bracket = v;
        }
        if let Some(v) = get_i64("NAME_MOJI_SIZE") {
            t.name_moji_size = v;
        }
        let name_moji_space = get_tuple("NAME_MOJI_SPACE");
        if name_moji_space.len() >= 2 {
            t.name_moji_space = (name_moji_space[0], name_moji_space[1]);
        }
        if let Some(v) = get_i64("NAME_MOJI_CNT") {
            t.name_moji_cnt = v;
        }
        let name_window_pos = get_tuple("NAME_WINDOW_POS");
        if name_window_pos.len() >= 2 {
            t.name_window_pos = (name_window_pos[0], name_window_pos[1]);
        }
        let name_window_size = get_tuple("NAME_WINDOW_SIZE");
        if name_window_size.len() >= 2 {
            t.name_window_size = (name_window_size[0], name_window_size[1]);
        }
        let name_msg_pos = get_tuple("NAME_MESSAGE_POS");
        if name_msg_pos.len() >= 2 {
            t.name_msg_pos = (name_msg_pos[0], name_msg_pos[1]);
        }
        let name_msg_pos_rep = get_tuple("NAME_MESSAGE_POS_REP");
        if name_msg_pos_rep.len() >= 2 {
            t.name_msg_pos_rep = (name_msg_pos_rep[0], name_msg_pos_rep[1]);
        }
        let name_msg_margin = get_tuple("NAME_MESSAGE_MARGIN");
        if name_msg_margin.len() >= 4 {
            t.name_msg_margin = (
                name_msg_margin[0],
                name_msg_margin[1],
                name_msg_margin[2],
                name_msg_margin[3],
            );
        }
        if let Some(v) = get_i64("NAME_MOJI_COLOR") {
            t.name_moji_color = v;
        }
        if let Some(v) = get_i64("NAME_SHADOW_COLOR") {
            t.name_shadow_color = v;
        }
        if let Some(v) = get_i64("NAME_FUCHI_COLOR") {
            t.name_fuchi_color = v;
        }
        if let Some(v) = get_i64("NAME_WAKU_NO") {
            t.name_waku_no = v;
        }
        if let Some(v) = get_i64("NAME_EXTEND_TYPE") {
            t.name_extend_type = v;
        }
        if let Some(v) = get_i64("NAME_WINDOW_ALIGN") {
            t.name_window_align = v;
        }
        if let Some(v) = get_i64("FACE_HIDE_NAME") {
            t.face_hide_name = v;
        }
        let talk_margin = get_tuple("TALK_MARGIN");
        if talk_margin.len() >= 4 {
            t.talk_margin = (
                talk_margin[0],
                talk_margin[1],
                talk_margin[2],
                talk_margin[3],
            );
        }
        if let Some(v) = get_i64("OVERFLOW_CHECK_SIZE") {
            t.overflow_check_size = v;
        }
        if let Some(v) = get_i64("MSG_BACK_INSERT_NL") {
            t.msg_back_insert_nl = v;
        }
        if let Some(v) = get_i64("OPEN_ANIME_TYPE") {
            t.open_anime_type = v;
        }
        if let Some(v) = get_i64("OPEN_ANIME_TIME") {
            t.open_anime_time = v;
        }
        if let Some(v) = get_i64("CLOSE_ANIME_TYPE") {
            t.close_anime_type = v;
        }
        if let Some(v) = get_i64("CLOSE_ANIME_TIME") {
            t.close_anime_time = v;
        }
        out[i] = t;
    }

    out
}

fn nested_indexed_field<'a>(
    cfg: &'a GameexeConfig,
    prefix: &str,
    index: usize,
    nested: &str,
    nested_index: usize,
    field: &str,
) -> Option<&'a str> {
    // Original Gameexe syntax accepts numeric fields, and shipped Gameexe files
    // conventionally write both levels as zero-padded indices, for example:
    //   #WAKU.000.BTN.000.FILE = "_mbtn00"
    // GameexeConfig::get_indexed_field handles the first indexed component, but it
    // does not reinterpret an already-flattened nested component such as
    // "BTN.0.FILE" as "BTN.000.FILE". Try the exact C++ textual forms for both
    // index levels before falling back to the generic indexed helper.
    for direct in [
        format!("{prefix}.{index}.{nested}.{nested_index}.{field}"),
        format!("{prefix}.{index:03}.{nested}.{nested_index}.{field}"),
        format!("{prefix}.{index}.{nested}.{nested_index:03}.{field}"),
        format!("{prefix}.{index:03}.{nested}.{nested_index:03}.{field}"),
    ] {
        if let Some(v) = cfg.get_value(&direct) {
            return Some(v);
        }
    }

    for nested_prefix in [
        format!("{prefix}.{index}.{nested}"),
        format!("{prefix}.{index:03}.{nested}"),
    ] {
        if let Some(v) = cfg.get_indexed_field(&nested_prefix, nested_index, field) {
            return Some(v);
        }
    }

    for flat_field in [
        format!("{nested}.{nested_index}.{field}"),
        format!("{nested}.{nested_index:03}.{field}"),
    ] {
        if let Some(v) = cfg.get_indexed_field(prefix, index, &flat_field) {
            return Some(v);
        }
    }
    None
}

fn nested_indexed_field_unquoted<'a>(
    cfg: &'a GameexeConfig,
    prefix: &str,
    index: usize,
    nested: &str,
    nested_index: usize,
    field: &str,
) -> Option<&'a str> {
    // See nested_indexed_field(). Keep the unquoted path in lockstep so FILE,
    // TYPE, CALL, and FRAME_ACTION use the same original Gameexe addressing.
    for direct in [
        format!("{prefix}.{index}.{nested}.{nested_index}.{field}"),
        format!("{prefix}.{index:03}.{nested}.{nested_index}.{field}"),
        format!("{prefix}.{index}.{nested}.{nested_index:03}.{field}"),
        format!("{prefix}.{index:03}.{nested}.{nested_index:03}.{field}"),
    ] {
        if let Some(v) = cfg.get_unquoted(&direct) {
            return Some(v);
        }
    }

    for nested_prefix in [
        format!("{prefix}.{index}.{nested}"),
        format!("{prefix}.{index:03}.{nested}"),
    ] {
        if let Some(v) = cfg.get_indexed_field_unquoted(&nested_prefix, nested_index, field) {
            return Some(v);
        }
    }

    for flat_field in [
        format!("{nested}.{nested_index}.{field}"),
        format!("{nested}.{nested_index:03}.{field}"),
    ] {
        if let Some(v) = cfg.get_indexed_field_unquoted(prefix, index, &flat_field) {
            return Some(v);
        }
    }
    None
}

type RawGameexeFields<'a> = HashMap<String, &'a str>;

fn canonical_raw_gameexe_key(raw: &str) -> String {
    normalize_gameexe_key(raw)
        .split('.')
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<usize>()
                .map(|value| value.to_string())
                .unwrap_or_else(|_| part.to_string())
        })
        .collect::<Vec<_>>()
        .join(".")
}

fn index_raw_gameexe_fields(raw_text: Option<&str>) -> RawGameexeFields<'_> {
    let mut fields = HashMap::new();
    for line in raw_text.unwrap_or_default().lines() {
        let mut s = line.trim();
        if s.is_empty() {
            continue;
        }
        if let Some(rest) = s.strip_prefix('\u{feff}') {
            s = rest.trim_start();
        }
        if let Some(rest) = s.strip_prefix('#') {
            s = rest.trim_start();
        }
        let Some((lhs, rhs)) = s.split_once('=') else {
            continue;
        };
        let value = rhs.trim().trim_end_matches(';').trim();
        // C_tnm_ini::analize() processes Gameexe linearly and assigns directly
        // into the destination field, so a later duplicate definition wins.
        // Use insert (not or_insert) to retain that original behavior.
        fields.insert(canonical_raw_gameexe_key(lhs), value);
    }
    fields
}

fn raw_gameexe_field(fields: &RawGameexeFields<'_>, key: &str) -> Option<String> {
    fields
        .get(&canonical_raw_gameexe_key(key))
        .map(|value| (*value).to_string())
}

fn raw_nested_indexed_field(
    fields: &RawGameexeFields<'_>,
    prefix: &str,
    index: usize,
    nested: &str,
    nested_index: usize,
    field: &str,
) -> Option<String> {
    for key in [
        format!("{prefix}.{index}.{nested}.{nested_index}.{field}"),
        format!("{prefix}.{index:03}.{nested}.{nested_index}.{field}"),
        format!("{prefix}.{index}.{nested}.{nested_index:03}.{field}"),
        format!("{prefix}.{index:03}.{nested}.{nested_index:03}.{field}"),
    ] {
        if let Some(v) = raw_gameexe_field(fields, &key) {
            return Some(v);
        }
    }
    None
}

fn raw_indexed_field(
    fields: &RawGameexeFields<'_>,
    prefix: &str,
    index: usize,
    field: &str,
) -> Option<String> {
    for key in [
        format!("{prefix}.{index}.{field}"),
        format!("{prefix}.{index:03}.{field}"),
    ] {
        if let Some(v) = raw_gameexe_field(fields, &key) {
            return Some(v);
        }
    }
    None
}

fn trim_gameexe_scalar(raw: &str) -> &str {
    raw.trim().trim_matches('"')
}

fn parse_waku_button_type(raw: &str, button: &mut WakuButtonTemplate) {
    let parts: Vec<&str> = raw.split(',').map(|p| p.trim().trim_matches('"')).collect();
    if parts.is_empty() {
        return;
    }
    let ty = parts[0].to_ascii_lowercase();
    let n0 = parts
        .get(1)
        .and_then(|v| parse_i64_like_local(v))
        .unwrap_or(0);
    let n1 = parts
        .get(2)
        .and_then(|v| parse_i64_like_local(v))
        .unwrap_or(0);
    match ty.as_str() {
        "none" => button.sys_type = 0,
        "save" => button.sys_type = 1,
        "load" => button.sys_type = 2,
        "read_skip" => {
            button.sys_type = 3;
            button.btn_mode = n0;
        }
        "auto_mode" => {
            button.sys_type = 4;
            button.btn_mode = n0;
        }
        "return_sel" => button.sys_type = 5,
        "close_mwnd" => button.sys_type = 6,
        "msg_log" => button.sys_type = 7,
        "koe_play" => button.sys_type = 8,
        "qsave" => {
            button.sys_type = 9;
            button.sys_type_opt = n0;
        }
        "qload" => {
            button.sys_type = 10;
            button.sys_type_opt = n0;
        }
        "config" => button.sys_type = 11,
        "local_switch" => {
            button.sys_type = 12;
            button.sys_type_opt = n0;
            button.btn_mode = n1;
        }
        "local_mode" => {
            button.sys_type = 13;
            button.sys_type_opt = n0;
            button.btn_mode = n1;
        }
        "global_switch" => {
            button.sys_type = 14;
            button.sys_type_opt = n0;
            button.btn_mode = n1;
        }
        "global_mode" => {
            button.sys_type = 15;
            button.sys_type_opt = n0;
            button.btn_mode = n1;
        }
        _ => {}
    }
}

fn load_waku_templates(cfg: &GameexeConfig, raw_text: Option<&str>) -> Vec<WakuTemplate> {
    // The original parser consumes Gameexe once into C_tnm_ini. Mirror that
    // architecture instead of rescanning the full text for every WAKU field.
    let raw_fields = index_raw_gameexe_fields(raw_text);
    let cnt = cfg
        .get_usize("WAKU.CNT")
        .unwrap_or(INIDEF_WAKU_CNT)
        .min(INIMAX_WAKU_CNT);
    let btn_cnt = cfg
        .get_usize("WAKU.BTN.CNT")
        .unwrap_or(INIDEF_MWND_WAKU_BTN_CNT)
        .min(INIMAX_MWND_WAKU_BTN_CNT);
    let face_cnt = cfg
        .get_usize("WAKU.FACE.CNT")
        .unwrap_or(INIDEF_MWND_WAKU_FACE_CNT)
        .min(INIMAX_MWND_WAKU_FACE_CNT);
    let object_cnt = cfg
        .get_usize("WAKU.OBJECT.CNT")
        .unwrap_or(INIDEF_MWND_WAKU_OBJECT_CNT)
        .min(INIMAX_MWND_WAKU_OBJECT_CNT);
    let mut out = vec![WakuTemplate::default(); cnt];

    for i in 0..cnt {
        let mut t = WakuTemplate {
            buttons: vec![WakuButtonTemplate::default(); btn_cnt],
            face_pos: vec![(0, 0); face_cnt],
            object_cnt,
            ..Default::default()
        };

        let raw_top = |field: &str| raw_indexed_field(&raw_fields, "WAKU", i, field);

        let extend_type_raw = raw_top("EXTEND_TYPE");
        if let Some(v) = extend_type_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "EXTEND_TYPE"))
            .and_then(parse_i64_like_local)
        {
            t.extend_type = v;
        }
        let waku_file_raw = raw_top("WAKU_FILE");
        if let Some(v) = waku_file_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field_unquoted("WAKU", i, "WAKU_FILE"))
        {
            t.waku_file = trim_gameexe_scalar(v).to_string();
        }
        let filter_file_raw = raw_top("FILTER_FILE");
        if let Some(v) = filter_file_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field_unquoted("WAKU", i, "FILTER_FILE"))
        {
            t.filter_file = trim_gameexe_scalar(v).to_string();
        }
        let filter_margin_raw = raw_top("FILTER_MARGIN");
        let filter_margin = parse_i64_tuple(
            filter_margin_raw
                .as_deref()
                .or_else(|| cfg.get_indexed_field("WAKU", i, "FILTER_MARGIN")),
        );
        if filter_margin.len() >= 4 {
            t.filter_margin = (
                filter_margin[0],
                filter_margin[1],
                filter_margin[2],
                filter_margin[3],
            );
        }
        let filter_color_raw = raw_top("FILTER_COLOR");
        let filter_color = parse_i64_tuple(
            filter_color_raw
                .as_deref()
                .or_else(|| cfg.get_indexed_field("WAKU", i, "FILTER_COLOR")),
        );
        if filter_color.len() >= 4 {
            t.filter_color = (
                filter_color[0].clamp(0, 255) as u8,
                filter_color[1].clamp(0, 255) as u8,
                filter_color[2].clamp(0, 255) as u8,
                filter_color[3].clamp(0, 255) as u8,
            );
        }
        let filter_config_color_raw = raw_top("FILTER_CONFIG_COLOR");
        if let Some(v) = filter_config_color_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "FILTER_CONFIG_COLOR"))
            .and_then(parse_i64_like_local)
        {
            t.filter_config_color = v != 0;
        }
        let filter_config_tr_raw = raw_top("FILTER_CONFIG_TR");
        if let Some(v) = filter_config_tr_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "FILTER_CONFIG_TR"))
            .and_then(parse_i64_like_local)
        {
            t.filter_config_tr = v != 0;
        }
        let icon_no_raw = raw_top("ICON_NO");
        if let Some(v) = icon_no_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "ICON_NO"))
            .and_then(parse_i64_like_local)
        {
            t.icon_no = v;
        }
        let page_icon_no_raw = raw_top("PAGE_ICON_NO");
        if let Some(v) = page_icon_no_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "PAGE_ICON_NO"))
            .and_then(parse_i64_like_local)
        {
            t.page_icon_no = v;
        }
        let icon_pos_type_raw = raw_top("ICON_POS_TYPE");
        if let Some(v) = icon_pos_type_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "ICON_POS_TYPE"))
            .and_then(parse_i64_like_local)
        {
            t.icon_pos_type = v;
        }
        let icon_pos_base_raw = raw_top("ICON_POS_BASE");
        if let Some(v) = icon_pos_base_raw
            .as_deref()
            .or_else(|| cfg.get_indexed_field("WAKU", i, "ICON_POS_BASE"))
            .and_then(parse_i64_like_local)
        {
            t.icon_pos_base = v;
        }
        let icon_pos_raw = raw_top("ICON_POS");
        let icon_pos = parse_i64_tuple(
            icon_pos_raw
                .as_deref()
                .or_else(|| cfg.get_indexed_field("WAKU", i, "ICON_POS")),
        );
        if icon_pos.len() >= 3 {
            // Original tnm_ini parses ICON_POS as: base, x, y.
            // Keep base separate and store only the point coordinates in icon_pos.
            t.icon_pos_base = icon_pos[0];
            t.icon_pos = (icon_pos[1], icon_pos[2], 0);
        }

        for btn_idx in 0..t.buttons.len() {
            let mut b = WakuButtonTemplate::default();

            let file_raw = raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "FILE");
            if let Some(v) = file_raw
                .as_deref()
                .or_else(|| nested_indexed_field_unquoted(cfg, "WAKU", i, "BTN", btn_idx, "FILE"))
            {
                b.file_name = trim_gameexe_scalar(v).to_string();
            }

            let cut_raw =
                raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "CUT_NO");
            if let Some(v) = cut_raw
                .as_deref()
                .or_else(|| nested_indexed_field(cfg, "WAKU", i, "BTN", btn_idx, "CUT_NO"))
                .and_then(parse_i64_like_local)
            {
                b.cut_no = v;
            }

            let pos_raw = raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "POS");
            let pos = parse_i64_tuple(
                pos_raw
                    .as_deref()
                    .or_else(|| nested_indexed_field(cfg, "WAKU", i, "BTN", btn_idx, "POS")),
            );
            if pos.len() >= 3 {
                // Original C_tnm_ini::analize_step_waku() parses:
                // WAKU.n.BTN.i.POS = pos_base, x, y
                b.pos_base = pos[0];
                b.pos = (pos[1], pos[2]);
            }

            let action_raw =
                raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "ACTION");
            if let Some(v) = action_raw
                .as_deref()
                .or_else(|| nested_indexed_field(cfg, "WAKU", i, "BTN", btn_idx, "ACTION"))
                .and_then(parse_i64_like_local)
            {
                b.action_no = v;
            }

            let se_raw = raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "SE");
            if let Some(v) = se_raw
                .as_deref()
                .or_else(|| nested_indexed_field(cfg, "WAKU", i, "BTN", btn_idx, "SE"))
                .and_then(parse_i64_like_local)
            {
                b.se_no = v;
            }

            let type_raw = raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "TYPE");
            if let Some(v) = type_raw
                .as_deref()
                .or_else(|| nested_indexed_field_unquoted(cfg, "WAKU", i, "BTN", btn_idx, "TYPE"))
            {
                parse_waku_button_type(v, &mut b);
            }

            let call_raw = raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "CALL");
            if let Some(v) = call_raw
                .as_deref()
                .or_else(|| nested_indexed_field_unquoted(cfg, "WAKU", i, "BTN", btn_idx, "CALL"))
            {
                let parts: Vec<&str> = v.split(',').map(|p| p.trim().trim_matches('"')).collect();
                if parts.len() >= 2 {
                    b.scn_name = parts[0].to_string();
                    if let Some(z) = parse_i64_like_local(parts[1]) {
                        b.z_no = z;
                    } else {
                        b.cmd_name = parts[1].to_string();
                    }
                }
            }

            let frame_action_raw =
                raw_nested_indexed_field(&raw_fields, "WAKU", i, "BTN", btn_idx, "FRAME_ACTION");
            if let Some(v) = frame_action_raw.as_deref().or_else(|| {
                nested_indexed_field_unquoted(cfg, "WAKU", i, "BTN", btn_idx, "FRAME_ACTION")
            }) {
                let parts: Vec<&str> = v.split(',').map(|p| p.trim().trim_matches('"')).collect();
                if parts.len() >= 2 {
                    b.frame_action_scn_name = parts[0].to_string();
                    b.frame_action_cmd_name = parts[1].to_string();
                }
            }

            if !b.file_name.is_empty()
                || b.cut_no != 0
                || b.pos != (0, 0)
                || b.action_no != 0
                || b.se_no != 0
            {
                t.buttons[btn_idx] = b;
            }
        }

        for face_idx in 0..t.face_pos.len() {
            let pos_raw = raw_nested_indexed_field(&raw_fields, "WAKU", i, "FACE", face_idx, "POS");
            let pos = parse_i64_tuple(
                pos_raw
                    .as_deref()
                    .or_else(|| nested_indexed_field(cfg, "WAKU", i, "FACE", face_idx, "POS")),
            );
            if pos.len() >= 2 {
                t.face_pos[face_idx] = (pos[0], pos[1]);
            }
        }

        out[i] = t;
    }

    out
}

fn default_color_table() -> Vec<(u8, u8, u8)> {
    let mut out = vec![(255, 255, 255); 256];
    out[0] = (255, 255, 255);
    out[1] = (0, 0, 0);
    out[2] = (255, 0, 0);
    out[3] = (0, 255, 0);
    out[4] = (0, 0, 255);
    out[5] = (255, 255, 0);
    out[6] = (255, 0, 255);
    out[7] = (0, 255, 255);
    out
}

fn load_color_table(cfg: &GameexeConfig) -> Vec<(u8, u8, u8)> {
    let cnt = cfg
        .get_usize("COLOR_TABLE.CNT")
        .unwrap_or(256)
        .clamp(1, 4096);
    let mut out = default_color_table();
    if out.len() < cnt {
        out.resize(cnt, (255, 255, 255));
    }

    for i in 0..cnt {
        let Some(raw) = cfg
            .get_indexed_value("COLOR_TABLE", i)
            .or_else(|| cfg.get_indexed_field("COLOR_TABLE", i, "RGB"))
            .or_else(|| cfg.get_indexed_field("COLOR_TABLE", i, "COLOR"))
        else {
            continue;
        };
        let vals = parse_i64_tuple(Some(raw));
        if vals.len() >= 3 {
            out[i] = (
                vals[0].clamp(0, 255) as u8,
                vals[1].clamp(0, 255) as u8,
                vals[2].clamp(0, 255) as u8,
            );
        }
    }
    out
}

fn load_font_config_defaults(cfg: &GameexeConfig) -> FontConfigDefaults {
    FontConfigDefaults {
        font_type: cfg
            .get_unquoted("CONFIG.FONT.TYPE")
            .and_then(parse_i64_like_local)
            .unwrap_or(0),
        futoku: cfg
            .get_unquoted("CONFIG.FONT.FUTOKU")
            .and_then(parse_i64_like_local)
            .unwrap_or(0),
        shadow: cfg
            .get_unquoted("CONFIG.FONT.SHADOW")
            .and_then(parse_i64_like_local)
            .unwrap_or(0),
    }
}

fn load_namae_entries(raw_text: Option<&str>) -> Vec<NamaeEntry> {
    let Some(text) = raw_text else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let mut t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(rest) = t.strip_prefix('#') {
            t = rest.trim_start();
        }
        let Some(rhs) = t.strip_prefix("NAMAE") else {
            continue;
        };
        let Some(rhs) = rhs.trim_start().strip_prefix('=') else {
            continue;
        };
        let fields = split_gameexe_fields(rhs);
        if fields.len() < 5 {
            continue;
        }
        let source = trim_gameexe_scalar(&fields[0]).to_string();
        let display = trim_gameexe_scalar(&fields[1]).to_string();
        if source.is_empty() {
            continue;
        }
        let color_mod = fields
            .get(2)
            .and_then(|v| parse_i64_like_local(v))
            .unwrap_or(0);
        let moji_color_no = fields
            .get(3)
            .and_then(|v| parse_i64_like_local(v))
            .unwrap_or(-1);
        let shadow_color_no = fields
            .get(4)
            .and_then(|v| parse_i64_like_local(v))
            .unwrap_or(-1);
        let fuchi_color_no = fields
            .get(5)
            .and_then(|v| parse_i64_like_local(v))
            .unwrap_or(-1);
        out.push(NamaeEntry {
            source,
            display,
            color_mod,
            moji_color_no,
            shadow_color_no,
            fuchi_color_no,
        });
    }
    out
}

fn split_gameexe_fields(raw: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    let mut paren_depth = 0i32;
    let mut chars = raw.chars().peekable();
    for ch in chars {
        match ch {
            '"' => {
                in_quote = !in_quote;
                cur.push(ch);
            }
            '(' if !in_quote => {
                paren_depth += 1;
                cur.push(ch);
            }
            ')' if !in_quote => {
                paren_depth = (paren_depth - 1).max(0);
                cur.push(ch);
            }
            ',' if !in_quote && paren_depth == 0 => {
                fields.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        fields.push(cur.trim().to_string());
    }
    fields
}

fn load_icon_templates(cfg: &GameexeConfig) -> Vec<IconTemplate> {
    let cnt = cfg
        .get_usize("ICON.CNT")
        .unwrap_or(INIDEF_ICON_CNT)
        .min(INIMAX_ICON_CNT);
    let mut out = vec![IconTemplate::default(); cnt];

    for i in 0..cnt {
        let mut t = IconTemplate::default();
        if let Some(v) = cfg
            .get_indexed_field_unquoted("ICON", i, "FILE_NAME")
            .or_else(|| cfg.get_indexed_field_unquoted("ICON", i, "FILE"))
            .or_else(|| cfg.get_indexed_unquoted("ICON", i))
        {
            t.file_name = v.to_string();
        }
        if let Some(v) = cfg
            .get_indexed_field("ICON", i, "ANIME_PAT_CNT")
            .or_else(|| cfg.get_indexed_field("ICON", i, "CNT"))
            .or_else(|| cfg.get_indexed_field("ICON", i, "PAT_CNT"))
            .and_then(parse_i64_like_local)
        {
            t.anime_pat_cnt = v.max(1);
        }
        if let Some(v) = cfg
            .get_indexed_field("ICON", i, "ANIME_SPEED")
            .or_else(|| cfg.get_indexed_field("ICON", i, "SPEED"))
            .and_then(parse_i64_like_local)
        {
            t.anime_speed = v.max(1);
        }
        out[i] = t;
    }

    out
}

fn load_sel_btn_templates(cfg: &GameexeConfig) -> Vec<SelBtnTemplate> {
    let cnt = cfg
        .get_usize("SELBTN.CNT")
        .unwrap_or(INIDEF_SEL_BTN_CNT)
        .clamp(INIMIN_SEL_BTN_CNT, INIMAX_SEL_BTN_CNT);
    let mut out = vec![SelBtnTemplate::default(); cnt];
    for i in 0..cnt {
        let mut t = SelBtnTemplate::default();
        let get_i64 = |field: &str| {
            cfg.get_indexed_field("SELBTN", i, field)
                .and_then(parse_i64_like_local)
        };
        let get_tuple = |field: &str| parse_i64_tuple(cfg.get_indexed_field("SELBTN", i, field));

        if let Some(v) = cfg.get_indexed_field_unquoted("SELBTN", i, "BASE_FILE") {
            t.base_file = v.to_string();
        }
        if let Some(v) = cfg.get_indexed_field_unquoted("SELBTN", i, "BACK_FILE") {
            t.filter_file = v.to_string();
        }
        let base_pos = get_tuple("BASE_POS");
        if base_pos.len() >= 2 {
            t.base_pos = (base_pos[0], base_pos[1]);
        }
        let rep_pos = get_tuple("REP_POS");
        if rep_pos.len() >= 2 {
            t.rep_pos = (rep_pos[0], rep_pos[1]);
        }
        let align = get_tuple("ALIGN");
        if align.len() >= 2 {
            t.x_align = align[0];
            t.y_align = align[1];
        }
        if let Some(v) = get_i64("MAX_Y_CNT") {
            t.max_y_cnt = v;
        }
        if let Some(v) = get_i64("LINE_WIDTH") {
            t.line_width = v;
        }
        let moji_size = get_tuple("MOJI_SIZE");
        if moji_size.len() >= 4 {
            t.moji_size = moji_size[0];
            t.moji_space = (moji_size[1], moji_size[2]);
            t.moji_cnt = moji_size[3];
        }
        let moji_pos = get_tuple("MOJI_POS");
        if moji_pos.len() >= 2 {
            t.moji_pos = (moji_pos[0], moji_pos[1]);
        }
        let moji_align = get_tuple("MOJI_ALIGN");
        if moji_align.len() >= 2 {
            t.moji_x_align = moji_align[0];
            t.moji_y_align = moji_align[1];
        }
        if let Some(v) = get_i64("MOJI_COLOR") {
            t.moji_color = v;
        }
        if let Some(v) = get_i64("MOJI_HIT_COLOR") {
            t.moji_hit_color = v;
        }
        if let Some(v) = get_i64("BTN_ACTION") {
            t.btn_action_no = v;
        }
        let open_anime = get_tuple("OPEN_ANIME");
        if open_anime.len() >= 2 {
            t.open_anime_type = open_anime[0];
            t.open_anime_time = open_anime[1];
        }
        let close_anime = get_tuple("CLOSE_ANIME");
        if close_anime.len() >= 2 {
            t.close_anime_type = close_anime[0];
            t.close_anime_time = close_anime[1];
        }
        let decide_anime = get_tuple("DECIDE_ANIME");
        if decide_anime.len() >= 2 {
            t.decide_anime_type = decide_anime[0];
            t.decide_anime_time = decide_anime[1];
        }
        out[i] = t;
    }
    out
}

fn load_emoji_templates(cfg: &GameexeConfig) -> Vec<EmojiTemplate> {
    let cnt = cfg.get_usize("EMOJI.CNT").unwrap_or(16).min(256);
    let mut out = vec![EmojiTemplate::default(); cnt];
    for (i, item) in out.iter_mut().enumerate() {
        let raw = cfg.get_unquoted(&format!("EMOJI.{i}"));
        let Some(raw) = raw else {
            continue;
        };
        let mut parts = raw.split(',').map(str::trim);
        item.file_name = parts.next().unwrap_or("").trim_matches('"').to_string();
        item.font_size = parts.next().and_then(parse_i64_like_local).unwrap_or(0);
    }
    out
}

fn load_message_button_templates(cfg: &GameexeConfig) -> Vec<MessageButtonTemplate> {
    let cnt = cfg.get_usize("MSGBTN.CNT").unwrap_or(16).min(256);
    let mut out = vec![MessageButtonTemplate::default(); cnt];
    const STATES: [(&str, usize); 3] = [("NORMAL", 0), ("HIT", 1), ("PUSH", 2)];
    for (i, item) in out.iter_mut().enumerate() {
        for (name, state) in STATES {
            let key = format!("MSGBTN.{i}.{name}.COLOR");
            if let Some(value) = cfg.get_unquoted(&key).and_then(parse_i64_like_local) {
                item.color_no[state] = value;
            }
        }
    }
    out
}

fn load_button_action_templates(cfg: &GameexeConfig) -> Vec<ButtonActionTemplate> {
    let cnt = cfg
        .get_usize("BUTTON.ACTION.CNT")
        .unwrap_or(INIDEF_BTN_ACTION_CNT)
        .min(INIMAX_BTN_ACTION_CNT);
    let mut out = vec![ButtonActionTemplate::default(); cnt];

    const STATES: [(&str, usize); TNM_BTN_STATE_MAX] = [
        ("NORMAL", 0),
        ("HIT", 1),
        ("PUSH", 2),
        ("SELECT", 3),
        ("DISABLE", 4),
    ];

    for i in 0..cnt {
        for (name, state_idx) in STATES {
            let Some(raw) = cfg.get_indexed_field("BUTTON.ACTION", i, name) else {
                continue;
            };
            let nums = parse_button_action_numbers(raw);
            if nums.len() >= 6 {
                out[i].state[state_idx] = ButtonActionPattern {
                    rep_pat_no: nums[0],
                    rep_pos_x: nums[1],
                    rep_pos_y: nums[2],
                    rep_tr: nums[3].clamp(0, 255),
                    rep_bright: nums[4].clamp(0, 255),
                    rep_dark: nums[5].clamp(0, 255),
                };
            }
        }
    }

    out
}

fn parse_button_action_numbers(raw: &str) -> Vec<i64> {
    raw.split(|c: char| c == ',' || c.is_ascii_whitespace())
        .filter(|s| !s.trim().is_empty())
        .filter_map(parse_i64_like_local)
        .collect()
}

fn parse_signed_decimal_numbers(raw: &str) -> Vec<i64> {
    let bytes = raw.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let signed = bytes[i] == b'-' || bytes[i] == b'+';
        let digit_at = if signed { i + 1 } else { i };
        if digit_at >= bytes.len() || !bytes[digit_at].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        i = digit_at + 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if let Ok(v) = raw[start..i].parse::<i64>() {
            out.push(v);
        }
    }
    out
}

fn load_shake_templates(cfg: &GameexeConfig) -> Vec<Vec<ShakeStep>> {
    let cnt = cfg
        .get_usize("SHAKE.CNT")
        .unwrap_or(INIDEF_SHAKE_CNT)
        .min(INIMAX_SHAKE_CNT);
    let mut out = vec![Vec::new(); cnt];
    for (i, steps) in out.iter_mut().enumerate() {
        let Some(entry) = cfg.get_indexed_entry("SHAKE", i) else {
            continue;
        };
        let nums = parse_signed_decimal_numbers(&entry.value);
        for triple in nums.as_chunks::<3>().0.iter() {
            steps.push(ShakeStep {
                x: triple[0].clamp(i32::MIN as i64, i32::MAX as i64) as i32,
                y: triple[1].clamp(i32::MIN as i64, i32::MAX as i64) as i32,
                time_ms: triple[2].clamp(0, i32::MAX as i64) as i32,
            });
        }
    }
    out
}

fn load_se_file_names(cfg: &GameexeConfig) -> Vec<Option<String>> {
    let cnt = cfg
        .get_usize("SE.CNT")
        .unwrap_or(INIDEF_SE_CNT)
        .clamp(INIMIN_SE_CNT, INIMAX_SE_CNT);
    let mut out = vec![None; cnt];

    for i in 0..cnt {
        if let Some(v) = cfg.get_indexed_unquoted("SE", i) {
            let t = v.trim();
            if !t.is_empty() {
                out[i] = Some(t.to_string());
            }
        }
    }

    out
}

fn load_button_se_templates(cfg: &GameexeConfig) -> Vec<ButtonSeTemplate> {
    let cnt = cfg
        .get_usize("BUTTON.SE.CNT")
        .unwrap_or(INIDEF_BTN_SE_CNT)
        .min(INIMAX_BTN_SE_CNT);
    let mut out = vec![ButtonSeTemplate::default(); cnt];

    for i in 0..cnt {
        if let Some(v) = cfg
            .get_indexed_field("BUTTON.SE", i, "HIT")
            .and_then(parse_i64_like_local)
        {
            out[i].hit_no = v;
        }
        if let Some(v) = cfg
            .get_indexed_field("BUTTON.SE", i, "PUSH")
            .and_then(parse_i64_like_local)
        {
            out[i].push_no = v;
        }
        if let Some(v) = cfg
            .get_indexed_field("BUTTON.SE", i, "DECIDE")
            .and_then(parse_i64_like_local)
        {
            out[i].decide_no = v;
        }
    }

    out
}

fn parse_i64_like_local(s: &str) -> Option<i64> {
    let t = s.trim().trim_matches('"');
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).ok()
    } else {
        t.parse::<i64>().ok()
    }
}

fn path_is_file(path: &Path) -> bool {
    crate::resource::game_file_exists(path)
}

fn path_is_dir(path: &Path) -> bool {
    crate::resource::resolve_game_path(path)
        .ok()
        .flatten()
        .is_some_and(|p| {
            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            {
                crate::resource::wasm_path_is_dir(&p)
            }
            #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
            {
                p.is_dir()
            }
        })
}

fn load_key_toml_config(
    project_dir: &Path,
) -> anyhow::Result<Option<siglus_assets::key_toml::KeyTomlConfig>> {
    crate::resource::load_project_key_toml(project_dir)
}

fn load_gameexe_decode_options(project_dir: &Path) -> anyhow::Result<GameexeDecodeOptions> {
    crate::resource::load_gameexe_decode_options(project_dir)
}

fn resolve_table_path(
    project_dir: &Path,
    dat_dir: &Path,
    raw: &str,
    default_ext: Option<&str>,
) -> Option<PathBuf> {
    let s = raw.trim().trim_matches('"');
    if s.is_empty() {
        return None;
    }

    let normalized = s.replace('\\', "/");
    let mut candidates = Vec::new();
    candidates.push(PathBuf::from(&normalized));

    let mut with_ext = PathBuf::from(&normalized);
    if with_ext.extension().is_none()
        && let Some(ext) = default_ext
    {
        with_ext.set_extension(ext);
    }
    if with_ext != normalized {
        candidates.push(with_ext.clone());
    }

    for cand in candidates {
        let direct = if cand.is_absolute() {
            cand.clone()
        } else {
            project_dir.join(&cand)
        };
        if let Some(resolved) = crate::resource::resolve_game_file(&direct).ok().flatten() {
            return Some(resolved);
        }

        let file_name = cand.file_name().map(PathBuf::from);
        if let Some(name_only) = file_name {
            let p = dat_dir.join(&name_only);
            if let Some(resolved) = crate::resource::resolve_game_file(&p).ok().flatten() {
                return Some(resolved);
            }
        }

        let p = dat_dir.join(&cand);
        if let Some(resolved) = crate::resource::resolve_game_file(&p).ok().flatten() {
            return Some(resolved);
        }

        if let Ok(stripped) = cand.strip_prefix("dat") {
            let p = dat_dir.join(stripped);
            if let Some(resolved) = crate::resource::resolve_game_file(&p).ok().flatten() {
                return Some(resolved);
            }
        }
    }
    None
}

#[cfg(test)]
mod raw_gameexe_tests {
    use super::*;

    #[test]
    fn raw_index_matches_original_last_definition_wins_and_keeps_quoting() {
        let text = "\u{feff} # waku . 000 . waku_file = \"first\" ;\n\
                    #WAKU.000.WAKU_FILE = \"last,b=c\"\n\
                    malformed line\n";
        let fields = index_raw_gameexe_fields(Some(text));
        assert_eq!(
            raw_indexed_field(&fields, "WAKU", 0, "WAKU_FILE").as_deref(),
            Some("\"last,b=c\"")
        );
        assert!(raw_indexed_field(&fields, "WAKU", 1, "WAKU_FILE").is_none());
        assert!(index_raw_gameexe_fields(None).is_empty());
    }

    #[test]
    fn raw_index_canonicalizes_numeric_components_and_last_definition_wins() {
        let fields = index_raw_gameexe_fields(Some(
            "#WAKU.1.BTN.2.FILE = \"plain first\"\n\
             #WAKU.001.BTN.002.FILE = \"padded last\"\n\
             #WAKU.001.BTN.003.FILE = \"only padded\"\n",
        ));
        assert_eq!(
            raw_nested_indexed_field(&fields, "WAKU", 1, "BTN", 2, "FILE").as_deref(),
            Some("\"padded last\"")
        );
        assert_eq!(
            raw_nested_indexed_field(&fields, "WAKU", 1, "BTN", 3, "FILE").as_deref(),
            Some("\"only padded\"")
        );
    }
}
