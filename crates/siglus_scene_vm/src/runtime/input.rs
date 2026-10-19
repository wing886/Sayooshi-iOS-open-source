//! Runtime input state (winit-agnostic).
//!
//! Siglus scripts query input via numeric forms (INPUT/MOUSE/KEYLIST) and helper
//! key objects. The original engine stores per-key state in fixed tables.
//!
//! Runtime input model:
//! - A fixed 0..=255 virtual-key table (down + edge "stock" flags)
//! - Mouse position
//! - Mouse wheel delta since last read / frame

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VmKey {
    Escape,
    Enter,
    Space,
    Backspace,
    Delete,
    Tab,
    Shift,
    Control,
    Meta,
    Alt,
    Home,
    End,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    /// Function keys (F1..F12).
    F(u8),
    /// Digit keys 0..9.
    Digit(u8),
    /// Latin letter keys A..Z.
    Letter(char),
    /// Any other unmapped physical key.
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VmMouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone, Copy)]
struct KeyState {
    down: bool,
    down_stock: bool,
    up_stock: bool,
    // Mirrors tona3 C_input_state::BUTTON::down_up_stock:
    // 0 = no down/up sequence pending; 1 = down happened; 2 = down+up completed.
    // NEXT/frame clears only state 2, so state 1 survives until the matching up event.
    down_up_stock: u8,
    flick_stock: bool,
    flick_angle: f32,
    flick_pixel: f32,
    flick_mm: f32,
    flick_start: Option<(i32, i32)>,
}

impl KeyState {
    const fn new() -> Self {
        Self {
            down: false,
            down_stock: false,
            up_stock: false,
            down_up_stock: 0,
            flick_stock: false,
            flick_angle: 0.0,
            flick_pixel: 0.0,
            flick_mm: 0.0,
            flick_start: None,
        }
    }

    fn clear_all(&mut self) {
        self.down = false;
        self.down_stock = false;
        self.up_stock = false;
        self.down_up_stock = 0;
        self.flick_stock = false;
        self.flick_angle = 0.0;
        self.flick_pixel = 0.0;
        self.flick_mm = 0.0;
        self.flick_start = None;
    }

    fn clear_stocks(&mut self) {
        self.down_stock = false;
        self.up_stock = false;
        // tona3 BUTTON::frame() only clears completed down-up stock.
        // The intermediate state (1) must persist across NEXT/frame until set_up().
        if self.down_up_stock == 2 {
            self.down_up_stock = 0;
        }
        self.flick_stock = false;
    }

    fn use_stocks(&mut self) {
        self.down_stock = false;
        self.up_stock = false;
        self.down_up_stock = 0;
        self.flick_stock = false;
    }
}

// Contemporary SiglusEngine uses the WinMM JOYINFOEX button state as one of
// its raw joypad input backends. JOYINFOEX::dwButtons is a 32-bit button mask
// (JOY_BUTTON1..JOY_BUTTON32), so the script-visible raw joypad key domain must
// not truncate the backend at the 20 controls used by one particular game UI.
pub const JOYPAD_KEY_COUNT: usize = 32;

#[derive(Debug, Clone)]
pub struct InputState {
    keys: [KeyState; 256],
    // Newer Siglus builds expose the raw joypad button domain. Keep the complete
    // 32-button WinMM domain; individual games are free to iterate only the
    // subset they assign. The script-visible copy follows the same BUTTON
    // stock semantics as keyboard keys: held state plus down/up/down-up edges.
    joypad_keys: [KeyState; JOYPAD_KEY_COUNT],

    pub mouse_x: i32,
    pub mouse_y: i32,
    mouse_position_valid: bool,

    wheel_delta: i32,

    /// Last key-down event since start.
    pub last_key_down: Option<VmKey>,
    /// Last mouse-down event since start.
    pub last_mouse_down: Option<VmMouseButton>,

    /// Newer Siglus builds expose whether UI navigation is currently driven by
    /// a joypad. Keyboard/mouse activity switches this off; a platform gamepad
    /// bridge can switch it on through `note_joypad_activity`.
    joypad_mode_active: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            keys: [KeyState::new(); 256],
            joypad_keys: [KeyState::new(); JOYPAD_KEY_COUNT],
            mouse_x: -1,
            mouse_y: -1,
            mouse_position_valid: false,
            wheel_delta: 0,
            last_key_down: None,
            last_mouse_down: None,
            joypad_mode_active: false,
        }
    }
}

impl InputState {
    /// Returns the input-family state queried by the newer SYSCOM opcode 333.
    pub fn joypad_mode_active(&self) -> bool {
        self.joypad_mode_active
    }

