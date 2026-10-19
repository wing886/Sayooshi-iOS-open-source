//! Real-time map battles (seg013): `I0` start, `I3` round, `I1` end,
//! `I2` HP panels, `I4` full heal, plus the enemy AI helpers.
//!
//! Battle slots live at DS:391C (13 x 0x1D bytes): `+0` entity id, `+2`
//! "showing hit sprite" flag, `+4` HP, `+6` FIGHT.TAB record, `+8` entity
//! pointer, `+0C` saved 17-byte entity.  Slots 0-3 are the party, 4-12 are
//! three enemy groups of three.  FIGHT.TAB records are 0x29 bytes: name at
//! `+0`, `+1B` player-controlled flag, `+1D` AI kind, `+1F` level, `+21`
//! current HP, `+23` maximum HP.

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::map::ENTITY_SIZE;
use super::mem::{FarPtr, ds_ptr, ptr_add};
use super::text::PrintfArg;

const SLOTS: u16 = 0x391C;
const SLOT_SIZE: u16 = 0x1D;
const PANEL_TIMERS: u16 = 0x3910;
const FIGHT_TAB: u16 = l(0x270E7);
const FIGHT_COUNT: u16 = l(0x270E5);
const LEVEL_TAB: u16 = l(0x270F9);
const LEVEL_COUNT: u16 = l(0x270F7);
const ENTITY_BACKUP: u16 = l(0x270F3);
const PARTY_PANEL: u16 = l(0x270EF);
const ENEMY_PANEL: u16 = l(0x270EB);
const BATTLE_MAP: u16 = l(0x270DF);
const EVENT_CHANCE: u16 = l(0x270E3);
const DIFFICULTY: u16 = l(0x26F1E);
const ROUND_MODE: u16 = l(0x26F1A);
const AUTO_BATTLE: u16 = l(0x28EB1);
const BATTLE_MODE: u16 = l(0x27150);
const FIGHT_REC: i32 = 0x29;

impl Engine {
    fn slot(&self, index: u16) -> u16 {
        SLOTS + index * SLOT_SIZE
    }
    fn slot_w(&self, index: u16, off: u16) -> u16 {
        self.w(self.slot(index) + off)
    }
    fn slot_sw(&self, index: u16, off: u16) -> i32 {
        i32::from(self.sw(self.slot(index) + off))
    }
    fn set_slot_w(&mut self, index: u16, off: u16, value: u16) {
        let at = self.slot(index) + off;
        self.set_w(at, value);
    }
    fn fight_rec(&self, kind: u16) -> FarPtr {
        ptr_add(self.mem.d(FIGHT_TAB), i32::from(kind) * FIGHT_REC)
    }

    /// `sub_17FDE(x, y)`: free spot on the battle map.
    fn battle_spot_free(&self, x: i32, y: i32) -> bool {
        let list = self.mem.d(ENTITIES);
        let count = i32::from(self.w(ENTITY_COUNT));
        let mut hit = count;
        for i in 0..count {
            let e = ptr_add(list, i * ENTITY_SIZE);
            if self.ob(e, 0x0c) == 0xfe && self.osw(e, 0) == x && self.osw(e, 2) == y {
                hit = i;
                break;
            }
        }
        let map = self.mem.d(BATTLE_MAP);
        let tiles = self.od(map, 0x2e);
        let index = y * self.osw(map, 0) + x;
        let mut layer = (self.mem.rw(ptr_add(tiles, index * 2)) >> 9) & 3;
        if layer == 0 {
            layer = (self.mem.rw(ptr_add(tiles, (index + 1) * 2)) >> 9) & 3;
        }
        hit == count && layer == 0
    }

