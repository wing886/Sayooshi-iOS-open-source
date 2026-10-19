//! Engine start-up, files, save data, background transitions and the
//! remaining opcode handlers of `sub_19640`.

use anyhow::{Context, Result};
use encoding_rs::SHIFT_JIS;

use super::Engine;
use super::ds::*;
use super::interp::trim_name;
use super::map::ENTITY_SIZE;
use super::mem::{FarPtr, ds_ptr, ptr_add};
use super::object::{OBJ_SIZE, flag};
use super::{MusicCommand, QuitRequested};

const SAVE_COMMENT_LEN: usize = 0x28;

impl Engine {
    // ---- files ------------------------------------------------------------

    fn engine_name(name: &[u8]) -> String {
        let bytes = trim_name(name);
        let (text, _, _) = SHIFT_JIS.decode(&bytes);
        text.into_owned()
    }

    /// `sub_2231D` + read: a resource by engine name (virtual extensions,
    /// loose files first, then DLB archives).
    pub fn read_resource(&mut self, name: &[u8]) -> Result<Vec<u8>> {
        let name = Self::engine_name(name);
        self.game
            .read(&name)
            .with_context(|| format!("uk2: missing resource {name:?}"))
    }

    pub fn resource_exists(&self, name: &[u8]) -> bool {
        let name = Self::engine_name(name);
        self.game.resolve(&name).is_some()
    }

    pub fn read_data_file(&self, name: &[u8]) -> Option<Vec<u8>> {
        let name = Self::engine_name(name);
        let path = self.game.resolve(&name)?;
        std::fs::read(path).ok()
    }

    pub fn write_data_file(&mut self, name: &[u8], bytes: &[u8]) -> Result<()> {
        let name = Self::engine_name(name);
        self.game.write_file(&name, bytes)
    }

    // ---- start-up ---------------------------------------------------------

    /// `sub_1EE5B`: reset interpreter-visible engine state.
    pub fn reset_state(&mut self) {
        for off in [
            W_29008,
            W_27140,
            GLYPH_STYLE,
            W_27152,
            MAP_ACTIVE,
            W_29014,
            W_28FC6,
            W_29016,
        ] {
            self.set_w(off, 1);
        }
        for off in [
            W_28FFC,
            W_28FF8,
            W_28FFA,
            W_28FEC,
            W_28FF2,
            W_29000,
            W_29002,
            W_29004,
            TEXT_DONE,
            WAIT_MODE,
            FORCE_REDRAW,
            CELL_MAP_SEL,
            MAP_CELL_SEL,
            l(0x2714A),
            l(0x2714C),
            MAP_EVENT,
            ENTITY_COUNT,
            W_28FC4,
            OBJ_COUNT,
            FRAME_STYLE,
            W_2901A,
            W_29012,
        ] {
            self.set_w(off, 0);
        }
        self.set_b(WAIT_FLAG, 0);
        self.set_b(WAIT_OBJECT, 0);
        self.set_w(l(0x27150), 1);
        self.set_w(FRAME_DELAY, 1);
        self.set_w(TEXT_COLOUR, 0);
        self.set_w(TEXT_OUTLINE, 7);
        self.set_w(TEXT_SHADOW, 0);
        self.mem.memset(ds_ptr(INDEXED_5), 0, 0x100);
        self.mem.memset(ds_ptr(INDEXED_8), 0, 0x100);
        self.mem.memset(ds_ptr(SYSTEM_WORDS), 0, 0x34);
        self.mem.memset(ds_ptr(FIXED_STRINGS), 0, 0x3a2);
    }

