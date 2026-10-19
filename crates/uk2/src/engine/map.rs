//! Maps and actors (seg015).
//!
//! A map is a 0x3A-byte header read from the `.MAP` file followed by its
//! tile words, 17-byte entities and 10-byte triggers.  Map views are window
//! objects whose content routine composes every visible 16x16 cell from the
//! base chip set (`dword_27110`), up to two actor sprites from the overlay
//! chip set (`dword_27114`) and the tile's intensity mask.
//!
//! Entity record: `+0` x, `+2` y, `+4` sprite base, `+6` id, `+0C` map id,
//! `+0D` AI, `+0E` height class (high nibble) / direction (low nibble),
//! `+0F` blocked directions (high) / speed divider (low), `+10` flags
//! (1/2 static, 4 player, 8 AI target).

use anyhow::Result;

use super::Engine;
use super::ds::*;
use super::mem::{FarPtr, ds_ptr, ptr_add};
use super::object::{content, flag};

pub const ENTITY_SIZE: i32 = 0x11;
const MAP_HEADER: usize = 0x3a;

// Scratch globals of the collision helpers.
const TILE_INDEX: u16 = l(0x27146);
const TILE_CHIP: u16 = l(0x27108);
const TILE_LAYER: u16 = l(0x27106);
const TILE_OCCUPIED: u16 = l(0x270FE);
const TILE_TRIGGER: u16 = l(0x27100);
const HIT_TYPE: u16 = l(0x27102);
const HIT_SPRITE: u16 = l(0x27104);
const HIT_ENTITY: u16 = l(0x2715A);
const HIT_TRIGGER: u16 = l(0x27156);
const MAP_NAME: u16 = l(0x2712A);
const MOVE_PREF: u16 = l(0x2714A);
const BATTLE_MODE: u16 = l(0x27150);
const MOVE_TARGET_X: u16 = l(0x2907A);
const MOVE_TARGET_Y: u16 = l(0x2907C);
const W_23B5C: u16 = l(0x23B5C);

impl Engine {
    fn entities(&self) -> (FarPtr, i32) {
        (self.mem.d(ENTITIES), i32::from(self.w(ENTITY_COUNT)))
    }

    fn map_entities(&self, map: FarPtr) -> (FarPtr, i32) {
        (self.od(map, 0x32), i32::from(self.ob(map, 0x26)))
    }

    /// `sub_1CC8D(name, own_tiles)`: load a MAP (F0 / FG).
    pub fn map_load(&mut self, name: &[u8], own_tiles: u16) -> Result<FarPtr> {
        let cursor = self.cursor_show(0);
        let mut padded = super::interp::trim_name(name);
        padded.truncate(13);
        padded.resize(13, b' ');
        self.mem.strcpy_bytes(ds_ptr(MAP_NAME), &padded);
        let bytes = self.read_resource(name)?;
        let map = self.mem.alloc(MAP_HEADER);
        let header = bytes.get(..MAP_HEADER).unwrap_or(&bytes).to_vec();
        self.mem.write_bytes(map, &header);
        let mut at = MAP_HEADER;
        let tile_bytes = usize::from(self.ow(map, 0x24));
        let tiles = if own_tiles == 0 {
            self.mem.d(SHARED_TILES)
        } else {
            self.mem.alloc(tile_bytes)
        };
        self.set_od(map, 0x2e, tiles);
        let chunk = bytes
            .get(at..(at + tile_bytes).min(bytes.len()))
            .unwrap_or(&[])
            .to_vec();
        self.mem.write_bytes(tiles, &chunk);
        at += tile_bytes;
        self.set_od(map, 0x32, 0);
        let entity_bytes = usize::from(self.ob(map, 0x26)) * ENTITY_SIZE as usize;
        if entity_bytes > 0 {
            let chunk = bytes
                .get(at..(at + entity_bytes).min(bytes.len()))
                .unwrap_or(&[])
                .to_vec();
            let p = self.mem.alloc(entity_bytes);
            self.mem.write_bytes(p, &chunk);
            self.set_od(map, 0x32, p);
        }
        at += entity_bytes;
        self.set_od(map, 0x36, 0);
        let trigger_bytes = usize::from(self.ob(map, 0x28)) * 10;
        if trigger_bytes > 0 {
            let chunk = bytes
                .get(at..(at + trigger_bytes).min(bytes.len()))
                .unwrap_or(&[])
                .to_vec();
            let p = self.mem.alloc(trigger_bytes);
            self.mem.write_bytes(p, &chunk);
            self.set_od(map, 0x36, p);
        }
        let (list, count) = self.entities();
        let mut player = 0;
        for i in 0..count {
            let e = ptr_add(list, i * ENTITY_SIZE);
            if self.ob(e, 0x10) & 4 != 0 {
                player = e;
                break;
            }
        }
        self.mem.set_d(PLAYER, player);
        self.set_w(FRAME_COUNTER, 0);
        self.set_w(W_27144, 0);
        self.access_back();
        if own_tiles == 0 {
            let base = self.mem.cstr(ptr_add(map, 4));
            let overlay = self.mem.cstr(ptr_add(map, 0x14));
            let base_buf = self.mem.d(CHIP_BASE);
            let overlay_buf = self.mem.d(CHIP_OVERLAY);
            self.pdt_to_cells(&base, base_buf)?;
            self.pdt_to_cells(&overlay, overlay_buf)?;
        }
        self.vram.clear_planes(0x0f);
        let bg = self.mem.cstr(ds_ptr(BG_NAME));
        self.pdt_show(&super::interp::trim_name(&bg))?;
        self.access_front();
        self.cursor_show(cursor);
        Ok(map)
    }

    /// `sub_1CBFD`: free an object's map.
    pub fn free_obj_map(&mut self, obj: FarPtr) {
        if obj == 0 {
            return;
        }
        let map = self.od(obj, 0x4e);
        if map == 0 {
            return;
        }
        let entities = self.od(map, 0x32);
        let triggers = self.od(map, 0x36);
        self.mem.free(entities);
        self.mem.free(triggers);
        let tiles = self.od(map, 0x2e);
        if tiles != self.mem.d(SHARED_TILES) {
            self.mem.free(tiles);
        }
        self.mem.free(map);
        self.set_od(obj, 0x4e, 0);
    }