    /// `sub_17EF9(id, remove, x, y)`: place (0) / remove (1) a battle actor.
    fn battle_place(&mut self, id: u16, remove: u16, x: u16, y: u16) {
        let e = self.find_entity(id, 0);
        if e == 0 {
            return;
        }
        let mut slot = 13;
        for cx in 0..13 {
            if self.slot_w(cx, 6) != 0 && self.slot_w(cx, 0) == id {
                slot = cx;
                break;
            }
        }
        if slot < 13 {
            if remove == 0 {
                let at = self.slot(slot) + 8;
                self.mem.set_d(at, e);
            }
            if remove == 1 && self.slot_w(slot, 2) != 0 {
                self.mem.memcpy(e, ds_ptr(self.slot(slot) + 0x0c), 0x11);
            }
        }
        self.set_ow(e, 0, x);
        self.set_ow(e, 2, y);
        self.set_ob(e, 0x0c, if remove == 0 { 0xfe } else { 0xff });
        if (0xe6..=0xef).contains(&id) {
            let v = (self.ob(e, 0x0e) & 0xf0) + 3;
            self.set_ob(e, 0x0e, v);
        }
        if (0xdc..=0xe5).contains(&id) {
            let v = (self.ob(e, 0x0e) & 0xf0) + 2;
            self.set_ob(e, 0x0e, v);
        }
    }

    fn load_table(
        &mut self,
        name: &[u8],
        record: usize,
        ptr_slot: u16,
        count_slot: u16,
    ) -> Result<()> {
        let bytes = self.read_resource(name)?;
        self.set_w(count_slot, (bytes.len() / record) as u16);
        let buf = self.mem.alloc(bytes.len());
        self.mem.write_bytes(buf, &bytes);
        self.mem.set_d(ptr_slot, buf);
        Ok(())
    }

    /// `sub_1806E(map name, party/enemy kinds, enemies per group)`: I0.
    pub fn battle_start(&mut self, name: &[u8], kinds: [u16; 8], per_group: u16) -> Result<()> {
        let mut table = 0x4C0u16;
        while self.w(table) != 0 {
            let e = self.find_entity(self.w(table), 0);
            if e != 0 {
                let sprite = self.w(table + 2);
                self.set_ow(e, 4, sprite);
            }
            table += 4;
        }
        self.load_table(b"fight.tab1", FIGHT_REC as usize, FIGHT_TAB, FIGHT_COUNT)?;
        self.load_table(b"level.tab1", 2, LEVEL_TAB, LEVEL_COUNT)?;
        let count = usize::from(self.w(ENTITY_COUNT)) * ENTITY_SIZE as usize;
        let list = self.mem.d(ENTITIES);
        let backup = self.mem.alloc(count);
        self.mem.memcpy(backup, list, count);
        self.mem.set_d(ENTITY_BACKUP, backup);
        let party = self.obj_ptr(0x67);
        self.mem.set_d(PARTY_PANEL, party);
        let enemy = self.obj_ptr(0x66);
        self.mem.set_d(ENEMY_PANEL, enemy);
        let map = self.map_load(name, 1)?;
        self.mem.set_d(BATTLE_MAP, map);
        self.map_attach(6, map, name)?;
        self.mem.memset(ds_ptr(SLOTS), 0, 0x1b3);
        self.battle_sync_hp();
        self.mem.memset(ds_ptr(PANEL_TIMERS), 0, 0x0c);
        let mut party_count = 0;
        let mut sprite_id = 0u16;
        for di in 0..4u16 {
            let kind = kinds[usize::from(di)];
            if kind == 0 {
                continue;
            }
            sprite_id = match kind {
                1 => 0xe6,
                2 => 0xe7,
                3 => 0xe8,
                4 => 0xec,
                5 => 0xeb,
                _ => sprite_id,
            };
            let (x, y) = loop {
                let x = (self.rand_n(0x0f) + 2) & !1;
                let y = (self.rand_n(9) + 7) & !1;
                if self.battle_spot_free(x, y) {
                    break (x, y);
                }
            };
            party_count += 1;
            self.set_slot_w(di, 0, sprite_id);
            self.set_slot_w(di, 6, kind);
            let hp = self.ow(self.fight_rec(kind), 0x21);
            self.set_slot_w(di, 4, hp);
            self.battle_place(sprite_id, 0, x as u16, y as u16);
        }
        let mut id = 0xdcu16;
        let mut enemy_count = 0;
        self.set_w(l(0x26F1C), 0);
        for group in 0..3usize {
            let kind = kinds[group + 4];
            for di in 0..3u16 {
                if di < per_group && kind != 0 {
                    let (x, y) = loop {
                        let x = (self.rand_n(0x0f) + 0x11) & !1;
                        let y = (self.rand_n(9) + 7) & !1;
                        if self.battle_spot_free(x, y) {
                            break (x, y);
                        }
                    };
                    enemy_count += 1;
                    let slot = id - 0xdc + 4;
                    self.set_slot_w(slot, 0, id);
                    self.set_slot_w(slot, 6, kind);
                    let hp = self.ow(self.fight_rec(kind), 0x21);
                    self.set_slot_w(slot, 4, hp);
                    self.battle_place(id, 0, x as u16, y as u16);
                }
                id += 1;
            }
        }
        let difficulty = if enemy_count > party_count {
            (enemy_count - party_count) * 10 + 10
        } else {
            10
        };
        self.set_w(DIFFICULTY, difficulty as u16);
        for panel in [PARTY_PANEL, ENEMY_PANEL] {
            let obj = self.mem.d(panel);
            if obj == 0 {
                continue;
            }
            let buf = self.mem.alloc(300);
            self.set_od(obj, 0x2e, buf);
            self.set_od(obj, 0x32, buf);
            self.set_od(obj, 0x36, buf);
        }
        self.battle_panels(1000)?;
        self.obj_raise(6)?;
        self.set_w(FRAME_COUNTER, 0);
        self.map_frame(-1)?;
        Ok(())
    }