    /// Called by a platform gamepad backend when a navigation-capable joypad
    /// input becomes the active UI device.
    pub fn note_joypad_activity(&mut self) {
        self.joypad_mode_active = true;
    }

    /// Returns whether a newer-engine joypad key is currently held.
    pub fn joypad_is_down(&self, key_no: usize) -> bool {
        self.joypad_keys.get(key_no).is_some_and(|st| st.down)
    }

    /// Returns the non-consuming down edge stock for a joypad key.
    pub fn joypad_down_stock(&self, key_no: usize) -> bool {
        self.joypad_keys.get(key_no).is_some_and(|st| st.down_stock)
    }

    /// Returns the non-consuming up edge stock for a joypad key.
    pub fn joypad_up_stock(&self, key_no: usize) -> bool {
        self.joypad_keys.get(key_no).is_some_and(|st| st.up_stock)
    }

    /// Returns the non-consuming completed down/up stock for a joypad key.
    pub fn joypad_down_up_stock(&self, key_no: usize) -> bool {
        self.joypad_keys
            .get(key_no)
            .is_some_and(|st| st.down_up_stock == 2)
    }

    /// Platform bridge entry point for a joypad-key press.  Keeping this in
    /// InputState means the VM semantics are complete even on platforms where
    /// no gamepad backend has been wired yet.
    pub fn on_joypad_key_down(&mut self, key_no: usize) {
        let Some(st) = self.joypad_keys.get_mut(key_no) else {
            return;
        };
        self.joypad_mode_active = true;
        if !st.down {
            st.down = true;
            st.down_stock = true;
        }
        if st.down_up_stock == 0 {
            st.down_up_stock = 1;
        }
        if st.down_stock && st.up_stock {
            st.down_up_stock = 2;
        }
    }

    /// Platform bridge entry point for a joypad-key release.
    pub fn on_joypad_key_up(&mut self, key_no: usize) {
        if key_no >= JOYPAD_KEY_COUNT {
            return;
        }
        self.joypad_mode_active = true;
        let st = &mut self.joypad_keys[key_no];
        if st.down {
            st.down = false;
            st.up_stock = true;
            if st.down_up_stock == 1 {
                st.down_up_stock = 2;
            }
        }
    }

    fn note_keyboard_mouse_activity(&mut self) {
        self.joypad_mode_active = false;
    }

    /// Drop transient input edges after returning from a native configuration
    /// UI. Held state is retained, matching `use_current`.
    pub fn resync_after_native_ui(&mut self) {
        self.use_current();
    }

    // ---------------------------------------------------------------------
    // Virtual key helpers
    // ---------------------------------------------------------------------

    /// Returns true if the given virtual-key is currently held down.
    pub fn vk_is_down(&self, vk: u8) -> bool {
        self.keys[vk as usize].down
    }

    /// Returns true if the key transitioned to down since the last `next_frame`.
    pub fn vk_down_stock(&self, vk: u8) -> bool {
        self.keys[vk as usize].down_stock
    }

    /// Returns true if the key transitioned to up since the last `next_frame`.
    pub fn vk_up_stock(&self, vk: u8) -> bool {
        self.keys[vk as usize].up_stock
    }

    /// Returns true if a down+up pair happened since the last `next_frame`.
    pub fn vk_down_up_stock(&self, vk: u8) -> bool {
        self.keys[vk as usize].down_up_stock == 2
    }

    /// Consume one key-down edge while preserving the held state.
    ///
    /// Matches `C_input_state::BUTTON::use_down_stock()`: consuming DOWN also
    /// invalidates any in-progress DOWN_UP sequence so the later release cannot
    /// be observed as a second logical input.
    pub fn use_vk_down_stock(&mut self, vk: u8) -> bool {
        let st = &mut self.keys[vk as usize];
        if !st.down_stock {
            return false;
        }
        st.down_stock = false;
        st.down_up_stock = 0;
        true
    }

    /// Consume one key-up edge. Matches tona3 `use_up_stock()`.
    pub fn use_vk_up_stock(&mut self, vk: u8) -> bool {
        let st = &mut self.keys[vk as usize];
        if !st.up_stock {
            return false;
        }
        st.up_stock = false;
        st.down_up_stock = 0;
        true
    }

    /// Consume a completed key down/up pair. Matches tona3
    /// `C_input_state::BUTTON::use_down_up_stock()`.
    pub fn use_vk_down_up_stock(&mut self, vk: u8) -> bool {
        let st = &mut self.keys[vk as usize];
        if st.down_up_stock != 2 {
            return false;
        }
        st.down_stock = false;
        st.up_stock = false;
        st.down_up_stock = 0;
        true
    }