    /// `sub_1CEC9(obj, x, y)`: centre a map view on a cell.
    pub fn map_center_view(&mut self, obj: FarPtr, x: u16, y: u16) {
        if obj == 0 {
            return;
        }
        let map = self.od(obj, 0x4e);
        if map == 0 {
            return;
        }
        let cols = self.osw(obj, 0x12) / 2;
        let rows = self.osw(obj, 0x14) / 16;
        let mut cx = i32::from(x as i16) - cols / 2;
        let mut cy = i32::from(y as i16) - rows / 2;
        let mw = self.osw(map, 0);
        let mh = self.osw(map, 2);
        if cx + cols > mw {
            cx = mw - cols;
        }
        if mh < cy + rows {
            cy = mh - rows;
        }
        cx = cx.max(0);
        cy = cy.max(0);
        self.set_osw(obj, 0x22, cx);
        self.set_osw(obj, 0x24, cy);
    }

    /// `sub_1CF65(id, map, name)`: attach a map to a view object (F5).
    pub fn map_attach(&mut self, id: u16, map: FarPtr, name: &[u8]) -> Result<()> {
        let mut obj = self.obj_ptr(id);
        if obj == 0 {
            obj = self.window_default(id)?;
        }
        if self.od(obj, 0x4e) != 0 {
            self.free_obj_map(obj);
        }
        self.set_od(obj, 0x4e, map);
        self.set_od(obj, 0, content::MAP_VIEW);
        let mut stored = name.to_vec();
        stored.truncate(19);
        self.mem.strcpy_bytes(ptr_add(obj, 0x3a), &stored);
        let visible = self.ow(obj, 0x28) & flag::VISIBLE;
        let si = ((self.osw(map, 0) * 2) & !1).min(0x50);
        let di = ((self.osw(map, 2) * 16) & !0xf).min(0x190);
        if self.osw(obj, 0x12) >= si || self.osw(obj, 0x14) >= di {
            self.obj_close(id, 0)?;
            let x = self.osw(obj, 0x0a) + si;
            let y = self.osw(obj, 0x0c) + di;
            self.set_osw(obj, 0x0e, x);
            self.set_osw(obj, 0x10, y);
            self.set_osw(obj, 0x12, si);
            self.set_osw(obj, 0x14, di);
        }
        self.set_osw(obj, 0x1a, si);
        self.set_osw(obj, 0x1c, di);
        self.set_ow(obj, 0x22, 0);
        self.set_ow(obj, 0x24, 0);
        if visible != 0 && self.obj_raise(id)? == 0 {
            let index = self.obj_index(id);
            self.obj_redraw(index)?;
        }
        let player = self.mem.d(PLAYER);
        if player != 0 && self.ob(player, 0x0c) == self.ob(map, 0x27) {
            let (px, py) = (self.ow(player, 0), self.ow(player, 2));
            self.map_center_view(obj, px, py);
        }
        self.set_w(W_27152, 0);
        self.cell_buffers_swap();
        self.map_fill_cells(obj);
        Ok(())
    }

    /// `sub_1D0D0(id, map)`: find an entity by id.
    pub fn find_entity(&self, id: u16, map: FarPtr) -> FarPtr {
        let (list, count) = self.entities();
        for i in 0..count {
            let e = ptr_add(list, i * ENTITY_SIZE);
            if self.ow(e, 6) == id {
                return e;
            }
        }
        if map != 0 {
            let (list, count) = self.map_entities(map);
            for i in 0..count {
                let e = ptr_add(list, i * ENTITY_SIZE);
                if self.ow(e, 6) == id {
                    return e;
                }
            }
        }
        0
    }

    /// `sub_1BC6E`: F4 entity query. Returns (x, y, map id, direction,
    /// where) with where = 0 global list, 1 map, 2 not found.
    pub fn entity_query(&self, id: u16) -> (u16, u16, u16, u16, u16) {
        let e = if id == 1000 {
            self.mem.d(PLAYER)
        } else {
            self.find_entity(id, 0)
        };
        if e != 0 {
            return (
                self.ow(e, 0),
                self.ow(e, 2),
                u16::from(self.ob(e, 0x0c)),
                u16::from(self.ob(e, 0x0e) & 0xf),
                0,
            );
        }
        for index in 0..self.obj_count() {
            let obj = self.obj_at(index);
            if self.ow(obj, 0x28) & flag::VISIBLE == 0 {
                continue;
            }
            let map = self.od(obj, 0x4e);
            if map == 0 {
                continue;
            }
            let e = self.find_entity(id, map);
            if e != 0 {
                return (
                    self.ow(e, 0),
                    self.ow(e, 2),
                    u16::from(self.ob(e, 0x0c)),
                    u16::from(self.ob(e, 0x0e) & 0xf),
                    1,
                );
            }
        }
        (0, 0, 0, 0, 2)
    }

    /// `sub_1BD89(values)`: FA entity update.
    pub fn entity_update(&mut self, values: [u16; 5]) {
        let apply = |engine: &mut Engine, e: FarPtr| {
            if values[0] != 1000 {
                engine.set_ow(e, 0, values[0]);
            }
            if values[1] != 1000 {
                engine.set_ow(e, 2, values[1]);
            }
            if values[2] != 1000 {
                engine.set_ob(e, 0x0c, values[2] as u8);
            }
            let dir = (engine.ob(e, 0x0e) & 0xf0).wrapping_add(values[4] as u8 & 0x0f);
            engine.set_ob(e, 0x0e, dir);
            let frame = engine.ow(e, 4) ^ 2;
            engine.set_ow(e, 4, frame);
            engine.set_w(FRAME_COUNTER, 0);
            engine.set_w(W_27144, 0);
        };
        let (list, count) = self.entities();
        for i in 0..count {
            let e = ptr_add(list, i * ENTITY_SIZE);
            if self.ow(e, 6) == values[3] {
                apply(self, e);
                return;
            }
        }
        for index in 0..self.obj_count() {
            let obj = self.obj_at(index);
            let map = self.od(obj, 0x4e);
            if map == 0 {
                continue;
            }
            let (list, count) = self.map_entities(map);
            for i in 0..count {
                let e = ptr_add(list, i * ENTITY_SIZE);
                if self.ow(e, 6) == values[3] {
                    let x = values[0];
                    let y = values[1];
                    if x != 1000 {
                        self.set_ow(e, 0, x);
                    }
                    if y != 1000 {
                        self.set_ow(e, 2, y);
                    }
                    if values[2] != 1000 {
                        self.set_ob(e, 0x0c, values[2] as u8);
                    }
                    let dir = (self.ob(e, 0x0e) & 0xf0).wrapping_add(values[4] as u8 & 0x0f);
                    self.set_ob(e, 0x0e, dir);
                    let frame = self.ow(e, 4) ^ 2;
                    self.set_ow(e, 4, frame);
                    self.set_w(FRAME_COUNTER, 0);
                    self.set_w(W_27144, 0);
                    return;
                }
            }
        }
    }

