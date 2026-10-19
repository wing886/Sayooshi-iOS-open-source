//! `GAMEEXE.INI` / `SETUP.INI` for AVG32.
//!
//! AVG32's configuration keys are matched by prefix, including the
//! original quirks (`#SEL=`
//! entries without an index number are numbered in order of appearance,
//! `#SEEN_SRT` is accepted as a synonym of `#SEEN_START`, and so on).

use std::collections::BTreeMap;

use crate::effect::Effect;

pub const MAX_SEL: usize = 256;
pub const MAX_SE: usize = 16;
pub const MAX_MENU: usize = 32;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DsTrack {
    pub name: String,
    pub file: String,
    /// Byte offset playback loops back to (`#DSTRACK=..-..-<cut>`).
    pub cut_size: i32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Directory {
    pub directory: String,
    /// `P` (packed in `archive`) or `N` (loose files).
    pub mode: char,
    pub archive: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SysCom {
    pub name: String,
    pub enabled: bool,
    pub items: BTreeMap<usize, String>,
}

#[derive(Debug, Clone)]
pub struct Ini {
    pub sel: Vec<Option<Effect>>,
    pub cd: BTreeMap<usize, String>,
    pub dsound: Vec<DsTrack>,
    pub names: [String; 26],
    pub waku_file: String,
    pub save_file: String,
    pub caption: String,
    /// Optional encoding for script chapter/save titles, independent of FN.DAT text.
    pub title_nls: Option<crate::nls::Nls>,
    pub directories: BTreeMap<String, Directory>,
    pub bgm_dir: String,
    pub koe_dir: String,
    pub mov_dir: String,
    pub exfont: String,
    pub reg_name: String,
    pub save_header: String,
    pub save_no_title: String,
    pub cgm_file: String,
    pub use_font: String,
    pub se: [String; MAX_SE],
    pub music_type: i32,
    pub music_linear: i32,
    pub wav_linear: i32,
    pub koe_linear: i32,
    pub koe_type: i32,
    pub exfont_x: i32,
    pub exfont_y: i32,
    pub exfont_max_x: i32,
    pub exfont_max_y: i32,
    pub color_table: [[i32; 3]; 16],
    pub fade_table: [[i32; 3]; 16],
    pub shake: Vec<Vec<[i32; 3]>>,
    pub win_color: [i32; 3],
    pub win_color_flag: i32,
    pub syscom: BTreeMap<usize, SysCom>,
    pub win_attr_area: [i32; 4],
    pub win_x: i32,
    pub win_y: i32,
    pub mes_x: i32,
    pub mes_y: i32,
    pub sub_win_x: i32,
    pub sub_win_y: i32,
    pub sub_win_min: i32,
    pub sys_win: [i32; 2],
    pub sub_pos: [i32; 2],
    pub font_x: i32,
    pub font_y: i32,
    pub font_color: i32,
    /// Milliseconds per character (`#MSG_SPEED`, stored with its `+8` bias).
    pub mes_wait: i32,
    pub font_size: i32,
    pub name_indent: i32,
    pub name_space: i32,
    pub sel_blink_count: i32,
    pub sel_blink_time: i32,
    pub start_seen: i32,
    pub menu_seen: i32,
    pub save_file_count: i32,
    pub default_fade_time: i32,
    pub mes_icon_wait: i32,
    pub mes_icon_x: i32,
    pub mes_icon_y: i32,
    pub mes_icon_count: i32,
    pub novel_mode: i32,
    /// Remaining one-value system settings keyed by name (`MOJI_KAGE`, …),
    /// readable and writable from the `0x73` command family.
    pub values: BTreeMap<String, i32>,
}

impl Default for Ini {
    fn default() -> Self {
        let mut directories = BTreeMap::new();
        for (key, directory) in [
            ("PDT", "PDT"),
            ("WAV", "WAV"),
            ("TXT", "DAT"),
            ("ANM", "DAT"),
            ("ARD", "DAT"),
            ("CUR", "DAT"),
            ("***", "DAT"),
        ] {
            directories.insert(
                key.to_owned(),
                Directory {
                    directory: directory.to_owned(),
                    mode: 'N',
                    archive: String::new(),
                },
            );
        }
        Self {
            sel: vec![None; MAX_SEL],
            cd: BTreeMap::new(),
            dsound: Vec::new(),
            names: Default::default(),
            waku_file: String::new(),
            save_file: String::new(),
            caption: String::new(),
            title_nls: None,
            directories,
            bgm_dir: "BGM".into(),
            koe_dir: "KOE".into(),
            mov_dir: "MOV".into(),
            exfont: String::new(),
            reg_name: String::new(),
            save_header: String::new(),
            save_no_title: String::new(),
            cgm_file: "MODE.CGM".into(),
            use_font: String::new(),
            se: Default::default(),
            music_type: 0,
            music_linear: 0,
            wav_linear: 0,
            koe_linear: 0,
            koe_type: 0,
            exfont_x: 0,
            exfont_y: 0,
            exfont_max_x: 0,
            exfont_max_y: 0,
            color_table: [[0; 3]; 16],
            fade_table: [[0; 3]; 16],
            shake: vec![Vec::new(); 16],
            win_color: [0; 3],
            win_color_flag: 0,
            syscom: BTreeMap::new(),
            win_attr_area: [0; 4],
            win_x: 0,
            win_y: 0,
            mes_x: 0,
            mes_y: 0,
            sub_win_x: 0,
            sub_win_y: 0,
            sub_win_min: 0,
            sys_win: [0; 2],
            sub_pos: [0; 2],
            font_x: 0,
            font_y: 0,
            font_color: 0,
            mes_wait: 0,
            font_size: 0,
            name_indent: 0,
            name_space: 0,
            sel_blink_count: 0,
            sel_blink_time: 0,
            start_seen: 0,
            menu_seen: 0,
            save_file_count: 0,
            default_fade_time: 0,
            mes_icon_wait: 100,
            mes_icon_x: 16,
            mes_icon_y: 16,
            mes_icon_count: 16,
            novel_mode: 0,
            values: BTreeMap::new(),
        }
    }
}

/// A cursor over one configuration line (character search, number and
/// string readers).
struct Line<'a> {
    text: &'a [u8],
    at: usize,
}

impl<'a> Line<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text: text.as_bytes(),
            at: 0,
        }
    }

    fn rest(&self) -> &'a [u8] {
        &self.text[self.at.min(self.text.len())..]
    }

    fn peek(&self) -> Option<u8> {
        self.text.get(self.at).copied()
    }

    /// Advances just past the next `wanted`; `false` (without moving) if the
    /// line ends first.
    fn search(&mut self, wanted: u8) -> bool {
        match self.rest().iter().position(|byte| *byte == wanted) {
            Some(offset) => {
                self.at += offset + 1;
                true
            }
            None => {
                self.at = self.text.len();
                false
            }
        }
    }

    fn number(&mut self) -> i32 {
        while self.peek() == Some(b' ') {
            self.at += 1;
        }
        let negative = self.peek() == Some(b'-');
        if negative {
            self.at += 1;
        }
        let mut value: i32 = 0;
        while let Some(digit) = self.peek().filter(u8::is_ascii_digit) {
            value = value.wrapping_mul(10).wrapping_add(i32::from(digit - b'0'));
            self.at += 1;
        }
        if negative { -value } else { value }
    }

    fn string(&mut self) -> String {
        if !self.search(b'"') {
            return String::new();
        }
        let start = self.at;
        if !self.search(b'"') {
            return String::new();
        }
        String::from_utf8_lossy(&self.text[start..self.at - 1]).into_owned()
    }
}

