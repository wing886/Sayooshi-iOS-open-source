//! Persisting player settings in the global save.

use anyhow::Result;

use crate::serial::{Reader, Writer};
use crate::settings::WindowAttr;
use crate::system::System;

pub fn write(w: &mut Writer, sys: &System) {
    let s = &sys.settings;
    w.i32(s.message_speed);
    w.bool(s.message_no_wait);
    w.i32(s.koe_mode);
    w.bool(s.bgm_koe_fade);
    w.i32(s.bgm_koe_fade_vol);
    w.i32s(&s.volume);
    for enabled in s.enabled {
        w.bool(enabled);
    }
    w.bool(s.auto_mode);
    w.i32(s.auto_char_time);
    w.i32(s.auto_base_time);
    w.i32(s.font_quality);
    w.i32(s.font_weight);
    w.i32(s.font_shadow);
    let a = s.window_attr;
    w.i32s(&[a.r, a.g, a.b, a.alpha, a.filter]);
    w.bool(s.show_object[0]);
    w.bool(s.show_object[1]);
    w.bool(s.show_weather);
    w.bool(s.classify_text);
    w.i32s(&s.generic);
    w.i32(s.screen_mode);
    w.bool(s.cursor_mono);
    w.bool(s.skip_animations);
    w.bool(s.low_priority);
    w.bool(s.confirm_save_load);
    w.bool(s.reduce_distortion);
    w.i32(s.sound_quality);
    w.len(s.use_koe.len());
    for (character, flag) in &s.use_koe {
        w.i32(*character);
        w.bool(*flag);
    }
    w.i32(s.waku_all);
    w.i32s(&sys.syscom.states);
}

pub fn read(r: &mut Reader, sys: &mut System) -> Result<()> {
    let s = &mut sys.settings;
    s.message_speed = r.i32()?;
    s.message_no_wait = r.bool()?;
    s.koe_mode = r.i32()?;
    s.bgm_koe_fade = r.bool()?;
    s.bgm_koe_fade_vol = r.i32()?;
    crate::serial::fill(&mut s.volume, &r.i32s()?);
    for enabled in &mut s.enabled {
        *enabled = r.bool()?;
    }
    s.auto_mode = r.bool()?;
    s.auto_char_time = r.i32()?;
    s.auto_base_time = r.i32()?;
    s.font_quality = r.i32()?;
    s.font_weight = r.i32()?;
    s.font_shadow = r.i32()?;
    if let Some(attr) = WindowAttr::from_slice(&r.i32s()?) {
        s.window_attr = attr;
    }
    s.show_object[0] = r.bool()?;
    s.show_object[1] = r.bool()?;
    s.show_weather = r.bool()?;
    s.classify_text = r.bool()?;
    crate::serial::fill(&mut s.generic, &r.i32s()?);
    s.screen_mode = r.i32()?;
    s.cursor_mono = r.bool()?;
    s.skip_animations = r.bool()?;
    s.low_priority = r.bool()?;
    s.confirm_save_load = r.bool()?;
    s.reduce_distortion = r.bool()?;
    s.sound_quality = r.i32()?;
    let count = r.len()?;
    s.use_koe.clear();
    for _ in 0..count {
        let character = r.i32()?;
        let flag = r.bool()?;
        s.use_koe.insert(character, flag);
    }
    s.waku_all = r.i32()?;
    let states = r.i32s()?;
    crate::serial::fill(&mut sys.syscom.states, &states);
    sys.volumes_changed = true;
    Ok(())
}