    /// `sub_1C2E3`: FB tile update.
    pub fn map_set_tile(&mut self, x: u16, y: u16, map: FarPtr, chip: u16, layer: u16) {
        if map == 0 || self.ow(map, 0) <= x || self.ow(map, 2) <= y {
            return;
        }
        let index = y.wrapping_mul(self.ow(map, 0)).wrapping_add(x);
        let tiles = self.od(map, 0x2e);
        let at = ptr_add(tiles, i32::from(index) * 2);
        let old = self.mem.rw(at);
        let chip = if chip > 0x1ff { old & 0x1ff } else { chip };
        let layer = if layer > 3 { (old >> 9) & 3 } else { layer };
        self.mem.ww(
            at,
            (layer << 9).wrapping_add(chip).wrapping_add(old & 0xf800),
        );
    }

    /// FC / FD: set trigger byte +8 for every matching trigger id.
    pub fn map_set_trigger(&mut self, id: u16, trigger: u16, value: u8) {
        let obj = self.obj_ptr(id);
        if obj == 0 {
            return;
        }
        let map = self.od(obj, 0x4e);
        if map == 0 {
            return;
        }
        let triggers = self.od(map, 0x36);
        for i in 0..i32::from(self.ob(map, 0x28)) {
            let t = ptr_add(triggers, i * 10);
            if self.ow(t, 4) == trigger {
                self.set_ob(t, 8, value);
            }
        }
    }

    /// `sub_1D671`: tile word and decoded attributes.
    fn tile_info(&mut self, map: FarPtr, x: i32, y: i32) -> u16 {
        let index = (y as i16 as i32).wrapping_mul(i32::from(self.ow(map, 0) as i16)) + x;
        self.set_w(TILE_INDEX, index as u16);
        let tiles = self.od(map, 0x2e);
        let tile = self.mem.rw(ptr_add(tiles, index * 2));
        self.set_w(TILE_CHIP, tile & 0x21ff);
        self.set_w(TILE_LAYER, (tile >> 9) & 3);
        self.set_w(TILE_OCCUPIED, tile & 0x1000);
        self.set_w(TILE_TRIGGER, tile & 0x800);
        tile
    }

    /// `sub_1D146(x, y, map, start, global)`: find an entity whose 2x2
    /// footprint covers a cell.
    fn entity_at(&mut self, x: i32, y: i32, map: FarPtr, start: i32, global: bool) -> i32 {
        self.mem.set_d(HIT_ENTITY, 0);
        self.set_w(HIT_TYPE, 0);
        let (list, count) = if global {
            let (list, count) = self.entities();
            if list == 0 {
                return -1;
            }
            (list, count)
        } else {
            self.map_entities(map)
        };
        let map_id = self.ob(map, 0x27);
        let mut dx = start;
        let mut found = -1;
        while dx < count {
            let e = ptr_add(list, dx * ENTITY_SIZE);
            self.mem.set_d(HIT_ENTITY, e);
            if self.ob(e, 0x0c) == map_id {
                let ex = self.osw(e, 0);
                let ey = self.osw(e, 2);
                let mut sprite = self.ow(e, 4);
                let kind = if ex == x && ey == y {
                    3
                } else if ex == x && ey - 1 == y {
                    sprite = sprite.wrapping_sub(0x20);
                    1
                } else if ex + 1 == x && ey == y {
                    sprite = sprite.wrapping_add(1);
                    4
                } else if ex + 1 == x && ey - 1 == y {
                    sprite = sprite.wrapping_sub(0x1f);
                    2
                } else {
                    0
                };
                if kind != 0 {
                    self.set_w(HIT_TYPE, kind);
                    let dir = u16::from(self.ob(e, 0x0e) & 0xf) * 4;
                    self.set_w(HIT_SPRITE, sprite.wrapping_add(dir));
                    found = dx;
                    break;
                }
            }
            dx += 1;
        }
        if found == -1 {
            self.mem.set_d(HIT_ENTITY, 0);
        }
        found
    }

    /// `sub_1D279`: trigger id at a cell (0 = none).
    fn trigger_at(&mut self, x: i32, y: i32, map: FarPtr) -> u16 {
        let triggers = self.od(map, 0x36);
        let count = i32::from(self.ob(map, 0x28));
        let mut id = 0;
        let mut hit = 0;
        for i in 0..count {
            let t = ptr_add(triggers, i * 10);
            if i32::from(self.ow(t, 0) as i16) == x && i32::from(self.ow(t, 2) as i16) == y {
                id = self.ow(t, 4);
                hit = t;
                break;
            }
        }
        self.mem.set_d(HIT_TRIGGER, if id == 0 { 0 } else { hit });
        id
    }