    /// `sub_18FA0`: copy party HP back into FIGHT.TAB.
    fn battle_sync_hp(&mut self) {
        for cx in 0..4 {
            let kind = self.slot_w(cx, 6);
            if kind != 0 {
                let hp = self.slot_w(cx, 4);
                let rec = self.fight_rec(kind);
                self.set_ow(rec, 0x21, hp);
            }
        }
    }

    /// `sub_18F76`: I4 — restore every FIGHT.TAB record to full HP.
    pub fn battle_heal(&mut self) {
        for kind in 1..=10 {
            let rec = self.fight_rec(kind);
            let max = self.ow(rec, 0x23);
            self.set_ow(rec, 0x21, max);
        }
    }

    /// `sub_184CD`: I1 — tear the battle down.
    pub fn battle_end(&mut self) -> Result<()> {
        for i in 0..6 {
            if self.w(PANEL_TIMERS + i * 2) != 0 {
                self.set_w(PANEL_TIMERS + i * 2, 1);
            }
        }
        self.battle_highlight(-1);
        let obj = self.obj_ptr(6);
        self.obj_close(6, 0)?;
        self.free_obj_map(obj);
        for id in 0xdc..=0xed {
            self.battle_place(id, 1, 0, 0);
        }
        let fight = self.mem.d(FIGHT_TAB);
        self.mem.free(fight);
        let level = self.mem.d(LEVEL_TAB);
        self.mem.free(level);
        for panel in [PARTY_PANEL, ENEMY_PANEL] {
            let obj = self.mem.d(panel);
            if obj == 0 {
                continue;
            }
            let buf = self.od(obj, 0x2e);
            self.mem.free(buf);
            self.set_od(obj, 0x2e, 0);
            self.set_od(obj, 0x32, 0);
            self.set_od(obj, 0x36, 0);
        }
        let backup = self.mem.d(ENTITY_BACKUP);
        let count = usize::from(self.w(ENTITY_COUNT)) * ENTITY_SIZE as usize;
        let list = self.mem.d(ENTITIES);
        self.mem.memcpy(list, backup, count);
        self.mem.free(backup);
        self.obj_raise(5)?;
        Ok(())
    }

    /// `sub_185FA(slot)`: highlight a HP panel row (negative: tick timers).
    fn battle_highlight(&mut self, cx: i32) {
        if cx == 3 {
            return;
        }
        let party = self.mem.d(PARTY_PANEL);
        let enemy = self.mem.d(ENEMY_PANEL);
        if cx >= 0 {
            let (row, timer, panel) = if cx < 4 {
                (cx, cx, party)
            } else {
                ((cx - 4) / 3, (cx - 4) / 3 + 3, enemy)
            };
            let at = PANEL_TIMERS + timer as u16 * 2;
            if self.w(at) != 0 {
                return;
            }
            self.set_w(at, 2);
            let id = u16::from(self.ob(panel, 0x2b));
            self.invert_cells(0, row * 2, 0x0c, row * 2, id, 0x0f);
            return;
        }
        for (base, panel) in [(0u16, party), (3, enemy)] {
            let id = u16::from(self.ob(panel, 0x2b));
            for row in 0..3u16 {
                let at = PANEL_TIMERS + (base + row) * 2;
                let v = self.w(at);
                if v != 0 {
                    self.set_w(at, v - 1);
                    if v == 1 {
                        self.invert_cells(
                            0,
                            i32::from(row) * 2,
                            0x0c,
                            i32::from(row) * 2,
                            id,
                            0x0f,
                        );
                    }
                }
            }
        }
    }