impl Ini {
    /// Parses `GAMEEXE.INI` text (already decoded from Shift-JIS) and the
    /// optional `SETUP.INI`.
    pub fn parse(gameexe: &str, setup: Option<&str>) -> Self {
        let mut ini = Self::default();
        let mut sel_number = 0usize;
        for line in gameexe.lines() {
            let line = line.trim_end_matches(['\r', '\n']);
            if line.starts_with('#') {
                ini.item(line, &mut sel_number);
            }
        }
        if let Some(setup) = setup {
            for line in setup.lines() {
                ini.setup_item(line.trim_end_matches(['\r', '\n']));
            }
        }
        ini.finish();
        ini
    }

    fn item(&mut self, text: &str, sel_number: &mut usize) {
        let mut line = Line::new(text);
        let is = |prefix: &str| text.starts_with(prefix);
        if is("#CDTRACK") {
            line.search(b'=');
            let digits = line.rest();
            if digits.len() >= 2 && digits[0].is_ascii_digit() && digits[1].is_ascii_digit() {
                let track = usize::from(digits[0] - b'0') * 10 + usize::from(digits[1] - b'0');
                if line.search(b'=') {
                    self.cd.insert(track, line.string());
                }
            }
        } else if is("#DSTRACK") {
            line.search(b'=');
            line.search(b'-');
            line.search(b'-');
            let cut_size = line.number();
            line.search(b'=');
            let file = line.string();
            let name = if line.search(b'=') {
                line.string()
            } else {
                file.clone()
            };
            if self.dsound.len() < 100 {
                self.dsound.push(DsTrack {
                    name,
                    file,
                    cut_size,
                });
            }
        } else if let Some(kind) = text.strip_prefix("#DIRC.") {
            let key: String = kind
                .chars()
                .take_while(|c| *c != '=' && *c != ' ')
                .collect();
            line.search(b'=');
            let directory = line.string();
            line.search(b'=');
            while line.peek() == Some(b' ') {
                line.at += 1;
            }
            let mode = line
                .peek()
                .map_or('N', |byte| (byte as char).to_ascii_uppercase());
            let archive = if line.search(b':') {
                line.string()
            } else {
                String::new()
            };
            self.directories.insert(
                key.to_ascii_uppercase(),
                Directory {
                    directory,
                    mode,
                    archive,
                },
            );
        } else if is("#SE.") {
            line.search(b'.');
            let index = line.number();
            if (0..MAX_SE as i32).contains(&index) {
                line.search(b'=');
                self.se[index as usize] = line.string();
            }
        } else if is("#SEL.") || is("#SEL=") {
            let index = if is("#SEL.") {
                line.search(b'.');
                line.number()
            } else {
                let index = *sel_number as i32;
                *sel_number += 1;
                index
            };
            if (0..MAX_SEL as i32).contains(&index) {
                let mut values = [0i32; 15];
                line.search(b'=');
                values[0] = line.number();
                for value in values.iter_mut().skip(1) {
                    line.search(b',');
                    *value = line.number();
                }
                self.sel[index as usize] = Some(Effect::from_sel(&values));
            }
        } else if is("#WAKUPDT") {
            line.search(b'=');
            self.waku_file = line.string();
        } else if is("#EXFONT_N_NAME") {
            line.search(b'=');
            self.exfont = line.string();
        } else if is("#EXFONT_N_XSIZE") {
            line.search(b'=');
            self.exfont_x = line.number();
        } else if is("#EXFONT_N_YSIZE") {
            line.search(b'=');
            self.exfont_y = line.number();
        } else if is("#EXFONT_N_XCONT") {
            line.search(b'=');
            self.exfont_max_x = line.number();
        } else if is("#EXFONT_N_YCONT") {
            line.search(b'=');
            self.exfont_max_y = line.number();
        } else if is("#TITLE_NLS") {
            line.search(b'=');
            self.title_nls = line.string().parse().ok();
        } else if is("#CAPTION") {
            line.search(b'=');
            self.caption = line.string();
        } else if is("#SAVENOTITLE") {
            line.search(b'=');
            self.save_no_title = line.string();
        } else if is("#NAME.") {
            let rest = &text.as_bytes()["#NAME.".len()..];
            if let Some(&letter) = rest.first() {
                let index = i32::from(letter) - i32::from(b'A');
                let single = rest.get(1).is_none_or(|next| !next.is_ascii_uppercase());
                if (0..26).contains(&index) && single {
                    line.search(b'=');
                    self.names[index as usize] = line.string();
                }
            }
        } else if is("#COLOR_TABLE") || is("#FADE_TABLE") {
            line.search(b'.');
            let index = line.number();
            if (0..16).contains(&index) {
                line.search(b'=');
                let colour = [
                    line.number(),
                    {
                        line.search(b',');
                        line.number()
                    },
                    {
                        line.search(b',');
                        line.number()
                    },
                ];
                if is("#COLOR_TABLE") {
                    self.color_table[index as usize] = colour;
                } else {
                    self.fade_table[index as usize] = colour;
                }
            }
        } else if is("#WINDOW_ATTR=") {
            line.search(b'=');
            self.win_color[0] = line.number();
            line.search(b',');
            self.win_color[1] = line.number();
            line.search(b',');
            self.win_color[2] = line.number();
        } else if is("#WINDOW_ATTR_AREA") {
            line.search(b'=');
            for (index, value) in self.win_attr_area.iter_mut().enumerate() {
                if index > 0 {
                    line.search(b',');
                }
                *value = line.number();
            }
        } else if is("#WINDOW_ATTR_TYPE") {
            line.search(b'=');
            self.win_color_flag = line.number();
        } else if is("#WINDOW_MSG_POS") {
            (self.win_x, self.win_y) = pair(&mut line);
        } else if is("#WINDOW_COM_POS") {
            (self.sub_win_x, self.sub_win_y) = pair(&mut line);
        } else if is("#WINDOW_SYS_POS") {
            let (x, y) = pair(&mut line);
            self.sys_win = [x, y];
        } else if is("#WINDOW_SUB_POS") {
            let (x, y) = pair(&mut line);
            self.sub_pos = [x, y];
        } else if is("#COM_WIND_MIN_SIZE") {
            line.search(b'=');
            self.sub_win_min = line.number();
        } else if is("#MESSAGE_SIZE") {
            (self.mes_x, self.mes_y) = pair(&mut line);
        } else if is("#MSG_MOJI_SIZE") {
            (self.font_x, self.font_y) = pair(&mut line);
        } else if is("#FONT_SIZE") {
            line.search(b'=');
            self.font_size = line.number();
        } else if is("#MOJI_COLOR") {
            line.search(b'=');
            self.font_color = line.number();
        } else if is("#MSG_SPEED") {
            line.search(b'=');
            let speed = line.number().max(0);
            self.mes_wait = if speed != 0 { speed + 8 } else { 0 };
        } else if is("#NAME_AFTER_SPACE") {
            line.search(b'=');
            self.name_space = line.number();
        } else if is("#NAME_INDENT_SPACE") {
            line.search(b'=');
            self.name_indent = line.number();
        } else if is("#SEL_BLINK_COUNT") {
            line.search(b'=');
            self.sel_blink_count = line.number();
        } else if is("#SEL_BLINK_SPEED") {
            line.search(b'=');
            self.sel_blink_time = line.number();
        } else if is("#FADE_TIME") {
            line.search(b'=');
            self.default_fade_time = line.number();
        } else if is("#MUSIC_TYPE") {
            line.search(b'=');
            self.music_type = line.number();
        } else if is("#MUSIC_LINEAR_PAC") {
            line.search(b'=');
            self.music_linear = line.number();
        } else if is("#WAVE_LINEAR_PAC") {
            line.search(b'=');
            self.wav_linear = line.number();
        } else if is("#KOE_LINEAR_PAC") {
            line.search(b'=');
            self.koe_linear = line.number();
        } else if is("#RETN_SPEED") {
            line.search(b'=');
            self.mes_icon_wait = line.number();
        } else if is("#RETN_CONT") {
            line.search(b'=');
            self.mes_icon_count = line.number();
        } else if is("#RETN_XSIZE") {
            line.search(b'=');
            self.mes_icon_x = line.number();
        } else if is("#RETN_YSIZE") {
            line.search(b'=');
            self.mes_icon_y = line.number();
        } else if is("#SHAKE.") {
            line.search(b'.');
            let index = line.number();
            if (0..16).contains(&index) {
                line.search(b'=');
                let mut steps = Vec::new();
                while line.peek() == Some(b'(') && steps.len() < 16 {
                    line.at += 1;
                    let x = line.number();
                    line.search(b',');
                    let y = line.number();
                    line.search(b',');
                    let wait = line.number();
                    line.search(b')');
                    steps.push([x, y, wait]);
                }
                self.shake[index as usize] = steps;
            }
        } else if is("#SEEN_START") || is("#SEEN_SRT") {
            line.search(b'=');
            self.start_seen = line.number();
        } else if is("#SEEN_MENU") {
            line.search(b'=');
            self.menu_seen = line.number();
        } else if is("#SAVENAME") {
            line.search(b'=');
            self.save_file = line.string();
        } else if is("#REGNAME") {
            line.search(b'=');
            self.reg_name = line.string();
        } else if is("#SAVETITLE") {
            line.search(b'=');
            self.save_header = line.string();
        } else if is("#SAVEFILETIME") {
            line.search(b'=');
            let digits = line.rest();
            let count = if digits.len() >= 2 {
                (i32::from(digits[0]) - i32::from(b'0')) * 10 + i32::from(digits[1])
                    - i32::from(b'0')
            } else {
                0
            };
            self.save_file_count = if (1..=30).contains(&count) { count } else { 30 };
        } else if is("#NVL_SYSTEM") {
            line.search(b'=');
            self.novel_mode = line.number();
        } else if is("#SYSCOM.") {
            line.search(b'.');
            let index = line.number();
            if !(0..MAX_MENU as i32).contains(&index) {
                return;
            }
            let entry = self.syscom.entry(index as usize).or_default();
            match line.peek() {
                Some(b'.') => {
                    line.at += 1;
                    let item = line.number();
                    line.search(b'=');
                    entry.items.insert(item.max(0) as usize, line.string());
                }
                Some(b'=') => {
                    line.at += 1;
                    if line.peek() == Some(b'U') {
                        entry.enabled = true;
                        entry.name = line.string();
                    }
                }
                _ => {}
            }
        } else if is("#KOE_DOUBLE_PAC") {
            line.search(b'=');
            self.koe_type = line.number();
        } else if is("#CGM_FILE") {
            line.search(b'=');
            let name = line.string();
            if !name.is_empty() {
                self.cgm_file = name;
            }
        } else if is("#USEFONT") {
            line.search(b'=');
            self.use_font = line.string();
        } else if let Some(rest) = text.strip_prefix('#') {
            // Every other single-number setting (`#MOJI_KAGE=000` …).
            let key: String = rest.chars().take_while(|c| *c != '=').collect();
            if line.search(b'=') {
                let digits = line.rest();
                if digits
                    .first()
                    .is_some_and(|c| c.is_ascii_digit() || *c == b'-')
                {
                    let value = line.number();
                    self.values.insert(key.trim().to_owned(), value);
                }
            }
        }
    }