    /// Returns true if a flick was detected since the last `next_frame`.
    pub fn vk_flick_stock(&self, vk: u8) -> bool {
        self.keys[vk as usize].flick_stock
    }

    /// Returns flick angle (radians) for the last flick on this key.
    pub fn vk_flick_angle(&self, vk: u8) -> f32 {
        self.keys[vk as usize].flick_angle
    }

    /// Returns flick distance in pixels for the last flick on this key.
    pub fn vk_flick_pixel(&self, vk: u8) -> f32 {
        self.keys[vk as usize].flick_pixel
    }

    /// Returns flick distance in millimeters for the last flick on this key.
    pub fn vk_flick_mm(&self, vk: u8) -> f32 {
        self.keys[vk as usize].flick_mm
    }

    fn vk_set_down(&mut self, vk: u8) {
        let st = &mut self.keys[vk as usize];
        if !st.down {
            st.down = true;
            st.down_stock = true;
        }
        if st.down_up_stock == 0 {
            st.down_up_stock = 1;
        }
        // If both edges happen within the same frame, mark the completed down-up stock.
        if st.down_stock && st.up_stock {
            st.down_up_stock = 2;
        }
    }

    fn vk_set_up(&mut self, vk: u8) {
        let st = &mut self.keys[vk as usize];
        if st.down {
            st.down = false;
            st.up_stock = true;
            if st.down_up_stock == 1 {
                st.down_up_stock = 2;
            }
        }
    }

    fn vk_set_flick_start(&mut self, vk: u8) {
        if !is_mouse_vk(vk) {
            return;
        }
        let st = &mut self.keys[vk as usize];
        st.flick_stock = false;
        st.flick_start = Some((self.mouse_x, self.mouse_y));
    }

    fn vk_set_flick_end(&mut self, vk: u8) {
        if !is_mouse_vk(vk) {
            return;
        }
        let st = &mut self.keys[vk as usize];
        let Some((sx, sy)) = st.flick_start.take() else {
            return;
        };
        let dx = (self.mouse_x - sx) as f32;
        let dy = (self.mouse_y - sy) as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < FLICK_MIN_PIXEL {
            return;
        }
        // Note: the original engine seems to treat angle as atan2(dx, dy).
        st.flick_angle = dx.atan2(dy);
        st.flick_pixel = dist;
        st.flick_mm = dist * MM_PER_PX;
        st.flick_stock = true;
    }

    /// Clears all keys (including held-down state) and all edge stocks.
    pub fn clear_all(&mut self) {
        for st in &mut self.keys {
            st.clear_all();
        }
        for st in &mut self.joypad_keys {
            st.clear_all();
        }
        self.wheel_delta = 0;
        self.last_key_down = None;
        self.last_mouse_down = None;
        self.joypad_mode_active = false;
    }

    pub fn has_mouse_position(&self) -> bool {
        self.mouse_position_valid
    }

    /// Clears only keyboard-visible state and leaves mouse state intact.
    pub fn clear_keyboard(&mut self) {
        for (idx, st) in self.keys.iter_mut().enumerate() {
            if matches!(idx as u8, 0x01 | 0x02 | 0x04) {
                continue;
            }
            st.clear_all();
        }
        self.last_key_down = None;
    }

    /// Clears only mouse-visible state and leaves keyboard state intact.
    pub fn clear_mouse(&mut self) {
        for vk in [0x01usize, 0x02usize, 0x04usize] {
            self.keys[vk].clear_all();
        }
        self.wheel_delta = 0;
        self.last_mouse_down = None;
    }

    /// Consumes current input edges while preserving held-down state.
    ///
    /// Mirrors tona3 C_input_state::use(): clear down/up/down_up/flick stocks
    /// for mouse and keyboard plus wheel, but do not release held keys.
    pub fn use_current(&mut self) {
        for st in &mut self.keys {
            st.use_stocks();
        }
        for st in &mut self.joypad_keys {
            st.use_stocks();
        }
        self.wheel_delta = 0;
        self.last_key_down = None;
        self.last_mouse_down = None;
    }

    /// Advances to the next frame: clears edge stocks but keeps held-down state.
    pub fn next_frame(&mut self) {
        for st in &mut self.keys {
            st.clear_stocks();
        }
        for st in &mut self.joypad_keys {
            st.clear_stocks();
        }
        self.wheel_delta = 0;
    }