    /// `sub_186F7(slot)`: redraw the HP panels (1000 = no row refresh).
    pub fn battle_panels(&mut self, cx: u16) -> Result<()> {
        if cx == 3 {
            return Ok(());
        }
        if cx != 1000 {
            let row = if cx < 4 { cx } else { (cx - 4) / 3 };
            let column = if cx < 4 { 8u16 } else { 0x30 };
            let cursor = self.cursor_show(0);
            let temp = self.mem.alloc(128);
            for step in [0u16, 2, 4] {
                self.set_pages(0, 1);
                self.cells_grab(column + step, (row * 2 + 0x13) * 16, 2, 16, temp, 0);
                self.set_pages(0, 0);
                self.cell_put(column + step, (row * 2 + 0x13) * 16, temp, 0, 2);
            }
            self.mem.free(temp);
            self.cursor_show(cursor);
        }
        let format: &[u8] = b"\\C7%-26.26s\\r\\kHP%4d\\r";
        let party = self.mem.d(PARTY_PANEL);
        if party != 0 {
            let mut text = Vec::new();
            for slot in 0..3u16 {
                let kind = self.slot_w(slot, 6);
                let rec = self.fight_rec(kind);
                let hp = self.slot_w(slot, 4);
                if hp == 0 {
                    text.extend_from_slice(b"\\r\\r");
                } else {
                    text.extend(self.sprintf(format, &[PrintfArg::Far(rec), PrintfArg::Word(hp)]));
                }
            }
            self.panel_text(party, &text)?;
        }
        let enemy = self.mem.d(ENEMY_PANEL);
        if enemy != 0 {
            let mut text = Vec::new();
            for group in [4u16, 7, 10] {
                let kind = self.slot_w(group, 6);
                let rec = self.fight_rec(kind);
                let total =
                    (group..group + 3).fold(0u16, |sum, s| sum.wrapping_add(self.slot_w(s, 4)));
                if total == 0 {
                    text.extend_from_slice(b"\\r\\r");
                } else {
                    text.extend(
                        self.sprintf(format, &[PrintfArg::Far(rec), PrintfArg::Word(total)]),
                    );
                }
            }
            self.panel_text(enemy, &text)?;
        }
        Ok(())
    }

    fn panel_text(&mut self, obj: FarPtr, text: &[u8]) -> Result<()> {
        let buf = self.od(obj, 0x2e);
        let mut text = text.to_vec();
        text.truncate(299);
        self.mem.strcpy_bytes(buf, &text);
        let (ox, oy) = (self.ow(obj, 0x1e), self.ow(obj, 0x20));
        self.set_ow(obj, 0x22, ox);
        self.set_ow(obj, 0x24, oy);
        let index = self.obj_index(u16::from(self.ob(obj, 0x2b)));
        self.draw_text(index)
    }

    /// `sub_18937(e, dir, &x, &y)`.
    fn battle_step(&mut self, e: FarPtr, dir: u16) -> (i32, i32) {
        let (mut x, mut y) = (self.osw(e, 0), self.osw(e, 2));
        match dir {
            0 => y -= 1,
            1 => y += 1,
            2 => x -= 1,
            3 => x += 1,
            _ => {}
        }
        let v = (self.ob(e, 0x0e) & 0xf0).wrapping_add(dir as u8);
        self.set_ob(e, 0x0e, v);
        (x, y)
    }

    /// `sub_18995(e, tx, ty)`: direction towards a target.
    fn battle_toward(&mut self, e: FarPtr, tx: i32, ty: i32) -> u16 {
        let (mut tx, mut ty) = (tx, ty);
        let (ex, ey) = (self.osw(e, 0), self.osw(e, 2));
        if ex != tx && ey != ty {
            if (i32::from(self.rand() as i16) * 2) / 0x8000 == 0 {
                tx = ex;
            } else {
                ty = ey;
            }
        }
        let mut dir = u16::from(self.ob(e, 0x0e) & 0xf);
        if ex > tx {
            dir = 2;
        }
        if ex < tx {
            dir = 3;
        }
        if ey > ty {
            dir = 0;
        }
        if ey < ty {
            dir = 1;
        }
        dir
    }

