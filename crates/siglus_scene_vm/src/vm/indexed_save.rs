//! The 3120-byte split-file generation saves scene indices and individual
//! local fields, before the later local POD/envelope formats.
use super::*;

impl<'a> SceneVm<'a> {
    pub(super) fn indexed_scene_name(&self, scene_no: i32) -> Result<String> {
        self.ctx
            .scene_metadata()?
            .rows
            .get(scene_no as usize)
            .map(|row| row.0.clone())
            .ok_or_else(|| anyhow!("invalid saved scene index {scene_no}"))
    }

    pub(super) fn detect_indexed_local_layout(&self, data: &[u8]) -> Option<NativeLocalLayout> {
        [NativeLocalLayout::ShortElements, NativeLocalLayout::Indexed]
            .into_iter()
            .find(|layout| self.probe_indexed_local_stream(data, *layout))
    }

    #[cfg(test)]
    pub(super) fn detect_indexed_local_stream(&self, data: &[u8]) -> bool {
        self.detect_indexed_local_layout(data).is_some()
    }

    pub(super) fn probe_indexed_local_stream(
        &self,
        data: &[u8],
        layout: NativeLocalLayout,
    ) -> bool {
        let probe = || -> Result<()> {
            let mut rd = crate::original_save::OriginalStreamReader::new(data);
            rd.layout = layout;
            let element_size = (layout.element_capacity() + 1) * 4;
            anyhow::ensure!(
                rd.i32()? >= 0 && rd.i32()? >= 0 && rd.i32()? >= 0,
                "invalid lexer position"
            );
            self.skip_indexed_probe_proc(&mut rd)?;
            let count = rd.i32()?;
            anyhow::ensure!(
                count >= 0 && count as usize <= rd.remaining().len() / (element_size + 18),
                "invalid proc count"
            );
            for _ in 0..count {
                self.skip_indexed_probe_proc(&mut rd)?;
            }
            for _ in 0..3 {
                rd.skip_element()?;
            }
            rd.skip(16)?;
            rd.string()?;
            // Cursor, settings and key flags: no padding and no font/message.
            rd.skip(layout.indexed_settings_bytes() + self.mwnd_waku_btn_count())?;
            rd.check_local_layout(true, 0)
        };
        probe().is_ok()
    }

    // A format probe must not allocate vectors from counts read at the wrong
    // offset of a newer stream. These skips mirror the shared proc/prop records.
    fn indexed_probe_count(
        rd: &mut crate::original_save::OriginalStreamReader<'_>,
        min_size: usize,
    ) -> Result<usize> {
        let count = rd.i32()?;
        anyhow::ensure!(
            count >= 0 && count as usize <= rd.remaining().len() / min_size,
            "invalid probe array count"
        );
        Ok(count as usize)
    }

    fn skip_indexed_probe_prop(
        &self,
        rd: &mut crate::original_save::OriginalStreamReader<'_>,
        depth: usize,
    ) -> Result<()> {
        anyhow::ensure!(depth < 64, "probe property nesting limit");
        rd.i32()?;
        let form = rd.i32()?;
        rd.i32()?;
        rd.string()?;
        rd.skip_element()?;
        for _ in 0..Self::indexed_probe_count(rd, (rd.layout.element_capacity() + 1) * 4 + 24)? {
            self.skip_indexed_probe_prop(rd, depth + 1)?;
        }
        rd.i32()?;
        if form == self.cfg.fm_intlist {
            let count = Self::indexed_probe_count(rd, 4)?;
            rd.skip(count * 4)?;
        } else if form == self.cfg.fm_strlist {
            for _ in 0..Self::indexed_probe_count(rd, 4)? {
                rd.string()?;
            }
        }
        Ok(())
    }

    fn skip_indexed_probe_proc(
        &self,
        rd: &mut crate::original_save::OriginalStreamReader<'_>,
    ) -> Result<()> {
        rd.skip(4)?;
        rd.skip_element()?;
        rd.skip(4)?;
        for _ in 0..Self::indexed_probe_count(rd, (rd.layout.element_capacity() + 1) * 4 + 24)? {
            self.skip_indexed_probe_prop(rd, 0)?;
        }
        rd.skip(rd.layout.proc_trailer_bytes())
    }