    /// Advances only keyboard state to the next frame.
    pub fn next_keyboard_frame(&mut self) {
        for (idx, st) in self.keys.iter_mut().enumerate() {
            if matches!(idx as u8, 0x01 | 0x02 | 0x04) {
                continue;
            }
            st.clear_stocks();
        }
        self.last_key_down = None;
    }

    /// Advances only mouse state to the next frame.
    pub fn next_mouse_frame(&mut self) {
        for vk in [0x01usize, 0x02usize, 0x04usize] {
            self.keys[vk].clear_stocks();
        }
        self.wheel_delta = 0;
        self.last_mouse_down = None;
    }

    // ---------------------------------------------------------------------
    // Wheel
    // ---------------------------------------------------------------------

    pub fn on_mouse_wheel(&mut self, delta_y: i32) {
        self.note_keyboard_mouse_activity();
        self.wheel_delta = self.wheel_delta.saturating_add(delta_y);
    }

    /// Reads and clears the accumulated wheel delta.
    pub fn take_wheel_delta(&mut self) -> i32 {
        let v = self.wheel_delta;
        self.wheel_delta = 0;
        v
    }

    // ---------------------------------------------------------------------
    // Bridge from platform key/mouse events
    // ---------------------------------------------------------------------

    pub fn is_key_down(&self, k: VmKey) -> bool {
        vmkey_to_vk(k)
            .map(|vk| self.vk_is_down(vk))
            .unwrap_or(false)
    }

    pub fn is_mouse_down(&self, b: VmMouseButton) -> bool {
        match b {
            VmMouseButton::Left => self.vk_is_down(0x01),
            VmMouseButton::Right => self.vk_is_down(0x02),
            VmMouseButton::Middle => self.vk_is_down(0x04),
            VmMouseButton::Other(_) => false,
        }
    }

    pub fn on_key_down(&mut self, k: VmKey) {
        self.note_keyboard_mouse_activity();
        if let Some(vk) = vmkey_to_vk(k) {
            self.vk_set_down(vk);
        }
        self.last_key_down = Some(k);
    }

    pub fn on_key_up(&mut self, k: VmKey) {
        if let Some(vk) = vmkey_to_vk(k) {
            self.vk_set_up(vk);
        }
    }

    pub fn on_mouse_down(&mut self, b: VmMouseButton) {
        self.note_keyboard_mouse_activity();
        match b {
            VmMouseButton::Left => {
                self.vk_set_down(0x01);
                self.vk_set_flick_start(0x01);
            }
            VmMouseButton::Right => {
                self.vk_set_down(0x02);
                self.vk_set_flick_start(0x02);
            }
            VmMouseButton::Middle => {
                self.vk_set_down(0x04);
                self.vk_set_flick_start(0x04);
            }
            VmMouseButton::Other(_) => {}
        }
        self.last_mouse_down = Some(b);
    }

    pub fn on_mouse_up(&mut self, b: VmMouseButton) {
        match b {
            VmMouseButton::Left => {
                self.vk_set_flick_end(0x01);
                self.vk_set_up(0x01);
            }
            VmMouseButton::Right => {
                self.vk_set_flick_end(0x02);
                self.vk_set_up(0x02);
            }
            VmMouseButton::Middle => {
                self.vk_set_flick_end(0x04);
                self.vk_set_up(0x04);
            }
            VmMouseButton::Other(_) => {}
        }
    }

    pub fn on_mouse_move(&mut self, x: i32, y: i32) {
        self.note_keyboard_mouse_activity();
        self.mouse_x = x;
        self.mouse_y = y;
        self.mouse_position_valid = true;
    }

    /// Returns a direction bitmask based on arrow keys.
    ///
    /// Bit layout:
    ///   1=left, 2=right, 4=up, 8=down
    pub fn dir_mask(&self) -> i64 {
        let mut m = 0;
        if self.vk_is_down(0x25) {
            m |= 1;
        }
        if self.vk_is_down(0x27) {
            m |= 2;
        }
        if self.vk_is_down(0x26) {
            m |= 4;
        }
        if self.vk_is_down(0x28) {
            m |= 8;
        }
        m
    }
}

pub(crate) fn vmkey_to_vk_code(k: VmKey) -> Option<u8> {
    vmkey_to_vk(k)
}