    /// `sub_1C356(x, y, entity, map)`: 0 free, 4 blocked, 5 bumped into an
    /// entity, 6 stepped on an active trigger.
    fn map_collide(&mut self, x: i32, y: i32, entity: FarPtr, map: FarPtr) -> u16 {
        for dx in 0..2 {
            self.tile_info(map, x + dx, y);
            let height = u16::from(self.ob(entity, 0x0e) >> 4);
            if height < self.w(TILE_LAYER) {
                return 4;
            }
            if self.w(TILE_OCCUPIED) != 0 {
                for global in [false, true] {
                    let mut start = 0;
                    loop {
                        let hit = self.entity_at(x + dx, y, map, start, global) + 1;
                        start = hit;
                        if hit <= 0 {
                            break;
                        }
                        let other = self.mem.d(HIT_ENTITY);
                        if self.w(HIT_TYPE) > 2 && self.ow(other, 6) != self.ow(entity, 6) {
                            self.set_w(INTERACT_ID, self.ow(other, 6));
                            let ox = self.osw(other, 0);
                            let oy = self.osw(other, 2);
                            let other_dir = self.ob(other, 0x0e);
                            let mut dist = ox - self.osw(entity, 0);
                            if dist < 0 {
                                dist = -dist;
                            }
                            let battle = self.w(BATTLE_MODE) == 2;
                            let (mine, theirs) = if self.osw(entity, 2) <= oy && dist < 2 {
                                (1u8, 0u8)
                            } else if self.osw(entity, 2) > oy && dist < 2 {
                                (0, 1)
                            } else if self.osw(entity, 0) <= ox {
                                (3, 2)
                            } else {
                                (2, 3)
                            };
                            let e = (self.ob(entity, 0x0e) & 0xf0) + mine;
                            self.set_ob(entity, 0x0e, e);
                            if !battle {
                                self.set_ob(other, 0x0e, (other_dir & 0xf0) + theirs);
                            }
                            return 5;
                        }
                    }
                }
            }
            if self.w(TILE_TRIGGER) != 0 {
                let id = self.trigger_at(x + dx, y, map);
                self.set_w(INTERACT_ID, id);
                let t = self.mem.d(HIT_TRIGGER);
                if t != 0 && self.ob(t, 8) == 0 {
                    return 6;
                }
            }
        }
        0
    }

    /// `sub_1C535(entity)`: player movement from keys or mouse.
    pub fn player_move(&mut self, entity: FarPtr) -> Result<u16> {
        if self.w(MAP_ACTIVE) != 1 {
            return Ok(0);
        }
        let mut found = None;
        for index in 0..self.obj_count() {
            let obj = self.obj_at(index);
            let map = self.od(obj, 0x4e);
            if self.ow(obj, 0x28) & flag::NO_FREE == 0
                && map != 0
                && self.ob(map, 0x27) == self.ob(entity, 0x0c)
                && self.ow(obj, 0x28) & flag::VISIBLE != 0
            {
                found = Some((index, obj, map));
                break;
            }
        }
        let Some((index, obj, map)) = found else {
            return Ok(0);
        };
        if self.obj_count() - 1 != index {
            let id = u16::from(self.ob(obj, 0x2b));
            self.obj_raise(id)?;
        }
        let keys = self.direction_keys();
        let (mut si, mut di) = (self.osw(entity, 0), self.osw(entity, 2));
        if keys != 0 {
            let mut dir = 0u8;
            if keys & 1 != 0 {
                di -= 1;
                dir = 0;
            }
            if keys & 2 != 0 {
                di += 1;
                dir = 1;
            }
            if keys & 4 != 0 {
                si -= 1;
                dir = 2;
            }
            if keys & 8 != 0 {
                si += 1;
                dir = 3;
            }
            let v = (self.ob(entity, 0x0e) & 0xf0) + dir;
            self.set_ob(entity, 0x0e, v);
        } else {
            if self.w(LEFT_PRESSES) == 2 {
                self.set_w(MOVE_TARGET_X, (i32::from(self.sw(LEFT_PRESS_X)) / 8) as u16);
                self.set_w(MOVE_TARGET_Y, self.w(LEFT_PRESS_Y));
                self.set_w(W_27144, 1);
            } else if self.w(LEFT_HELD) != 0 {
                self.set_w(MOVE_TARGET_X, (i32::from(self.sw(MOUSE_X)) / 8) as u16);
                self.set_w(MOVE_TARGET_Y, self.w(MOUSE_Y));
                self.set_w(W_27144, 1);
            }
            if self.w(W_27144) == 0 || self.w(LEFT_HELD) == 0 {
                return Ok(0);
            }
            let tx = i32::from(self.sw(MOVE_TARGET_X));
            let ty = i32::from(self.sw(MOVE_TARGET_Y));
            if !(self.osw(obj, 0x0a) <= tx
                && self.osw(obj, 0x0e) > tx
                && self.osw(obj, 0x0c) <= ty
                && self.osw(obj, 0x10) > ty)
            {
                self.set_w(W_27144, 0);
                return Ok(0);
            }
            let cx = self.osw(obj, 0x22) + (tx - self.osw(obj, 0x0a)) / 2;
            let cy = self.osw(obj, 0x24) + (ty - self.osw(obj, 0x0c)) / 16;
            let mut dir: i32 = -1;
            if self.w(MOVE_PREF) != 0 {
                if di - 1 > cy {
                    di += 1;
                    dir = 1;
                }
                if di < cy {
                    di -= 1;
                    dir = 0;
                }
                if si > cx {
                    si += 1;
                    dir = 3;
                }
                if si + 1 < cx {
                    si -= 1;
                    dir = 2;
                }
            } else {
                if di < cy {
                    di += 1;
                    dir = 1;
                }
                if di - 1 > cy {
                    di -= 1;
                    dir = 0;
                }
                if si + 1 < cx {
                    si += 1;
                    dir = 3;
                }
                if si > cx {
                    si -= 1;
                    dir = 2;
                }
            }
            if dir != -1 {
                let v = (self.ob(entity, 0x0e) & 0xf0) + dir as u8;
                self.set_ob(entity, 0x0e, v);
            } else {
                self.set_w(W_27144, 0);
            }
        }
        if si < 0 || self.osw(map, 0) <= si {
            si = self.osw(entity, 0);
        }
        if di < 0 || self.osw(map, 2) <= di {
            di = self.osw(entity, 2);
        }
        if self.osw(entity, 0) == si && self.osw(entity, 2) == di {
            self.set_w(W_27144, 0);
            return Ok(3);
        }
        let mut result = self.map_collide(si, di, entity, map);
        if result == 4 {
            let ey = self.osw(entity, 2);
            result = self.map_collide(si, ey, entity, map);
            if result == 0 || result == 6 {
                di = ey;
            }
            if result == 4 {
                let ex = self.osw(entity, 0);
                result = self.map_collide(ex, di, entity, map);
                if result == 0 || result == 6 {
                    si = ex;
                }
                if result == 4 {
                    si = ex;
                    di = self.osw(entity, 2);
                }
            }
            let mut dir: i32 = -1;
            if self.osw(entity, 2) < di {
                dir = 1;
            }
            if self.osw(entity, 2) > di {
                dir = 0;
            }
            if self.osw(entity, 0) < si {
                dir = 3;
            }
            if self.osw(entity, 0) > si {
                dir = 2;
            }
            if dir != -1 {
                let v = (self.ob(entity, 0x0e) & 0xf0) + dir as u8;
                self.set_ob(entity, 0x0e, v);
            }
        }
        if result == 5 {
            return Ok(5);
        }
        if result == 4 {
            return Ok(4);
        }
        self.set_osw(entity, 0, si);
        self.set_osw(entity, 2, di);
        let cols = self.osw(obj, 0x12) / 2;
        let scroll_x = self.osw(obj, 0x22);
        if scroll_x + 0x11 > si {
            if scroll_x > 0 {
                self.set_osw(obj, 0x22, scroll_x - 1);
            }
        } else if scroll_x + cols - 0x11 < si && scroll_x + cols < self.osw(map, 0) {
            self.set_osw(obj, 0x22, scroll_x + 1);
        }
        let rows = self.osw(obj, 0x14) / 16;
        let scroll_y = self.osw(obj, 0x24);
        if scroll_y + 8 > di {
            if scroll_y > 0 {
                self.set_osw(obj, 0x24, scroll_y - 1);
            }
        } else if scroll_y + rows - 8 < di && self.osw(map, 2) > scroll_y + rows {
            self.set_osw(obj, 0x24, scroll_y + 1);
        }
        Ok(result)
    }