    fn opponent_range(&self, e: FarPtr) -> (u16, u16) {
        if self.ow(e, 6) < 0xe6 {
            (0, 3)
        } else {
            (4, 0x0c)
        }
    }

    /// `sub_18A15`: chase the opponent with the lowest recorded HP.
    fn battle_ai_weakest(&mut self, e: FarPtr) -> (i32, i32) {
        let (first, last) = self.opponent_range(e);
        let mut best_hp = 1000i32;
        let mut best = 0u16;
        for s in first..=last {
            let kind = self.slot_w(s, 6);
            if self.slot_w(s, 4) == 0 {
                continue;
            }
            let hp = i32::from(self.ow(self.fight_rec(kind), 0x21) as i16);
            if hp < best_hp {
                best_hp = hp;
                best = s;
            }
        }
        let target = self.mem.d(self.slot(best) + 8);
        let dir = self.battle_toward(e, self.osw(target, 0), self.osw(target, 2));
        self.battle_step(e, dir)
    }

    /// Nearest opponent by x distance (the original compares x twice).
    fn battle_nearest(&self, e: FarPtr) -> (i32, i32) {
        let (first, last) = self.opponent_range(e);
        let (mut bx, mut by) = (1000i32, 1000i32);
        let ex = self.osw(e, 0);
        for s in first..=last {
            if self.slot_w(s, 4) == 0 {
                continue;
            }
            let t = self.mem.d(self.slot(s) + 8);
            if (ex - bx).abs() > (ex - self.osw(t, 0)).abs() {
                bx = self.osw(t, 0);
                by = self.osw(t, 2);
            }
        }
        (bx, by)
    }

    /// `sub_18AC5`: chase the nearest opponent.
    fn battle_ai_nearest(&mut self, e: FarPtr) -> (i32, i32) {
        let (tx, ty) = self.battle_nearest(e);
        let dir = self.battle_toward(e, tx, ty);
        self.battle_step(e, dir)
    }

    /// `sub_18BB0`: random walk.
    fn battle_ai_random(&mut self, e: FarPtr) -> (i32, i32) {
        let r = self.rand_n(0x50);
        let dir = match r {
            0..=4 => 0,
            5..=9 => 1,
            10..=14 => 2,
            15..=19 => 3,
            _ => u16::from(self.ob(e, 0x0e) & 0xf),
        };
        let step = self.battle_step(e, dir);
        if r > 0x28 {
            (self.osw(e, 0), self.osw(e, 2))
        } else {
            step
        }
    }

    /// `sub_18C46`: keep away from the nearest opponent.
    fn battle_ai_flee(&mut self, e: FarPtr) -> (i32, i32) {
        let (tx, ty) = self.battle_nearest(e);
        if (self.osw(e, 0) - tx).abs() > 3 && (self.osw(e, 2) - ty).abs() > 3 {
            return self.battle_ai_random(e);
        }
        let dir = self.battle_toward(e, tx, ty) ^ 1;
        self.battle_step(e, dir)
    }

