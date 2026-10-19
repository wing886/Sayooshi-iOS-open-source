//! Player settings (the values behind the system command menu) and the
//! state of the system commands themselves.

use crate::gameexe::Gameexe;

/// System command numbers (`#SYSCOM`, `InvokeSyscom`).
pub mod syscom {
    pub const SAVE: i32 = 0;
    pub const LOAD: i32 = 1;
    pub const MESSAGE_SPEED: i32 = 2;
    pub const WINDOW_ATTRIBUTES: i32 = 3;
    pub const VOLUME_SETTINGS: i32 = 4;
    pub const DISPLAY_MODE: i32 = 5;
    pub const MISCELLANEOUS_SETTINGS: i32 = 6;
    pub const VOICE_SETTINGS: i32 = 8;
    pub const FONT_SELECTION: i32 = 9;
    pub const BGM_FADE: i32 = 10;
    pub const BGM_SETTINGS: i32 = 11;
    pub const WINDOW_DECORATION_STYLE: i32 = 12;
    pub const AUTO_MODE_SETTINGS: i32 = 13;
    pub const RETURN_TO_PREVIOUS_SELECTION: i32 = 14;
    pub const USE_KOE: i32 = 15;
    pub const DISPLAY_VERSION: i32 = 16;
    pub const SHOW_WEATHER: i32 = 17;
    pub const SHOW_OBJECT_1: i32 = 18;
    pub const SHOW_OBJECT_2: i32 = 19;
    pub const CLASSIFY_TEXT: i32 = 20;
    pub const GENERIC_1: i32 = 21;
    pub const GENERIC_2: i32 = 22;
    pub const OPEN_MANUAL_PATH: i32 = 24;
    pub const SET_SKIP_MODE: i32 = 25;
    pub const AUTO_MODE: i32 = 26;
    pub const MENU_RETURN: i32 = 28;
    pub const EXIT_GAME: i32 = 29;
    pub const HIDE_MENU: i32 = 30;
    pub const SHOW_BACKGROUND: i32 = 31;
    pub const COUNT: usize = 32;

    /// Labels used by the built-in menu when `#SYSCOM.nnn` gives none.
    pub fn default_label(index: i32) -> &'static str {
        match index {
            SAVE => "セーブ",
            LOAD => "ロード",
            MESSAGE_SPEED => "文字表示速度",
            WINDOW_ATTRIBUTES => "ウィンドウ背景色",
            VOLUME_SETTINGS => "音量",
            DISPLAY_MODE => "フルスクリーン",
            MISCELLANEOUS_SETTINGS => "その他の設定",
            VOICE_SETTINGS => "音声モード",
            FONT_SELECTION => "フォント",
            BGM_FADE => "BGMフェード",
            WINDOW_DECORATION_STYLE => "ウィンドウデザイン",
            AUTO_MODE_SETTINGS => "オートモード設定",
            RETURN_TO_PREVIOUS_SELECTION => "前の選択肢に戻る",
            USE_KOE => "キャラクターボイス",
            DISPLAY_VERSION => "バージョン情報",
            SHOW_WEATHER => "天候エフェクト",
            SHOW_OBJECT_1 => "オブジェクト１",
            SHOW_OBJECT_2 => "オブジェクト２",
            CLASSIFY_TEXT => "既読文字色",
            GENERIC_1 => "設定１",
            GENERIC_2 => "設定２",
            OPEN_MANUAL_PATH => "マニュアル",
            SET_SKIP_MODE => "既読スキップ",
            AUTO_MODE => "オートモード",
            MENU_RETURN => "タイトルに戻る",
            EXIT_GAME => "ゲーム終了",
            HIDE_MENU => "メニューを閉じる",
            SHOW_BACKGROUND => "ウィンドウを消す",
            _ => "",
        }
    }
}

/// The five components of a window background (`#WINDOW_ATTR`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowAttr {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub alpha: i32,
    /// 0 subtractive, 1 alpha blend.
    pub filter: i32,
}

impl WindowAttr {
    pub fn from_slice(values: &[i32]) -> Option<Self> {
        match values {
            [r, g, b, alpha, filter, ..] => Some(Self {
                r: *r,
                g: *g,
                b: *b,
                alpha: *alpha,
                filter: *filter,
            }),
            [r, g, b, alpha] => Some(Self {
                r: *r,
                g: *g,
                b: *b,
                alpha: *alpha,
                filter: 1,
            }),
            _ => None,
        }
    }
}

impl Default for WindowAttr {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            alpha: 128,
            filter: 1,
        }
    }
}