    /// `sub_1D71C(entity)`: NPC step planning. Returns the target cell or
    /// `None` when the entity does not move.
    fn entity_plan(&mut self, e: FarPtr) -> Option<(i32, i32)> {
        if self.ob(e, 0x10) & 3 != 0 {
            return None;
        }
        let mut si = self.osw(e, 0);
        let mut di = self.osw(e, 2);
        let map_id = self.ob(e, 0x0c);
        let ai = self.ob(e, 0x0d);
        let id = self.ow(e, 6);
        let mut wander = false;
        if ai != 0 {
            let (list, count) = self.entities();
            let range = i32::from(ai & 0x0f);
            let mut best = (1000i32, 1000i32, -1i32);
            for i in 0..count {
                let t = ptr_add(list, i * ENTITY_SIZE);
                if self.ob(t, 0x10) & 8 != 0 && self.ob(t, 0x0c) == map_id && self.ow(t, 6) != id {
                    let dx = (si - self.osw(t, 0)).abs();
                    let dy = (di - self.osw(t, 2)).abs();
                    if dx < best.0 && dy < best.1 {
                        best = (dx, dy, i);
                    }
                }
            }
            if best.0 == 1000 {
                wander = true;
            } else {
                let t = ptr_add(list, best.2 * ENTITY_SIZE);
                let dx = si - self.osw(t, 0);
                let dy = di - self.osw(t, 2);
                if ai & 0x20 != 0 && dx.abs() > 0 && dy.abs() > 0 {
                    wander = true;
                }
                if dx.abs() > range || dy.abs() > range {
                    wander = true;
                }
                if !wander {
                    let mut dir = i32::from(self.ob(e, 0x0e) & 0xf);
                    if dx > 0 {
                        si -= 1;
                        dir = 2;
                    }
                    if dx < 0 {
                        si += 1;
                        dir = 3;
                    }
                    if dy > 0 {
                        di -= 1;
                        dir = 0;
                    }
                    if dy < 0 {
                        di += 1;
                        dir = 1;
                    }
                    let v = (self.ob(e, 0x0e) & 0xf0) + dir as u8;
                    self.set_ob(e, 0x0e, v);
                }
            }
        }
        if self.ob(e, 0x0d) == 0 || wander {
            let r = self.rand_n(0x1e);
            if r < 4 {
                let v = (self.ob(e, 0x0e) & 0xf0) + r as u8;
                self.set_ob(e, 0x0e, v);
            }
            let bit = 1u16 << (u16::from(self.ob(e, 0x0e) & 0xf) + 4);
            if u16::from(self.ob(e, 0x0f)) & bit != 0 {
                return None;
            }
            match bit {
                0x10 => di -= 1,
                0x20 => di += 1,
                0x40 => si -= 1,
                0x80 => si += 1,
                _ => {}
            }
        }
        Some((si, di))
    }

    /// `sub_1D95F(entity, map, x, y)`: move an NPC if the cells are free.
    pub(crate) fn entity_step(&mut self, e: FarPtr, map: FarPtr, x: i32, y: i32) -> i32 {
        if x < 1 || y < 1 || self.osw(map, 0) - 1 <= x || self.osw(map, 2) - 1 <= y {
            return -3;
        }
        for dx in 0..2 {
            self.tile_info(map, x + dx, y);
            if u16::from(self.ob(e, 0x0e) >> 4) < self.w(TILE_LAYER) {
                let r = self.rand_n(3) as u8;
                let v = (self.ob(e, 0x0e) & 0xf0) + r;
                self.set_ob(e, 0x0e, v);
                return -4;
            }
            if self.w(TILE_OCCUPIED) != 0 {
                for global in [false, true] {
                    let mut start = 0;
                    loop {
                        let hit = self.entity_at(x + dx, y, map, start, global) + 1;
                        start = hit;
                        if hit <= 0 {
                            break;
                        }
                        let other = self.mem.d(HIT_ENTITY);
                        if self.w(HIT_TYPE) > 2 && self.ow(other, 6) != self.ow(e, 6) {
                            // The original returns the blocking actor's id in AX.
                            return i32::from(self.ow(other, 6));
                        }
                    }
                }
            }
        }
        self.set_osw(e, 0, x);
        self.set_osw(e, 2, y);
        0
    }