    /// `sub_15939` + `sub_1EF24`: configuration and engine initialisation.
    pub fn init(&mut self) -> Result<()> {
        // sub_14A52 / sub_15939: configuration.
        self.mem.strcpy_bytes(ds_ptr(ENTITY_FILE), b"font.tab1");
        let config = self.game.config.clone();
        self.mem
            .strcpy_bytes(ds_ptr(l(0x25DCC)), config.midi_ext.as_bytes());
        self.mem
            .strcpy_bytes(ds_ptr(l(0x25DD0)), config.fm_ext.as_bytes());
        self.mem
            .strcpy_bytes(ds_ptr(NEXT_MES_NAME), config.start.as_bytes());
        if !config.font.is_empty() {
            self.mem
                .strcpy_bytes(ds_ptr(ENTITY_FILE), config.font.as_bytes());
        }
        if let Some(value) = config.extra.get("NOTE")
            && value.eq_ignore_ascii_case("ON")
        {
            self.set_w(PDT_PALETTE, 0);
            self.mem.strcpy_bytes(ds_ptr(0x7e0), b"note.tbl1");
        }
        self.set_w(EMS_ACTIVE, 0);
        self.set_w(EMS_WANTED, 0);
        // sub_1EF24
        self.mem.set_d(OBJ_TABLE, 0);
        self.set_w(OPCODES_ENABLED, 0);
        self.set_w(TEMPLATE_COUNT, 0);
        self.set_w(MES_LEN, 0);
        self.set_w(MES_STACK, 0);
        self.reset_state();
        let work = self.mem.alloc(0x55f0);
        self.mem.set_d(l(0x2710C), work);
        self.mem.set_d(CELL_MAPS, work);
        self.mem.set_d(CELL_MAPS + 4, ptr_add(work, 0x3e8));
        self.mem.set_d(MAP_CELLS, ptr_add(work, 0x7d0));
        self.mem.set_d(MAP_CELLS + 4, ptr_add(work, 0x2ee0));
        self.mem.memset(work, 0xf0, 0x3e8);
        self.mem.memset(ptr_add(work, 0x3e8), 0xf0, 0x3e8);
        let tiles = self.mem.alloc(0x4e20);
        self.mem.set_d(SHARED_TILES, tiles);
        let overlay = self.mem.alloc(0xe100);
        self.mem.set_d(CHIP_OVERLAY, overlay);
        let base = self.mem.alloc(0xfa00);
        self.mem.set_d(CHIP_BASE, base);
        let table = self.mem.alloc(0x118);
        self.mem.set_d(OBJ_TABLE, table);
        let pool = self.mem.alloc(0x1acc);
        self.mem.set_d(l(0x2901E), pool);
        for i in 0..70 {
            self.mem
                .wd(ptr_add(table, i * 4), ptr_add(pool, i * OBJ_SIZE as i32));
        }
        // sub_21A7B: mouse driver, keyboard hook, music drivers.
        self.set_w(CURSOR_ENABLED, 1);
        self.set_w(CURSOR_MODE, 0);
        self.set_w(CURSOR_SHOWN, 0);
        self.set_w(CURSOR_SHOWN2, 0);
        self.set_b(EVENT_LIFETIME, 0x13);
        self.mem.memset(ds_ptr(KEY_TABLE), 0, 0x100);
        let drivers = self.platform.music_drivers();
        self.set_w(l(0x25DC6), drivers);
        self.cursor_show(1);
        // sub_1F835: graphics mode, black palette, page 0.
        self.mem.memset(ds_ptr(PALETTE), 0, 0x20);
        self.apply_palette()?;
        self.set_pages(0, 0);
        self.set_w(GLYPH_OP, 1);
        let win = self.mem.cstr(self.mem.d(l(0x23BDE)));
        let chips = self.pdt_to_cells(&win, 0)?;
        self.mem.set_d(WIN_CHIPS, chips);
        let disp = self.mem.cstr(self.mem.d(l(0x23BE2)));
        let parts = self.pdt_to_cells(&disp, 0)?;
        self.mem.set_d(WIN_DISP_CHIPS, parts);
        self.pdt_show(b"kana.pdt1")?;
        self.capture_ank_font();
        self.vram.clear_planes(0x0f);
        self.set_pages(0, 0);
        self.vram.clear_planes(0x0f);
        self.load_color_table()?;
        self.select_palette_bank(2)?;
        let entity_file = self.mem.cstr(ds_ptr(ENTITY_FILE));
        self.entities_load(&entity_file)?;
        for slot in 0..10u16 {
            let name = format!("flag{slot:02}.dat1");
            self.mem
                .strcpy_bytes(ds_ptr(SAVE_NAMES + slot * 16), name.as_bytes());
        }
        Ok(())
    }

    /// `sub_21CE6`: capture the 8x16 half-width font drawn by kana.pdt1.
    fn capture_ank_font(&mut self) {
        let mut out = ANK_FONT_DATA;
        let mut src = 0usize;
        for _ in 0..0xa0 {
            for row in 0..16 {
                let value = self.vram.get(0, src + row * 80);
                self.set_b(out, value);
                out += 1;
            }
            src += 1;
            if src == 0x50 {
                src = 0x500;
            }
        }
        self.mem.set_d(ANK_FONT, ds_ptr(ANK_FONT_DATA));
    }

    /// `sub_1F726`: COLOR.TBL banks.
    fn load_color_table(&mut self) -> Result<()> {
        let name = self.mem.cstr(ds_ptr(0x7e0));
        let bytes = self.read_resource(&name)?;
        let banks = (bytes.len() / 0x20).min(10);
        self.set_w(PALETTE_BANK_COUNT, banks as u16);
        self.mem
            .write_bytes(ds_ptr(PALETTE_BANKS), &bytes[..banks * 0x20]);
        Ok(())
    }