    /// `sub_18D63`: random battle event.
    fn battle_event(&mut self) -> Result<()> {
        let chance = self.w(EVENT_CHANCE);
        if chance == 0 {
            return Ok(());
        }
        let r = self.rand_n(i32::from(chance as i16));
        let mut damage = 0;
        if r == 1 {
            self.fade_to_bank(0, 3)?;
            self.fade_to_bank(0, 1)?;
            damage = 7;
        } else if r < 10 {
            damage = 5;
            let sprite = if r < 5 { 0x1b0 } else { 0x1b4 };
            let mut places = Vec::new();
            for id in 0xf0u16..0xf5 {
                let x = self.rand_n(0x14) + 0x0a;
                let y = self.rand_n(0x0a) + 4;
                places.push((x, y));
                self.battle_place(id, 0, x as u16, y as u16);
                let e = self.find_entity(id, 0);
                if e != 0 {
                    self.set_ow(e, 4, sprite);
                    let v = (self.ob(e, 0x0f) & 0xf0) + 1;
                    self.set_ob(e, 0x0f, v);
                }
            }
            let view = self.obj_index(6);
            self.set_w(FRAME_COUNTER, 0);
            let mut loops = 0;
            loop {
                for id in 0xf0u16..0xf5 {
                    let e = self.find_entity(id, 0);
                    if e != 0 {
                        let v = self.ow(e, 4) ^ 2;
                        self.set_ow(e, 4, v);
                    }
                    if id == 0xf0 {
                        loops += 1;
                    }
                }
                while !self.map_frame(view)? {
                    self.idle()?;
                }
                if self.shift_state() & 1 != 0 || loops >= 2 {
                    break;
                }
            }
            for (i, id) in (0xf0u16..0xf5).enumerate() {
                let (x, y) = places[i];
                self.battle_place(id, 1, x as u16, y as u16);
            }
        }
        if damage != 0 {
            for s in 4..0x0d {
                if self.slot_w(s, 4) == 0 {
                    continue;
                }
                let hit = self.rand_n(3) + damage;
                let hp = self.slot_sw(s, 4) - hit;
                if hp <= 0 {
                    self.set_slot_w(s, 4, 0);
                    let id = self.slot_w(s, 0);
                    self.battle_place(id, 1, 0, 0);
                    self.set_w(FRAME_COUNTER, 0);
                    self.map_frame(-1)?;
                } else {
                    self.set_slot_w(s, 4, hp as u16);
                }
                self.battle_panels(s)?;
            }
        }
        Ok(())
    }