    /// `sub_1DA67(obj, pass)`: animate / move the entities of a view.
    fn map_update_entities(&mut self, obj: FarPtr, map_list: bool) -> Result<bool> {
        let id = self.ob(obj, 0x2b);
        if (0x96..=0xef).contains(&id) {
            return Ok(true);
        }
        let map = self.od(obj, 0x4e);
        let map_id = self.ob(map, 0x27);
        let (list, count) = if map_list {
            self.map_entities(map)
        } else {
            self.entities()
        };
        for i in 0..count {
            let e = ptr_add(list, i * ENTITY_SIZE);
            if self.ob(e, 0x0c) != map_id {
                continue;
            }
            let divider = u16::from(self.ob(e, 0x0f) & 0xf);
            if divider == 0 || !self.w(FRAME_COUNTER).is_multiple_of(divider) {
                continue;
            }
            let frame = self.ow(e, 4) ^ 2;
            self.set_ow(e, 4, frame);
            if self.w(MAP_ACTIVE) == 2 {
                continue;
            }
            if self.ob(e, 0x10) & 4 != 0 {
                let result = self.player_move(e)?;
                if result == 5 {
                    self.set_w(W_27144, 0);
                }
                if result == 5 || result == 6 {
                    self.set_w(MAP_EVENT, result);
                    return Ok(false);
                }
            } else if let Some((x, y)) = self.entity_plan(e) {
                self.entity_step(e, map, x, y);
            }
        }
        Ok(true)
    }

    fn cell_buffer(&self, which: u16) -> FarPtr {
        self.mem.d(MAP_CELLS + (which & 1) * 4)
    }

    /// `sub_1D2D9`: switch cell buffers and clear the new one.
    pub fn cell_buffers_swap(&mut self) {
        let sel = self.w(MAP_CELL_SEL) ^ 1;
        self.set_w(MAP_CELL_SEL, sel);
        let buf = self.cell_buffer(sel);
        self.mem.memset(buf, 0, 10000);
    }

    /// `sub_1D305(obj)`: fill the current cell buffer for a view.
    fn map_fill_cells(&mut self, obj: FarPtr) {
        let cells = self.cell_buffer(self.w(MAP_CELL_SEL));
        let owner = u16::from(self.ob(obj, 0x2b));
        let map = self.od(obj, 0x4e);
        if map == 0 {
            return;
        }
        let tiles = self.od(map, 0x2e);
        let map_id = self.ob(map, 0x27);
        let x1 = self.osw(obj, 0x0a);
        let y1 = self.osw(obj, 0x0c);
        let cols = self.osw(obj, 0x12) / 2;
        let rows = self.osw(obj, 0x14) / 16;
        let sx = self.osw(obj, 0x22);
        let sy = self.osw(obj, 0x24);
        let ex = sx + cols;
        let ey = sy + rows;
        let mw = self.osw(map, 0);
        for pass in 0..2 {
            let (list, count) = if pass == 0 {
                self.entities()
            } else {
                self.map_entities(map)
            };
            for i in 0..count {
                let e = ptr_add(list, i * ENTITY_SIZE);
                if self.ob(e, 0x0c) != map_id {
                    continue;
                }
                let mut sprite = self
                    .ow(e, 4)
                    .wrapping_add(u16::from(self.ob(e, 0x0e) & 0xf) * 4)
                    .wrapping_add(1);
                let (px, py) = (self.osw(e, 0), self.osw(e, 2));
                let mut y = py;
                for _ in 0..2 {
                    let mut x = self.osw(e, 0);
                    for _ in 0..2 {
                        let screen_x = (x - sx) * 2 + x1;
                        let screen_y = (y - sy) * 16 + y1;
                        let state = self.cell_state(screen_x, screen_y, owner);
                        if state == 1 || state == 3 {
                            let tile = ptr_add(tiles, (y * mw + x) * 2);
                            let v = self.mem.rw(tile) | 0x1000;
                            self.mem.ww(tile, v);
                            let cell = (screen_y / 16) * 40 + screen_x / 2;
                            let entry = ptr_add(cells, cell * 10);
                            let mut rec = self.mem.read_bytes(entry, 10);
                            let s1 = u16::from_le_bytes([rec[4], rec[5]]);
                            let s2 = u16::from_le_bytes([rec[6], rec[7]]);
                            let set1 = |rec: &mut Vec<u8>| {
                                rec[0] = px as u8;
                                rec[1] = py as u8;
                                rec[4..6].copy_from_slice(&sprite.to_le_bytes());
                            };
                            if s1 == 0 {
                                set1(&mut rec);
                            } else if i32::from(rec[1]) > py
                                || (i32::from(rec[1]) == py && i32::from(rec[0]) < px)
                            {
                                rec[2] = rec[0];
                                rec[3] = rec[1];
                                rec[6] = rec[4];
                                rec[7] = rec[5];
                                set1(&mut rec);
                            } else if s2 == 0
                                || i32::from(rec[3]) > py
                                || (i32::from(rec[3]) == py && i32::from(rec[2]) < px)
                            {
                                rec[2] = px as u8;
                                rec[3] = py as u8;
                                rec[6..8].copy_from_slice(&sprite.to_le_bytes());
                            }
                            self.mem.write_bytes(entry, &rec);
                        }
                        x += 1;
                        sprite = sprite.wrapping_add(1);
                    }
                    y -= 1;
                    sprite = sprite.wrapping_sub(0x22);
                }
            }
        }
        let mut screen_row = y1 / 16;
        let mut screen_y = y1;
        for y in sy..ey {
            let mut entry = ptr_add(cells, (screen_row * 40 + x1 / 2) * 10);
            let mut tile = ptr_add(tiles, (y * mw + sx) * 2);
            let mut screen_x = x1;
            for _ in sx..ex {
                let state = self.cell_state(screen_x, screen_y, owner);
                if state == 1 || state == 3 {
                    let v = self.mem.rw(tile) & 0x21ff;
                    self.mem.ww(ptr_add(entry, 8), v);
                }
                entry = ptr_add(entry, 10);
                tile = ptr_add(tile, 2);
                screen_x += 2;
            }
            screen_row += 1;
            screen_y += 16;
        }
    }

    /// Map view content routine (seg015:1A55).
    pub fn map_view_draw(&mut self, index: i32) -> Result<()> {
        let obj = self.obj_at(index);
        if self.od(obj, 0x4e) == 0 {
            return Ok(());
        }
        let saved = self.w(W_27152);
        if self.ow(obj, 0x28) & flag::NO_FREE != 0 {
            self.set_w(W_27152, 0);
        }
        self.set_w(FRAME_COUNTER, 0);
        let active = self.w(MAP_ACTIVE);
        self.set_w(MAP_ACTIVE, 2);
        self.map_frame(index)?;
        self.set_w(MAP_ACTIVE, active);
        self.set_w(W_27152, saved);
        Ok(())
    }