    /// `sub_14A52`: run MES files until one returns something other than a
    /// J3/load restart.
    pub fn run(&mut self) -> Result<()> {
        self.init()?;
        if let Some(name) = self.start_override.take() {
            self.mem.strcpy_bytes(ds_ptr(NEXT_MES_NAME), &name);
        }
        loop {
            let name = self.mem.cstr(ds_ptr(NEXT_MES_NAME));
            match self.call_mes(&name) {
                Ok(5) => continue,
                Ok(_) => return Ok(()),
                Err(error) if error.downcast_ref::<QuitRequested>().is_some() => return Ok(()),
                Err(error) => return Err(error),
            }
        }
    }

    // ---- save data ----------------------------------------------------------

    fn save_name(&self, slot: u16) -> Vec<u8> {
        self.mem.cstr(ds_ptr(SAVE_NAMES + slot * 16))
    }

    /// `sub_17A5F`: U6 save.
    fn save_game(&mut self, slot: u16, comment: FarPtr) -> Result<()> {
        self.obj_close_all(2)?;
        let mut out = self.mem.read_bytes(comment, SAVE_COMMENT_LEN);
        out.extend(self.mem.read_bytes(ds_ptr(GLOBAL_SLOTS), 0x54));
        out.extend(self.w(ENTITY_COUNT).to_le_bytes());
        let count = usize::from(self.w(ENTITY_COUNT)) * ENTITY_SIZE as usize;
        out.extend(self.mem.read_bytes(self.mem.d(ENTITIES), count));
        out.extend(self.mem.read_bytes(ds_ptr(INDEXED_5), 0x100));
        out.extend(self.mem.read_bytes(ds_ptr(INDEXED_8), 0x100));
        out.extend(self.mem.read_bytes(ds_ptr(l(0x27142)), 2));
        out.extend(self.mem.read_bytes(ds_ptr(NEXT_MES_NAME), 0x0e));
        let saved: Vec<FarPtr> = (0..self.obj_count())
            .map(|i| self.obj_at(i))
            .filter(|obj| (1..=0x95).contains(&self.ob(*obj, 0x2b)))
            .collect();
        for _ in 0..2 {
            out.extend((saved.len() as u16).to_le_bytes());
            for obj in &saved {
                out.extend(self.mem.read_bytes(*obj, OBJ_SIZE));
            }
        }
        let name = self.save_name(slot);
        self.write_data_file(&name, &out)
    }

    /// `sub_17827`: U7 load.  Returns true when a save was restored.
    fn load_game(&mut self, slot: u16) -> Result<bool> {
        let name = self.save_name(slot);
        let Some(bytes) = self.read_data_file(&name).filter(|b| !b.is_empty()) else {
            return Ok(false);
        };
        self.fade_to_bank(5, 2)?;
        self.obj_close_all(1)?;
        self.reset_state();
        let mut at = SAVE_COMMENT_LEN;
        let mut take = |n: usize| {
            let chunk = bytes
                .get(at..(at + n).min(bytes.len()))
                .unwrap_or(&[])
                .to_vec();
            at += n;
            chunk
        };
        let slots = take(0x54);
        self.mem.write_bytes(ds_ptr(GLOBAL_SLOTS), &slots);
        let count = take(2);
        let count = u16::from_le_bytes([
            count.first().copied().unwrap_or(0),
            count.get(1).copied().unwrap_or(0),
        ]);
        self.set_w(ENTITY_COUNT, count);
        let size = usize::from(count) * ENTITY_SIZE as usize;
        let list = self.mem.d(ENTITIES);
        let list = self.mem.realloc(list, size);
        self.mem.set_d(ENTITIES, list);
        let entities = take(size);
        self.mem.write_bytes(list, &entities);
        let t5 = take(0x100);
        self.mem.write_bytes(ds_ptr(INDEXED_5), &t5);
        let t8 = take(0x100);
        self.mem.write_bytes(ds_ptr(INDEXED_8), &t8);
        let w = take(2);
        self.mem.write_bytes(ds_ptr(l(0x27142)), &w);
        let next = take(0x0e);
        self.mem.write_bytes(ds_ptr(NEXT_MES_NAME), &next);
        let cursor = self.cursor_show(0);
        let objects = take(2);
        let objects = u16::from_le_bytes([
            objects.first().copied().unwrap_or(0),
            objects.get(1).copied().unwrap_or(0),
        ]);
        self.set_w(OBJ_COUNT, 0);
        for _ in 0..objects {
            let index = self.obj_count();
            self.set_w(OBJ_COUNT, (index + 1) as u16);
            let obj = self.obj_at(index);
            let record = take(OBJ_SIZE);
            self.mem.write_bytes(obj, &record);
            for off in [0, 0x2e, 0x36, 0x32, 0x4e, 0x56] {
                self.set_od(obj, off, 0);
            }
            let f = self.ow(obj, 0x28) & 0x00fe;
            self.set_ow(obj, 0x28, f);
        }
        let templates = take(2);
        let templates = u16::from_le_bytes([
            templates.first().copied().unwrap_or(0),
            templates.get(1).copied().unwrap_or(0),
        ]);
        self.set_w(TEMPLATE_COUNT, templates);
        let size = usize::from(templates) * OBJ_SIZE;
        let old = self.mem.d(TEMPLATES);
        let table = self.mem.realloc(old, size.max(1));
        self.mem.set_d(TEMPLATES, table);
        let data = take(size);
        self.mem.write_bytes(table, &data);
        self.cursor_show(cursor);
        Ok(true)
    }