    pub(super) fn read_indexed_local_settings(
        &mut self,
        rd: &mut crate::original_save::OriginalStreamReader<'_>,
    ) -> Result<()> {
        let button_count = self.mwnd_waku_btn_count();
        let script = &mut self.ctx.globals.script;
        let syscom = &mut self.ctx.globals.syscom;
        script.cur_koe_no = rd.i32()? as i64;
        script.cur_chr_no = rd.i32()? as i64;
        script.cur_read_flag_scn_no = rd.i32()? as i64;
        script.cur_read_flag_flag_no = rd.i32()? as i64;
        syscom.current_save_scene_title = rd.string()?;
        syscom.current_save_full_message.clear();
        script.cursor_no = rd.i32()? as i64;
        syscom.syscom_menu_disable = rd.bool()?;
        script.hide_mwnd_disable = rd.bool()?;
        syscom.mwnd_btn_disable_all = rd.bool()?;
        syscom.mwnd_btn_disable.clear();
        for idx in 0..button_count {
            if rd.bool()? {
                syscom.mwnd_btn_disable.insert(idx as i64, true);
            }
        }
        syscom.mwnd_btn_touch_disable = rd.bool()?;
        script.skip_disable = rd.bool()?;
        script.ctrl_disable = rd.bool()?;
        script.not_stop_skip_by_click = rd.bool()?;
        script.not_skip_msg_by_click = rd.bool()?;
        script.skip_unread_message = rd.bool()?;
        script.shortcut_disable = rd.bool()?;
        script.msg_back_off = rd.bool()?;
        script.msg_back_disp_off = rd.bool()?;
        script.auto_mode_flag = rd.bool()?;
        script.auto_mode_moji_wait = rd.i32()? as i64;
        script.auto_mode_min_wait = rd.i32()? as i64;
        script.msg_speed = rd.i32()? as i64;
        script.msg_nowait = rd.bool()?;
        script.async_msg_mode = rd.bool()?;
        script.async_msg_mode_once = rd.bool()?;
        script.multi_msg_mode = rd.bool()?;
        script.skip_trigger = rd.bool()?;
        script.cursor_disp_off = rd.bool()?;
        script.cursor_runtime_visible = !script.cursor_disp_off;
        script.cursor_move_by_key_disable = rd.bool()?;
        script.key_disable.clear();
        for key in 0u16..=255 {
            if rd.bool()? {
                script.key_disable.insert(key as u8);
            }
        }
        script.mwnd_anime_on_flag = rd.bool()?;
        script.mwnd_anime_off_flag = rd.bool()?;
        script.mwnd_disp_off_flag = rd.bool()?;
        script.koe_dont_stop_on_flag = rd.bool()?;
        script.koe_dont_stop_off_flag = rd.bool()?;
        script.quake_stop_flag = rd.bool()?;
        self.ctx.globals.cg_table_off = rd.bool()?;
        script.bgmfade_flag = rd.bool()?;
        script.dont_set_save_point = rd.bool()?;
        script.ignore_r_flag = rd.bool()?;
        script.wait_display_vsync_off_flag = rd.bool()?;
        // These fields did not exist in this generation.
        script.font_name.clear();
        script.font_bold = -1;
        script.font_shadow = -1;
        script.msg_back_disable = false;
        script.auto_mode_moji_cnt = 0;
        script.mouse_cursor_hide_onoff = -1;
        script.mouse_cursor_hide_time = -1;
        script.msg_back_save_cntr = 0;
        script.emote_mouth_stop_flag = false;
        script.time_stop_flag = false;
        script.counter_time_stop_flag = false;
        script.frame_action_time_stop_flag = false;
        script.stage_time_stop_flag = false;
        syscom.replay_koe =
            (script.cur_koe_no >= 0).then_some((script.cur_koe_no, script.cur_chr_no));
        Ok(())
    }
}