    /// `sub_1DB80(index)`: one map frame (FK).  Returns false when the
    /// frame timer has not elapsed.
    pub fn map_frame(&mut self, index: i32) -> Result<bool> {
        if self.w(MAP_ACTIVE) == 0 {
            return Ok(false);
        }
        if self.w(FRAME_COUNTER) != 0 && self.shift_state() & 1 == 0 {
            if self.w(FRAME_TIMER) != 0 {
                return Ok(false);
            }
            let delay = self.w(FRAME_DELAY);
            self.set_w(FRAME_TIMER, delay);
        }
        let (first, last) = if index == -1 {
            (0, self.obj_count() - 1)
        } else {
            (index, index)
        };
        let counter = self.w(FRAME_COUNTER).wrapping_add(1);
        self.set_w(FRAME_COUNTER, counter);
        if counter != 1 && self.w(BATTLE_MODE) == 1 {
            let mut i = first;
            while i <= last {
                let obj = self.obj_at(i);
                if self.ow(obj, 0x28) & 9 == 1 && self.od(obj, 0x4e) != 0 {
                    self.map_update_entities(obj, false)?;
                    self.map_update_entities(obj, true)?;
                }
                i += 1;
            }
        }
        self.cell_buffers_swap();
        let previous = self.cell_buffer(self.w(MAP_CELL_SEL) ^ 1);
        let current = self.cell_buffer(self.w(MAP_CELL_SEL));
        let mut i = first;
        while i <= last {
            let obj = self.obj_at(i);
            if self.ow(obj, 0x28) & flag::VISIBLE != 0 && self.od(obj, 0x4e) != 0 {
                self.map_fill_cells(obj);
            }
            i += 1;
        }
        let saved_mode = self.w(CURSOR_MODE);
        if saved_mode == 0 {
            self.set_w(CURSOR_MODE, 1);
        }
        let temp = self.mem.alloc(128);
        let mut i = first;
        while i <= last {
            let obj = self.obj_at(i);
            i += 1;
            if self.ow(obj, 0x28) & flag::VISIBLE == 0 || self.od(obj, 0x4e) == 0 {
                continue;
            }
            let map = self.od(obj, 0x4e);
            let owner = u16::from(self.ob(obj, 0x2b));
            let x1 = self.osw(obj, 0x0a);
            let y1 = self.osw(obj, 0x0c);
            let cols = self.osw(obj, 0x12) / 2;
            let rows = self.osw(obj, 0x14) / 16;
            let sx = self.osw(obj, 0x22);
            let sy = self.osw(obj, 0x24);
            for row in 0..rows {
                let y = sy + row;
                let screen_y = y1 + row * 16;
                for col in 0..cols {
                    let x = sx + col;
                    let screen_x = x1 + col * 2;
                    let state = self.cell_state(screen_x, screen_y, owner);
                    if state != 1 && state != 3 {
                        continue;
                    }
                    let cell = (screen_y / 16) * 40 + screen_x / 2;
                    let cur = ptr_add(current, cell * 10);
                    let prev = ptr_add(previous, cell * 10);
                    let same = self.mem.read_bytes(cur, 10) == self.mem.read_bytes(prev, 10);
                    if same && self.w(W_27152) != 0 {
                        continue;
                    }
                    let sprite1 = self.ow(cur, 4);
                    let tile = self.ow(cur, 8);
                    let overlay = self.mem.d(CHIP_OVERLAY);
                    let base = self.mem.d(CHIP_BASE);
                    if sprite1 != 0 {
                        self.cell_copy(temp, overlay, sprite1 - 1);
                        let sprite2 = self.ow(cur, 6);
                        if sprite2 != 0 {
                            self.cell_overlay_mask(temp, overlay, sprite2 - 1);
                        }
                        if self.w(W_23B5C) != 0 {
                            self.cell_overlay_and(temp, base, tile);
                        } else {
                            self.cell_overlay_either(temp, base, tile);
                        }
                    } else {
                        self.cell_copy(temp, base, tile);
                    }
                    let highlight = self.w(W_29012);
                    if highlight != 0 {
                        self.tile_info(map, x, y);
                        let layer_bit = 1u16 << self.w(TILE_LAYER);
                        let hit = highlight & layer_bit != 0
                            || (highlight & 0x10 != 0 && self.w(TILE_TRIGGER) != 0);
                        if hit {
                            for k in (0..128).step_by(2) {
                                let v = self.mem.rw(ptr_add(temp, k)) ^ 0xffff;
                                self.mem.ww(ptr_add(temp, k), v);
                            }
                        }
                    }
                    let mode = if self.w(W_23B5C) != 0 { 2 } else { 3 };
                    self.draw_chip(screen_x, screen_y, temp, 0, mode, owner);
                }
            }
        }
        self.mem.free(temp);
        self.set_w(CURSOR_MODE, saved_mode);
        self.set_w(W_27152, 1);
        Ok(true)
    }

    /// `sub_17E5F`: I5 — interactive NPC (0xC8..0xD6) next to the player.
    pub fn nearby_npc(&mut self) -> u16 {
        let player = self.mem.d(PLAYER);
        if player == 0 {
            return 0;
        }
        let map_id = u16::from(self.ob(player, 0x0c));
        let px = self.ow(player, 0);
        let py = self.ow(player, 2);
        self.set_w(INTERACT_ID, 0);
        for id in 0xc8u16..0xd7 {
            let (x, y, m, _dir, place) = self.entity_query(id);
            if place == 1
                && m == map_id
                && py.wrapping_sub(2) <= y
                && py.wrapping_add(1) >= y
                && px.wrapping_sub(1) <= x
                && px.wrapping_add(2) >= x
            {
                self.set_w(MAP_EVENT, 5);
                self.set_w(INTERACT_ID, id);
                return 1;
            }
        }
        0
    }