    /// `sub_17774`: UJ save slot summary.
    fn save_info(&mut self, slot: u16) -> u16 {
        let name = self.save_name(slot);
        let engine_name = Self::engine_name(&name);
        let data = self.read_data_file(&name).filter(|b| !b.is_empty());
        let Some(data) = data else {
            self.set_b(SAVE_DATE, 0);
            self.set_b(SAVE_COMMENT, 0);
            return 0;
        };
        let comment = data.get(..SAVE_COMMENT_LEN).unwrap_or(&data).to_vec();
        self.mem.write_bytes(ds_ptr(SAVE_COMMENT), &comment);
        let stamp = self
            .game
            .resolve(&engine_name)
            .and_then(|path| std::fs::metadata(path).ok())
            .and_then(|meta| meta.modified().ok())
            .map(|time| {
                use chrono::{Datelike, Timelike};
                let local: chrono::DateTime<chrono::Local> = time.into();
                format!(
                    "{:02}/{:02} {:02}:{:02}",
                    local.month(),
                    local.day(),
                    local.hour(),
                    local.minute()
                )
            })
            .unwrap_or_default();
        self.mem.strcpy_bytes(ds_ptr(SAVE_DATE), stamp.as_bytes());
        1
    }

    // ---- backgrounds --------------------------------------------------------

    /// `sub_1EB9E(name)`: W5 / UE background transition.
    pub fn show_background(&mut self, name: Option<&[u8]>) -> Result<()> {
        let cursor = self.cursor_show(0);
        let has_name = name.is_some_and(|n| n.first().copied().unwrap_or(0) != 0);
        if let Some(name) = name.filter(|_| has_name) {
            self.access_back();
            self.pdt_show(name)?;
            let mut stored = trim_name(name);
            stored.truncate(13);
            stored.resize(13, b' ');
            self.mem.strcpy_bytes(ds_ptr(BG_NAME), &stored);
            self.access_front();
        }
        let mode = self.w(W_29000);
        match mode {
            0 => {
                self.fade_to_bank(3, 2)?;
                let display = self.w(DISPLAY_PAGE);
                self.copy_page(display ^ 1, display, 0x0f);
                self.fade_to_bank(3, 1)?;
            }
            1 => {
                self.select_palette_bank(1)?;
                self.dissolve(0x50, 0x190, 0, 0)?;
            }
            2 | 3 => {
                if mode == 2 {
                    self.select_palette_bank(1)?;
                }
                let map = self.mem.d(CELL_MAPS + ((self.w(CELL_MAP_SEL) ^ 1) & 1) * 4);
                for cell in 0..1000 {
                    let p = ptr_add(map, cell);
                    if self.mem.rb(p) == 0xf0 {
                        self.mem.wb(p, 0xf1);
                    }
                }
            }
            _ => {}
        }
        if has_name && mode < 4 {
            self.compose(1)?;
        }
        self.cursor_show(cursor);
        Ok(())
    }

    /// `sub_16E46(name, x, y)`: composite a sprite PDT (colour 8
    /// transparent) onto the background page.
    fn background_sprite(&mut self, name: &[u8], x: u16, y: u16) -> Result<()> {
        let cursor = self.cursor_show(0);
        self.access_back();
        let buf = self.load_file(name)?;
        let rect = ptr_add(buf, 0x23);
        if x != 1000 {
            let w = self
                .mem
                .rw(ptr_add(rect, 4))
                .wrapping_sub(self.mem.rw(rect));
            if (x.wrapping_add(w) as i16) < 0x50 {
                self.mem.ww(rect, x);
                self.mem.ww(ptr_add(rect, 4), w.wrapping_add(x));
            }
        }
        if y != 1000 {
            let h = self
                .mem
                .rw(ptr_add(rect, 6))
                .wrapping_sub(self.mem.rw(ptr_add(rect, 2)));
            if (y.wrapping_add(h) as i16) < 0x18f {
                self.mem.ww(ptr_add(rect, 2), y);
                self.mem.ww(ptr_add(rect, 6), h.wrapping_add(y));
            }
        }
        let left = self.mem.rw(rect);
        let right = self.mem.rw(ptr_add(rect, 4));
        let top = self.mem.rw(ptr_add(rect, 2));
        let bottom = self.mem.rw(ptr_add(rect, 6));
        self.set_w(PDT_LEFT, left);
        self.set_w(PDT_RIGHT, right);
        self.set_w(PDT_TOP, top);
        self.set_w(PDT_BOTTOM, bottom);
        let page = self.w(ACCESS_PAGE);
        let saved = self.gbuf_alloc_get(
            i32::from(left),
            i32::from(top),
            i32::from(right),
            i32::from(bottom),
            page,
        );
        self.pdt_decode(buf);
        self.gbuf_merge_sprite(saved);
        let page = self.w(ACCESS_PAGE);
        self.gbuf_put(i32::from(left), i32::from(top), saved, page);
        self.mem.free(saved);
        self.mem.free(buf);
        self.access_front();
        self.cursor_show(cursor);
        Ok(())
    }