    fn setup_item(&mut self, text: &str) {
        let value = |prefix: &str| -> Option<String> {
            let rest = text.strip_prefix(prefix)?;
            let rest = rest.split_once('=')?.1;
            Some(rest.chars().filter(|c| *c != ' ').take(32).collect())
        };
        if let Some(directory) = value("BGM_FOLDER") {
            self.bgm_dir = directory;
        } else if let Some(directory) = value("KOE_FOLDER") {
            self.koe_dir = directory;
        } else if let Some(directory) = value("MOV_FOLDER") {
            self.mov_dir = directory;
        }
    }

    fn finish(&mut self) {
        if self.novel_mode != 0 && self.font_x == 0 {
            self.font_x = 12;
        }
        let limit = (self.font_x * 2).min(self.font_y);
        if self.font_size == 0 {
            self.font_size = limit;
        } else {
            if limit != 0 && limit < self.font_size {
                self.font_size = limit;
            }
            if self.font_x == 0 {
                self.font_x = self.font_size / 2;
            }
            if self.font_y == 0 {
                self.font_y = self.font_size + 2;
            }
        }
        if self.font_size == 0 {
            self.font_size = 16;
            self.font_y = 18;
            self.font_x = 8;
        }
        self.font_x &= !1;
        self.font_y &= !1;
        if self.menu_seen == 0 {
            self.menu_seen = self.start_seen;
        }
        if self.save_file_count == 0 {
            self.save_file_count = 30;
        }
        if self.mes_icon_count <= 0 {
            self.mes_icon_count = 1;
        }
    }