    /// `sub_1C1ED`: FH — map cell under a screen position of the top view.
    pub fn map_probe(&mut self, x: u16, y: u16) -> (u16, u16, u16, u16, u16) {
        let count = self.obj_count();
        if count == 0 {
            return (0, 0, 0, 0, 0);
        }
        let obj = self.obj_at(count - 1);
        let col = i32::from(x as i16) / 8;
        let y = i32::from(y as i16);
        if !(self.osw(obj, 0x0a) <= col
            && self.osw(obj, 0x0e) > col
            && self.osw(obj, 0x0c) <= y
            && self.osw(obj, 0x10) > y)
        {
            return (0, 0, 0, 0, 0);
        }
        let cx = self.osw(obj, 0x22) + (col - self.osw(obj, 0x0a)) / 2;
        let cy = self.osw(obj, 0x24) + (y - self.osw(obj, 0x0c)) / 16;
        let mut entity = 0;
        let mut trigger = 0;
        let map = self.od(obj, 0x4e);
        if map != 0 {
            for global in [false, true] {
                if self.entity_at(cx, cy, map, 0, global) != -1 {
                    entity = self.ow(self.mem.d(HIT_ENTITY), 6);
                }
            }
            trigger = self.trigger_at(cx, cy, map);
        }
        (
            u16::from(self.ob(obj, 0x2b)),
            cx as u16,
            cy as u16,
            entity,
            trigger,
        )
    }

    /// `sub_1CA90(id, name)`: F1 chip-sheet view.
    pub fn chip_view(&mut self, id: u16, name: &[u8]) -> Result<()> {
        let mut obj = self.obj_ptr(id);
        if obj == 0 {
            obj = self.obj_at(self.obj_count() - 1);
        }
        self.set_od(obj, 0, content::CHIP_VIEW);
        let cursor = self.cursor_show(0);
        let old = self.od(obj, 0x5a);
        self.mem.free(old);
        self.access_back();
        let chips = self.pdt_to_cells(name, 0)?;
        self.set_od(obj, 0x5a, chips);
        self.access_front();
        let w = (self
            .w(PDT_RIGHT)
            .wrapping_sub(self.w(PDT_LEFT))
            .wrapping_add(1))
            & 0xfffe;
        let h = (self
            .w(PDT_BOTTOM)
            .wrapping_sub(self.w(PDT_TOP))
            .wrapping_add(1))
            & 0xfff0;
        self.set_ow(obj, 0x1a, w);
        self.set_ow(obj, 0x1c, h);
        self.set_ow(obj, 0x22, 0);
        self.set_ow(obj, 0x24, 0);
        if (w as i16) < (self.ow(obj, 0x12) as i16) || (h as i16) < (self.ow(obj, 0x14) as i16) {
            self.obj_close(id, 0)?;
            self.set_ow(obj, 0x12, w);
            let x2 = self.ow(obj, 0x0a).wrapping_add(w);
            self.set_ow(obj, 0x0e, x2);
            self.set_ow(obj, 0x14, h);
            let y2 = self.ow(obj, 0x0c).wrapping_add(h);
            self.set_ow(obj, 0x10, y2);
            self.obj_raise(id)?;
        }
        self.access_back();
        self.vram.clear_planes(0x0f);
        let bg = self.mem.cstr(ds_ptr(BG_NAME));
        self.pdt_show(&super::interp::trim_name(&bg))?;
        self.access_front();
        let f = self.ow(obj, 0x28) | flag::VISIBLE;
        self.set_ow(obj, 0x28, f);
        self.chip_view_draw(self.obj_count() - 1)?;
        self.cursor_show(cursor);
        Ok(())
    }

    /// `sub_1C9E4`: draw a chip-sheet view.
    pub fn chip_view_draw(&mut self, index: i32) -> Result<()> {
        let obj = self.obj_at(index);
        if self.ow(obj, 0x28) & flag::VISIBLE == 0 {
            return Ok(());
        }
        let cursor = self.w(CURSOR_SHOWN);
        let per_row = self.osw(obj, 0x1a) / 2;
        let owner = u16::from(self.ob(obj, 0x2b));
        let chips = self.od(obj, 0x5a);
        let mut row = self.osw(obj, 0x24);
        let mut y = 0;
        while y < self.osw(obj, 0x14) {
            let mut col = self.osw(obj, 0x22);
            let mut x = 0;
            while x < self.osw(obj, 0x12) {
                let index = row * per_row + col;
                let sx = self.osw(obj, 0x0a) + x;
                let sy = self.osw(obj, 0x0c) + y;
                self.draw_chip(sx, sy, chips, index as u16, 2, owner);
                x += 2;
                col += 1;
            }
            y += 16;
            row += 1;
        }
        self.cursor_show(cursor);
        Ok(())
    }

    /// `sub_1C0B0(name)`: FI — load a PDT into the overlay chip set.
    pub fn load_overlay_chips(&mut self, name: &[u8]) -> Result<()> {
        let cursor = self.cursor_show(0);
        self.access_back();
        let overlay = self.mem.d(CHIP_OVERLAY);
        let mut index = 0u16;
        let mut y = 0x10u16;
        while y < 0xf0 {
            let mut x = 6u16;
            while x < 0x46 {
                self.cell_put(x, y, overlay, index, 2);
                x += 2;
                index += 1;
            }
            y += 0x10;
        }
        self.pdt_show(name)?;
        self.cells_grab(6, 0x10, 0x40, 0xe0, overlay, 0);
        self.vram.clear_planes(0x0f);
        let bg = self.mem.cstr(ds_ptr(BG_NAME));
        self.pdt_show(&super::interp::trim_name(&bg))?;
        self.access_front();
        self.cursor_show(cursor);
        Ok(())
    }

    /// `sub_1C964`: F6 — load the global entity list.
    pub fn entities_load(&mut self, name: &[u8]) -> Result<()> {
        let bytes = self.read_resource(name)?;
        let count = bytes.len() / ENTITY_SIZE as usize;
        self.set_w(ENTITY_COUNT, count as u16);
        let old = self.mem.d(ENTITIES);
        let list = self.mem.realloc(old, bytes.len());
        self.mem.set_d(ENTITIES, list);
        if count > 0 {
            self.mem.write_bytes(list, &bytes);
        }
        Ok(())
    }

    /// `sub_1C920`: F7 — save the global entity list.
    pub fn entities_save(&mut self, name: &[u8]) -> Result<()> {
        let (list, count) = self.entities();
        let bytes = self.mem.read_bytes(list, (count * ENTITY_SIZE) as usize);
        self.write_data_file(name, &bytes)
    }
}