    // ---- music ---------------------------------------------------------------

    /// `sub_22AB8`: M0 — load a score with the configured extension and
    /// start it.
    fn music_load(&mut self, name: &[u8]) -> Result<()> {
        let drivers = self.w(l(0x25DC6));
        if drivers == 0 {
            return Ok(());
        }
        let mut file = trim_name(name);
        let midi = drivers & 1 != 0;
        if let Some(dot) = file.iter().position(|&b| b == b'.') {
            let ext = if midi {
                self.mem.cstr(ds_ptr(l(0x25DCC)))
            } else {
                self.mem.cstr(ds_ptr(l(0x25DD0)))
            };
            let tail = file[dot + 1..].to_vec();
            file.truncate(dot + 1);
            for (i, c) in ext.iter().take(3).enumerate() {
                let _ = i;
                file.push(*c);
            }
            if tail.len() > 3 {
                file.extend_from_slice(&tail[3..]);
            }
        }
        let data = match self.read_resource(&file) {
            Ok(data) => data,
            Err(error) => {
                log::warn!("{error:#}");
                return Ok(());
            }
        };
        let name = Self::engine_name(&file);
        self.music(MusicCommand::Load { name, data, midi });
        self.music(MusicCommand::Start);
        Ok(())
    }

    // ---- remaining opcodes ------------------------------------------------------

    fn store(&mut self, dst: FarPtr, value: u16) {
        self.mem.ww(dst, value);
    }