    pub fn value(&self, key: &str) -> i32 {
        self.values.get(key).copied().unwrap_or(0)
    }

    pub fn set_value(&mut self, key: &str, value: i32) {
        self.values.insert(key.to_owned(), value);
    }

    /// A `#SEL` preset primed to run from buffer 1 to 0.
    pub fn sel(&self, index: i32, now: u64) -> Effect {
        let mut effect = usize::try_from(index)
            .ok()
            .and_then(|index| self.sel.get(index).copied().flatten())
            .unwrap_or_default();
        effect.curcount = 0;
        effect.prevtime = now;
        effect.srcpdt = 1;
        effect.dstpdt = 0;
        effect
    }
}

fn pair(line: &mut Line<'_>) -> (i32, i32) {
    line.search(b'=');
    let first = line.number();
    line.search(b',');
    (first, line.number())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#SEEN_START=002\n#SEEN_MENU=010\n#DIRC.PDT=\"PDT\" =N:\"ALLPDT.PDL\"\n\
#DIRC.TXT=\"DAT\" =P:\"SEEN.TXT\"\n#WINDOW_MSG_POS=022,350\n#MESSAGE_SIZE=023,003\n\
#MSG_MOJI_SIZE=012,029\n#FONT_SIZE=026\n#MSG_SPEED=000\n#COLOR_TABLE.001=255,255,255\n\
#SEL.004=150,000,639,479,000,000,003,031,001,000,000,000,000,000,000\n\
#SHAKE.005=(000,008,016)(000,-08,016)\n\
#CDTRACK=02:00:00:00-02:03:18:00-02:00:00:00=\"002\"\n#NAME.A=\"往人\"\n\
#SYSCOM.000=U:\"SAVE\"\n#SYSCOM.006=N:\"\"\n#SYSCOM.012.001=\"pattern 2\"\n#MOJI_KAGE=001\n";

    #[test]
    fn title_encoding_override_is_opt_in() {
        assert_eq!(Ini::default().title_nls, None);
        assert_eq!(Ini::parse("#TITLE_NLS=\"gbk\"", None).title_nls, Some(crate::nls::Nls::Gbk));
        assert_eq!(Ini::parse("#TITLE_NLS=\"invalid\"", None).title_nls, None);
    }

    #[test]
    fn parses_the_reference_keys() {
        let ini = Ini::parse(SAMPLE, None);
        assert_eq!(ini.start_seen, 2);
        assert_eq!(ini.menu_seen, 10);
        assert_eq!((ini.win_x, ini.win_y), (22, 350));
        assert_eq!((ini.mes_x, ini.mes_y), (23, 3));
        // FONT_SIZE is capped by twice the character width.
        assert_eq!((ini.font_x, ini.font_y, ini.font_size), (12, 28, 24));
        assert_eq!(ini.directories["TXT"].mode, 'P');
        assert_eq!(ini.directories["TXT"].archive, "SEEN.TXT");
        let sel = ini.sel[4].unwrap();
        assert_eq!((sel.sx1, sel.cmd, sel.steptime, sel.mask), (150, 31, 3, 1));
        assert_eq!(ini.shake[5], vec![[0, 8, 16], [0, -8, 16]]);
        assert_eq!(ini.cd[&2], "002");
        assert_eq!(ini.names[0], "往人");
        assert!(ini.syscom[&0].enabled);
        assert!(!ini.syscom[&6].enabled);
        assert_eq!(ini.syscom[&12].items[&1], "pattern 2");
        assert_eq!(ini.value("MOJI_KAGE"), 1);
    }
}