/// Indices into the per-channel volume and enable arrays.
pub mod channel {
    pub const BGM: usize = 0;
    pub const KOE: usize = 1;
    pub const PCM: usize = 2;
    pub const SE: usize = 3;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// 0 (slowest) .. 255 (fastest); `message_no_wait` shows text at once.
    pub message_speed: i32,
    pub message_no_wait: bool,
    /// Text only, text and voice, voice only.
    pub koe_mode: i32,
    pub bgm_koe_fade: bool,
    pub bgm_koe_fade_vol: i32,
    /// BGM, KOE, PCM, SE volumes (0-255).
    pub volume: [i32; 4],
    pub enabled: [bool; 4],
    pub auto_mode: bool,
    pub auto_char_time: i32,
    pub auto_base_time: i32,
    pub font_quality: i32,
    pub font_weight: i32,
    pub font_shadow: i32,
    pub window_attr: WindowAttr,
    pub show_object: [bool; 2],
    pub show_weather: bool,
    pub classify_text: bool,
    pub generic: [i32; 2],
    /// 1 windowed, 0 full screen.
    pub screen_mode: i32,
    pub cursor_mono: bool,
    pub skip_animations: bool,
    pub low_priority: bool,
    pub confirm_save_load: bool,
    pub reduce_distortion: bool,
    pub sound_quality: i32,
    /// Per-character voice switches (`SetUseKoe`), keyed by character id.
    pub use_koe: std::collections::BTreeMap<i32, bool>,
    /// Waku pattern for all windows (`SetWakuAll`).
    pub waku_all: i32,
}

/// Values the `Def*` functions report.
#[derive(Debug, Clone, PartialEq)]
pub struct Defaults(pub Settings);

impl Settings {
    pub fn from_gameexe(exe: &Gameexe) -> Self {
        let flag = |key: &str, default: bool| exe.int(key).map_or(default, |v| v != 0);
        let window_attr = WindowAttr::from_slice(&exe.ints("WINDOW_ATTR")).unwrap_or_default();
        Self {
            message_speed: exe.int_or("INIT_MESSAGE_SPEED", 30),
            message_no_wait: flag("INIT_MESSAGE_SPEED_MOD", false),
            koe_mode: exe.int_or("INIT_KOEMODE", 0),
            bgm_koe_fade: flag("INIT_BGMKOEFADE", true),
            bgm_koe_fade_vol: exe.int_or("INIT_BGMKOEFADEVOL", 128),
            volume: [
                exe.int_or("INIT_BGMVOLUME_MOD", 255),
                exe.int_or("INIT_KOEVOLUME_MOD", 255),
                exe.int_or("INIT_PCMVOLUME_MOD", 255),
                exe.int_or("INIT_SEVOLUME_MOD", 255),
            ],
            enabled: [
                flag("INIT_BGMONOFF_MOD", true),
                flag("INIT_KOEONOFF_MOD", true),
                flag("INIT_PCMONOFF_MOD", true),
                flag("INIT_SEONOFF_MOD", true),
            ],
            auto_mode: flag("INIT_AUTOMODE_ONOFF_MOD", false),
            auto_char_time: exe.int_or("INIT_AUTOCHARTIME_MOD", 100),
            auto_base_time: exe.int_or("INIT_AUTOBASETIME_MOD", 2000),
            font_quality: exe.int_or("INIT_FONT_QUALITY", 2),
            font_weight: exe.int_or("INIT_FONT_WEIGHT", 0),
            font_shadow: exe.int_or("INIT_FONT_SHADOW", 1),
            window_attr,
            show_object: [
                flag("INIT_OBJECT1_ONOFF_MOD", true),
                flag("INIT_OBJECT2_ONOFF_MOD", true),
            ],
            show_weather: flag("INIT_WEATHER_ONOFF_MOD", true),
            classify_text: flag("INIT_CLASSIFYTEXT_MOD", false),
            generic: [
                exe.int_or("INIT_ORIGINALSETING1_MOD", 0),
                exe.int_or("INIT_ORIGINALSETING2_MOD", 0),
            ],
            screen_mode: exe.int_or("INIT_SCREENMODE", 1),
            cursor_mono: false,
            skip_animations: false,
            low_priority: false,
            confirm_save_load: true,
            reduce_distortion: false,
            sound_quality: 5,
            use_koe: Default::default(),
            waku_all: 0,
        }
    }
}

/// Visibility and enablement of each system command: 0 hidden, 1 shown,
/// 2 shown but disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syscom {
    pub states: [i32; syscom::COUNT],
    /// `HideSyscom()` / `EnableSyscom()` without an argument.
    pub menu_enabled: bool,
    /// `EnableSkipMode` / `DisableSkipMode`.
    pub skip_mode_allowed: bool,
    /// The main game's skip mode (`SetSkipMode`/`SetLocalSkipMode`).
    pub skip_mode: bool,
}

impl Syscom {
    pub fn from_gameexe(exe: &Gameexe) -> Self {
        let mut states = [1; syscom::COUNT];
        // Games list the commands they use; others start hidden.
        let any_defined = exe.filter("SYSCOM.").next().is_some();
        if any_defined {
            for (index, state) in states.iter_mut().enumerate() {
                let key = format!("SYSCOM.{index:03}");
                *state = i32::from(exe.exists(&key));
            }
        }
        Self {
            states,
            menu_enabled: exe.int("SYSCOM_USE") != Some(0),
            skip_mode_allowed: true,
            skip_mode: false,
        }
    }

    pub fn state(&self, index: i32) -> i32 {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.states.get(index))
            .copied()
            .unwrap_or(0)
    }

    pub fn set_state(&mut self, index: i32, state: i32) {
        if let Some(slot) = usize::try_from(index)
            .ok()
            .and_then(|index| self.states.get_mut(index))
        {
            *slot = state;
        }
    }
}