fn vmkey_to_vk(k: VmKey) -> Option<u8> {
    match k {
        VmKey::Escape => Some(0x1B),
        VmKey::Enter => Some(0x0D),
        VmKey::Space => Some(0x20),
        VmKey::Backspace => Some(0x08),
        VmKey::Delete => Some(0x2E),
        VmKey::Tab => Some(0x09),
        VmKey::Shift => Some(0x10),
        VmKey::Control => Some(0x11),
        VmKey::Meta => Some(0x5B),
        VmKey::Alt => Some(0x12),
        VmKey::Home => Some(0x24),
        VmKey::End => Some(0x23),

        VmKey::ArrowLeft => Some(0x25),
        VmKey::ArrowUp => Some(0x26),
        VmKey::ArrowRight => Some(0x27),
        VmKey::ArrowDown => Some(0x28),

        VmKey::F(n @ 1..=12) => Some(0x6F + n), // F1=0x70
        VmKey::Digit(n @ 0..=9) => Some(0x30 + n),
        VmKey::Letter(c) => {
            let uc = c.to_ascii_uppercase();
            if uc.is_ascii_uppercase() {
                Some(uc as u8)
            } else {
                None
            }
        }
        VmKey::Other(_) => None,
        _ => None,
    }
}

fn is_mouse_vk(vk: u8) -> bool {
    matches!(vk, 0x01 | 0x02 | 0x04)
}

const FLICK_MIN_PIXEL: f32 = 30.0;
const MM_PER_PX: f32 = 25.4 / 96.0;

#[cfg(test)]
mod joypad_mode_tests {
    use super::{InputState, VmKey};

    #[test]
    fn keyboard_activity_leaves_joypad_mode() {
        let mut input = InputState::default();
        input.note_joypad_activity();
        assert!(input.joypad_mode_active());
        input.on_key_down(VmKey::Enter);
        assert!(!input.joypad_mode_active());
    }

    #[test]
    fn native_ui_resync_consumes_edges_but_keeps_mode() {
        let mut input = InputState::default();
        input.note_joypad_activity();
        input.on_key_down(VmKey::Enter);
        input.note_joypad_activity();
        input.resync_after_native_ui();
        assert!(input.joypad_mode_active());
        assert!(!input.vk_down_stock(0x0d));
        assert!(input.vk_is_down(0x0d));
    }

    #[test]
    fn joypad_key_uses_button_stock_lifecycle() {
        let mut input = InputState::default();
        input.on_joypad_key_down(0);
        assert!(input.joypad_mode_active());
        assert!(input.joypad_is_down(0));
        assert!(input.joypad_down_stock(0));
        assert!(!input.joypad_down_up_stock(0));

        input.next_frame();
        assert!(input.joypad_is_down(0));
        assert!(!input.joypad_down_stock(0));

        input.on_joypad_key_up(0);
        assert!(!input.joypad_is_down(0));
        assert!(input.joypad_up_stock(0));
        assert!(input.joypad_down_up_stock(0));
    }

    #[test]
    fn consuming_down_prevents_release_from_becoming_down_up() {
        let mut input = InputState::default();
        input.on_mouse_down(super::VmMouseButton::Left);
        assert!(input.vk_down_stock(0x01));
        assert!(input.use_vk_down_stock(0x01));
        assert!(!input.vk_down_stock(0x01));

        input.on_mouse_up(super::VmMouseButton::Left);
        assert!(input.vk_up_stock(0x01));
        assert!(!input.vk_down_up_stock(0x01));
    }

    #[test]
    fn consuming_up_or_down_up_matches_tona_button_stock_semantics() {
        let mut input = InputState::default();
        input.on_key_down(VmKey::Enter);
        input.on_key_up(VmKey::Enter);
        assert!(input.vk_down_up_stock(0x0d));
        assert!(input.use_vk_up_stock(0x0d));
        assert!(!input.vk_up_stock(0x0d));
        assert!(!input.vk_down_up_stock(0x0d));

        input.on_key_down(VmKey::Enter);
        input.on_key_up(VmKey::Enter);
        assert!(input.use_vk_down_up_stock(0x0d));
        assert!(!input.vk_down_stock(0x0d));
        assert!(!input.vk_up_stock(0x0d));
        assert!(!input.vk_down_up_stock(0x0d));
    }

    #[test]
    fn joypad_keeps_complete_32_button_backend_domain() {
        let mut input = InputState::default();

        // WinMM JOYINFOEX::dwButtons exposes JOY_BUTTON1..JOY_BUTTON32.
        // The last backend button therefore maps to script key index 31.
        input.on_joypad_key_down(31);
        assert!(input.joypad_is_down(31));
        assert!(input.joypad_down_stock(31));

        input.on_joypad_key_up(31);
        assert!(!input.joypad_is_down(31));
        assert!(input.joypad_up_stock(31));

        // Index 32 is outside the actual backend domain and must not alias or
        // grow the table implicitly.
        input.on_joypad_key_down(32);
        assert!(!input.joypad_is_down(32));
    }
}