    pub(crate) fn dispatch_service(&mut self, a: u8, b: u8) -> Result<u16> {
        let mut status = 0u16;
        match (a, b) {
            // ---- A: EMS effect sequencer (inactive without EMS) ----------------
            (b'A', b'0') => {
                let _ = self.operand()?;
                let cursor = self.w(CURSOR_SHOWN);
                self.set_w(l(0x264C2), cursor);
            }
            (b'A', b'1') => {
                let cursor = self.w(l(0x264C2));
                self.cursor_show(cursor);
            }
            (b'A', b'2') => {
                for _ in 0..6 {
                    let _ = self.operand()?;
                }
            }
            (b'A', b'3') | (b'A', b'4') => {
                self.expr_list(0x14)?;
            }
            (b'A', b'5') => {
                let _ = self.operand()?;
                self.set_w(TIMER_A, 0);
            }
            (b'A', b'6') => {}
            // ---- D: debugging -------------------------------------------------
            (b'D', b'0') => self.text_command(1)?,
            (b'D', b'1') => {
                let _ = self.expr_word()?;
                let name = self.expression()?;
                self.free_expr(name);
            }
            // ---- F: maps ------------------------------------------------------
            (b'F', b'0') => {
                let name = self.read_name(false)?;
                let map = self.map_load(&name, 0)?;
                self.mem.set_d(LAST_MAP, map);
                self.last_map_name = trim_name(&name);
            }
            (b'F', b'G') => {
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                let map = self.map_load(&name, 1)?;
                self.mem.set_d(LAST_MAP, map);
            }
            (b'F', b'1') => {
                let id = self.expr_word()?;
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                self.chip_view(id, &name)?;
            }
            (b'F', b'4') => {
                let x = self.operand()?;
                let y = self.operand()?;
                let m = self.operand()?;
                let id = self.expr_word()?;
                let dir = self.operand()?;
                let place = self.operand()?;
                let (vx, vy, vm, vd, vp) = self.entity_query(id);
                if vp != 2 {
                    self.store(x, vx);
                    self.store(y, vy);
                    self.store(m, vm);
                    self.store(dir, vd);
                }
                self.store(place, vp);
            }
            (b'F', b'5') => {
                let id = self.expr_word()?;
                let map = self.mem.d(LAST_MAP);
                let name = self.last_map_name.clone();
                self.map_attach(id, map, &name)?;
            }
            (b'F', b'6') => {
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                self.entities_load(&name)?;
            }
            (b'F', b'7') => {
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                self.entities_save(&name)?;
            }
            (b'F', b'8') => {
                let id = self.expr_word()?;
                self.obj_close(id, 0)?;
                let obj = self.obj_ptr(id);
                self.free_obj_map(obj);
            }
            (b'F', b'9') | (b'F', b'J') => {
                let [x, y, id] = self.expr_array::<3>()?;
                let obj = self.obj_ptr(id);
                if obj != 0 {
                    if b == b'9' {
                        let sx = i32::from(x as i16) - self.osw(obj, 0x12) / 2;
                        let sy = i32::from(y as i16) - self.osw(obj, 0x14) / 2;
                        self.set_osw(obj, 0x22, sx);
                        self.set_osw(obj, 0x24, sy);
                    }
                    self.map_center_view(obj, x, y);
                }
            }
            (b'F', b'A') => {
                let values = self.expr_array::<5>()?;
                self.entity_update(values);
            }
            (b'F', b'B') => {
                let [x, y, id, chip, layer] = self.expr_array::<5>()?;
                let obj = self.obj_ptr(id);
                if obj != 0 {
                    let map = self.od(obj, 0x4e);
                    self.map_set_tile(x, y, map, chip, layer);
                }
            }
            (b'F', b'C') | (b'F', b'D') => {
                let [id, trigger] = self.expr_array::<2>()?;
                self.map_set_trigger(id, trigger, u8::from(b == b'D'));
            }
            (b'F', b'E') => {
                let dst = self.operand()?;
                let count = self.obj_count();
                let obj = if count >= 1 {
                    self.obj_at(count - 1)
                } else {
                    0
                };
                let value = if count < 1 || self.od(obj, 0x4e) == 0 {
                    0
                } else {
                    u16::from(self.ob(self.od(obj, 0x4e), 0x27))
                };
                self.store(dst, value);
            }
            (b'F', b'H') => {
                let [x, y] = self.expr_array::<2>()?;
                let result = self.operand()?;
                let cx = self.operand()?;
                let cy = self.operand()?;
                let entity = self.operand()?;
                let trigger = self.operand()?;
                let (id, vx, vy, ve, vt) = self.map_probe(x, y);
                if id != 0 {
                    self.store(cx, vx);
                    self.store(cy, vy);
                    self.store(entity, ve);
                    self.store(trigger, vt);
                }
                self.store(result, id);
            }
            (b'F', b'I') => {
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                self.load_overlay_chips(&name)?;
            }
            (b'F', b'K') => {
                let index = if self.w(l(0x2714C)) != 0 {
                    -1
                } else {
                    self.obj_count() - 1
                };
                self.map_frame(index)?;
            }
            // ---- I: battles ---------------------------------------------------
            (b'I', b'0') => {
                let name = self.read_name(false)?;
                let values = self.expr_array::<8>()?;
                self.battle_start(&name, values, values[7])?;
            }
            (b'I', b'1') => self.battle_end()?,
            (b'I', b'2') => self.battle_panels(1000)?,
            (b'I', b'3') => {
                let dst = self.operand()?;
                let [mode] = self.expr_array::<1>()?;
                let result = self.battle_round(mode)?;
                self.store(dst, result);
            }
            (b'I', b'4') => self.battle_heal(),
            (b'I', b'5') => {
                let dst = self.operand()?;
                let result = self.nearby_npc();
                self.store(dst, result);
            }
            // ---- M: music -----------------------------------------------------
            (b'M', b'0') => {
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                self.music_load(&name)?;
            }
            (b'M', b'1') => {
                let [volume] = self.expr_array::<1>()?;
                let volume = volume as i16;
                let volume = if (0..=0x7f).contains(&volume) {
                    volume
                } else {
                    0x7f
                };
                self.music(MusicCommand::Fade(volume as u8));
            }
            (b'M', b'2') => self.music(MusicCommand::Stop),
            (b'M', b'3') => {}
            (b'M', b'4') => self.music(MusicCommand::Start),
            (b'M', b'5') => {
                self.expr_array::<2>()?;
            }
            // ---- U: system ----------------------------------------------------
            (b'U', b'0') => {
                let state = self.expr_word()?;
                self.cursor_show(state);
            }
            (b'U', b'1') => {
                let [wait, bank] = self.expr_array::<2>()?;
                self.fade_to_bank(wait, bank)?;
            }
            (b'U', b'2') => {
                self.set_w(W_27144, 0);
                self.text_command(0)?;
            }
            (b'U', b'3') => {
                let [page, palette] = self.expr_array::<2>()?;
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                let cursor = self.cursor_show(0);
                let display = self.w(DISPLAY_PAGE);
                self.set_pages(display, page);
                let saved = self.w(PDT_PALETTE);
                self.set_w(PDT_PALETTE, palette);
                self.pdt_show(&name)?;
                self.set_w(PDT_PALETTE, saved);
                self.set_pages(display, display);
                self.cursor_show(cursor);
            }
            (b'U', b'5') => {
                let dst = self.operand()?;
                let range = self.expr_word()?;
                let value = self.rand_n(i32::from(range as i16));
                self.store(dst, value as u16);
            }
            (b'U', b'6') => {
                let slot = self.expr_word()?;
                let comment = self.expression()?;
                if slot >= 10 {
                    return Err(self.fatal(0x10));
                }
                self.save_game(slot, comment)?;
                self.free_expr(comment);
            }
            (b'U', b'7') => {
                let slot = self.expr_word()?;
                if slot >= 10 {
                    return Err(self.fatal(0x10));
                }
                status = if self.load_game(slot)? { 5 } else { 0 };
            }
            (b'U', b'8') => {
                let [display, access] = self.expr_array::<2>()?;
                if display < 2 && access < 2 {
                    self.set_pages(display, access);
                }
            }
            (b'U', b'9') => {
                let dst = self.operand()?;
                let value = u16::from(self.shift_state() & 1);
                self.store(dst, value);
            }
            (b'U', b'B') => {
                let dst = self.operand()?;
                let ptr = self.operand()?;
                let name = self.mem.cstr(ptr);
                let missing = !self.resource_exists(&name);
                self.store(dst, u16::from(missing));
            }
            (b'U', b'D') => {
                let shape = self.expr_word()?;
                let ptr = self.operand()?;
                if shape >= 6 {
                    return Err(self.fatal(0x13));
                }
                let name = self.mem.cstr(ptr);
                let data = self.read_resource(&name)?;
                let len = data.len().min(usize::from(CURSOR_SIZE));
                self.mem
                    .write_bytes(ds_ptr(CURSOR_DATA + shape * CURSOR_SIZE), &data[..len]);
            }
            (b'U', b'E') => {
                let mode = self.expr_word()?;
                self.set_w(W_29000, mode);
                loop {
                    let name = self.expression()?;
                    if name == 0 {
                        break;
                    }
                    let [x, y] = self.expr_array::<2>()?;
                    let bytes = self.mem.cstr(name);
                    self.background_sprite(&bytes, x, y)?;
                    self.free_expr(name);
                }
                self.show_background(None)?;
            }
            (b'U', b'H') => {
                let planes = self.expr_word()?;
                let cursor = self.cursor_show(0);
                self.vram.clear_planes(planes);
                self.cursor_show(cursor);
            }
            (b'U', b'I') => {
                let v = self.expr_array::<6>()?;
                self.invert_cells(
                    i32::from(v[0] as i16),
                    i32::from(v[1] as i16),
                    i32::from(v[2] as i16),
                    i32::from(v[3] as i16),
                    v[4],
                    v[5],
                );
            }
            (b'U', b'J') => {
                let slot = self.expr_word()?;
                let dst = self.operand()?;
                if slot >= 10 {
                    return Err(self.fatal(0x10));
                }
                let result = self.save_info(slot);
                self.store(dst, result);
            }
            (b'U', b'K') => {
                let shape = self.expr_word()?;
                if shape < 6 {
                    let cursor = self.cursor_show(0);
                    self.set_w(CURSOR_SHAPE, shape);
                    self.set_w(CURSOR_FRAME, 0);
                    self.cursor_show(cursor);
                }
            }
            // ---- W: windows ---------------------------------------------------
            (b'W', b'1') => {
                let v = self.expr_array::<7>()?;
                self.window_define(
                    i32::from(v[0] as i16),
                    i32::from(v[1] as i16),
                    i32::from(v[2] as i16),
                    i32::from(v[3] as i16),
                    v[4],
                    v[5],
                    v[6],
                )?;
            }
            (b'W', b'2') => {
                self.wait_key_or_click()?;
                self.wait_all_released()?;
            }
            (b'W', b'3') => {
                let id = self.expr_word()?;
                self.obj_raise(id)?;
                self.obj_clear(-1)?;
            }
            (b'W', b'4') => {
                let [id, mode] = self.expr_array::<2>()?;
                let index = self.obj_index(id);
                if index >= 0 {
                    let obj = self.obj_at(index);
                    let f = self.ow(obj, 0x28) | flag::OWN_TEXT;
                    self.set_ow(obj, 0x28, f);
                    self.obj_close(id, mode)?;
                }
            }
            (b'W', b'5') => {
                let ptr = self.operand()?;
                let mode = self.expr_word()?;
                self.set_w(W_29000, mode);
                let name = self.mem.cstr(ptr);
                self.show_background(Some(&name))?;
            }
            (b'W', b'6') => {
                let id = self.expr_word()?;
                let ptr = self.operand()?;
                let keep = self.expr_word()?;
                let name = self.mem.cstr(ptr);
                self.image_window(id, &name, keep)?;
            }
            (b'W', b'7') => {
                let id = self.expr_byte()?;
                self.set_b(WAIT_OBJECT, id);
                self.set_w(WAIT_MODE, 0);
            }
            (b'W', b'8') => self.menu_command()?,
            (b'W', b'9') => {
                let id = self.expr_word()?;
                self.window_clone(1000, 1000, 1000, 1000, 1000, id)?;
            }
            (b'W', b'A') => {
                self.set_w(W_2901A, 0);
                let id = self.expr_word()?;
                self.obj_raise(id)?;
            }
            (b'W', b'B') => {
                let [id, direction] = self.expr_array::<2>()?;
                if self.w(TOP_OBJECT) != id {
                    self.obj_raise(id)?;
                }
                let cursor = self.cursor_show(0);
                self.text_scroll_request(direction)?;
                self.cursor_show(cursor);
            }
            (b'W', b'C') => {
                let id = self.expr_word()?;
                let index = self.obj_index(id);
                if index >= 0 {
                    self.obj_redraw(index)?;
                }
            }
            (b'W', b'D') => {
                let mode = self.expr_word()?;
                for i in 0..self.obj_count() {
                    let obj = self.obj_at(i);
                    let f = self.ow(obj, 0x28) | flag::OWN_TEXT;
                    self.set_ow(obj, 0x28, f);
                }
                self.obj_close_all(mode)?;
            }
            (b'W', b'E') => {
                let mode = self.expr_word()?;
                self.compose(mode)?;
            }
            (b'W', b'F') => {
                let id = self.expr_byte()?;
                self.set_b(WAIT_OBJECT, id);
                self.set_b(WAIT_FLAG, 1);
                self.set_w(WAIT_MODE, 0);
            }
            (b'W', b'G') => {
                for i in 0..self.obj_count() {
                    let obj = self.obj_at(i);
                    let f = self.ow(obj, 0x28) | flag::VISIBLE;
                    self.set_ow(obj, 0x28, f);
                }
                self.compose(1)?;
            }
            (b'W', b'H') => {
                let [id, x1, y1, x2, y2] = self.expr_array::<5>()?;
                let obj = self.obj_ptr(id);
                if obj != 0 {
                    self.set_ow(obj, 0x16, x1.wrapping_mul(2));
                    self.set_ow(obj, 0x18, y1.wrapping_mul(16));
                    self.set_ow(obj, 0x1a, x2.wrapping_mul(2));
                    self.set_ow(obj, 0x1c, y2.wrapping_mul(16));
                }
            }
            (b'W', b'I') => {
                self.set_w(WAIT_MODE, 1);
                let id = self.expr_byte()?;
                self.set_b(WAIT_OBJECT, id);
            }
            (b'W', b'J') => {
                let dst = self.operand()?;
                let id = self.expr_word()?;
                let _name = self.operand()?;
                let result = self.image_select(id)?;
                self.store(dst, result);
            }
            (b'W', b'K') => {
                let v = self.expr_array::<6>()?;
                self.window_clone(v[0], v[1], v[2], v[3], v[4], v[5])?;
            }
            (b'W', b'L') => {
                let v = self.expr_array::<4>()?;
                self.mark_background_dirty(
                    i32::from(v[0] as i16),
                    i32::from(v[1] as i16),
                    i32::from(v[2] as i16),
                    i32::from(v[3] as i16),
                );
                self.compose(2)?;
            }
            _ => {
                return Err(self.fatal(4));
            }
        }
        Ok(status)
    }