    /// `sub_18FD0(mode)`: I3 — run one battle frame.  Returns 1 victory,
    /// 2 defeat / retreat refused, 0 continue.
    pub fn battle_round(&mut self, mode: u16) -> Result<u16> {
        self.set_w(ROUND_MODE, mode);
        if mode == 3 {
            for s in 0..4 {
                let kind = self.slot_w(s, 6);
                let max = i32::from(self.ow(self.fight_rec(kind), 0x23) as i16);
                if self.slot_sw(s, 4) < max / 2 {
                    return Ok(2);
                }
            }
        }
        if (4..0x0d).all(|s| self.slot_w(s, 4) == 0) {
            return Ok(1);
        }
        self.set_w(EVENT_CHANCE, 0);
        let mut hero_alive = false;
        for s in 0..4 {
            if self.slot_w(0, 4) != 0 {
                hero_alive = true;
            }
            let kind = self.slot_w(s, 6);
            if self.slot_w(s, 4) != 0 && kind != 1 && kind != 3 {
                self.set_w(EVENT_CHANCE, 0x50);
            }
        }
        if !hero_alive {
            return Ok(2);
        }
        let view = self.obj_index(6);
        self.set_w(BATTLE_MODE, 2);
        if !self.map_frame(view)? {
            self.set_w(BATTLE_MODE, 1);
            return Ok(0);
        }
        self.battle_highlight(-1);
        let map = self.mem.d(BATTLE_MAP);
        let (ents, count) = (self.od(map, 0x32), i32::from(self.ob(map, 0x26)));
        for i in 0..count {
            let e = ptr_add(ents, i * ENTITY_SIZE);
            let divider = u16::from(self.ob(e, 0x0f) & 0xf);
            if divider != 0 && self.w(FRAME_COUNTER).is_multiple_of(divider) {
                let v = self.ow(e, 4) ^ 2;
                self.set_ow(e, 4, v);
            }
        }
        for di in 0..0x0du16 {
            let kind = self.slot_w(di, 6);
            let e = self.mem.d(self.slot(di) + 8);
            let rec = self.fight_rec(kind);
            if self.slot_w(di, 4) == 0 || e == 0 {
                continue;
            }
            let divider = u16::from(self.ob(e, 0x0f) & 0xf);
            if divider == 0 || !self.w(FRAME_COUNTER).is_multiple_of(divider) {
                continue;
            }
            let v = self.ow(e, 4) ^ 2;
            self.set_ow(e, 4, v);
            if self.slot_w(di, 2) == 1 {
                self.mem.memcpy(e, ds_ptr(self.slot(di) + 0x0c), 0x11);
                self.set_slot_w(di, 2, 0);
                continue;
            }
            let mut target: i32 = 0;
            if self.ow(rec, 0x1b) == 1 || self.b(AUTO_BATTLE) == 1 {
                let mut ai = i32::from(self.ow(rec, 0x1d) as i16);
                if ai >= 10 {
                    if i32::from(self.ow(rec, 0x21) as i16)
                        > i32::from(self.ow(rec, 0x23) as i16) / 3
                    {
                        ai -= 10;
                    } else {
                        ai = 3;
                    }
                }
                let (nx, ny) = loop {
                    if ai != 2 && self.rand_n(0x0a) == 0 {
                        ai = 2;
                    }
                    let (nx, ny) = match ai {
                        0 => self.battle_ai_weakest(e),
                        1 => self.battle_ai_nearest(e),
                        2 => self.battle_ai_random(e),
                        3 => self.battle_ai_flee(e),
                        _ => (self.osw(e, 0), self.osw(e, 2)),
                    };
                    if ai == 2 {
                        break (nx, ny);
                    }
                    let tiles = self.od(map, 0x2e);
                    let tile = self
                        .mem
                        .rw(ptr_add(tiles, (ny * self.osw(map, 0) + nx) * 2));
                    if (tile >> 9) & 3 == 0 {
                        break (nx, ny);
                    }
                    let turn = match self.ob(e, 0x0e) & 0xf {
                        0 => 3,
                        1 => 2,
                        3 => 1,
                        _ => 0,
                    };
                    let v = (self.ob(e, 0x0e) & 0xf0).wrapping_add(turn);
                    self.set_ob(e, 0x0e, v);
                    ai = 2;
                };
                target = self.entity_step(e, map, nx, ny);
            } else if self.player_move(e)? == 5 {
                target = i32::from(self.w(INTERACT_ID));
            }
            let valid = (di < 3 && target < 0xe6) || (di > 3 && target >= 0xe6);
            if target < 1 || !valid {
                continue;
            }
            let mut victim = 0u16;
            while victim < 0x0d {
                if i32::from(self.slot_w(victim, 0)) == target {
                    break;
                }
                victim += 1;
            }
            if victim >= 0x0d {
                continue;
            }
            let victim_rec = self.fight_rec(self.slot_w(victim, 6));
            self.battle_highlight(i32::from(victim));
            let damage = if mode == 1 && di > 3 {
                self.slot_sw(victim, 4) / 10
            } else if mode == 2 && di < 4 {
                0
            } else {
                let diff = i32::from(self.ow(victim_rec, 0x1f) as i16)
                    - i32::from(self.ow(rec, 0x1f) as i16);
                let max = i32::from(self.ow(victim_rec, 0x23) as i16);
                let mut d = if diff == 0 {
                    max / 0x0a
                } else if diff > 0 {
                    max / 0x0f
                } else {
                    max / 5
                };
                if di > 3 {
                    d = d * 10 / i32::from(self.sw(DIFFICULTY)).max(1);
                }
                if d == 0 { 1 } else { d }
            };
            let hp = self.slot_sw(victim, 4) - damage;
            self.set_slot_w(victim, 4, hp as u16);
            if self.slot_w(victim, 2) != 0 {
                let ve = self.mem.d(self.slot(victim) + 8);
                let v = self.ob(ve, 0x0e) & 0xf0;
                self.set_ob(ve, 0x0e, v);
            }
            if hp <= 0 {
                self.battle_place(target as u16, 1, 0, 0);
                self.set_slot_w(victim, 4, 0);
                self.obj_raise(6)?;
                self.set_w(FRAME_COUNTER, 0);
                self.map_frame(-1)?;
            }
            self.battle_panels(victim)?;
            self.battle_event()?;
            if self.slot_w(victim, 2) == 0 {
                self.set_slot_w(victim, 2, 1);
                let ve = self.mem.d(self.slot(victim) + 8);
                self.mem.memcpy(ds_ptr(self.slot(victim) + 0x0c), ve, 0x11);
                let sprite = if victim > 3 {
                    self.w(0x4FC + victim * 2)
                } else {
                    match self.slot_w(victim, 6) {
                        1 => 0xf0,
                        2 => 0xf4,
                        3 => 0xf8,
                        4 => 0xfc,
                        5 => 0x110,
                        _ => self.ow(ve, 4),
                    }
                };
                self.set_ow(ve, 4, sprite);
                let v = self.ob(ve, 0x0e) & 0xf0;
                self.set_ob(ve, 0x0e, v);
            }
        }
        self.set_w(BATTLE_MODE, 1);
        Ok(0)
    }
}