    /// `sub_1E443` (WJ): pick a 4x32 cell from an image window with the
    /// mouse.  Unused by the supplied game; implemented as a click on the
    /// window returning the cell index.
    fn image_select(&mut self, id: u16) -> Result<u16> {
        while self.w(LEFT_HELD) != 0 {
            self.idle()?;
        }
        self.set_w(LEFT_PRESSES, 0);
        let mut obj = self.obj_ptr(id);
        if obj == 0 {
            obj = self.window_default(id)?;
        }
        self.obj_raise(id)?;
        let per_row = self.osw(obj, 0x12) / 4;
        loop {
            if self.w(LEFT_PRESSES) != 0 {
                let col = (i32::from(self.sw(MOUSE_X)) / 8) & !1;
                let y = i32::from(self.sw(MOUSE_Y)) & !0xf;
                self.set_w(LEFT_PRESSES, 0);
                if self.osw(obj, 0x0a) <= col
                    && self.osw(obj, 0x0e) > col
                    && self.osw(obj, 0x0c) <= y
                    && self.osw(obj, 0x10) > y
                {
                    let cx = (col - self.osw(obj, 0x0a)) / 4;
                    let cy = (y - self.osw(obj, 0x0c)) / 32;
                    while self.w(LEFT_HELD) != 0 {
                        self.idle()?;
                    }
                    return Ok((cy * per_row.max(1) + cx) as u16);
                }
            }
            if self.w(RIGHT_PRESSES) != 0 {
                self.set_w(RIGHT_PRESSES, 0);
                return Ok(0xffff);
            }
            self.idle()?;
        }
    }
}
