//! Shared IME requests for native and Web windows.

use winit::dpi::{Position, Size};
use winit::window::{
    ImeCapabilities, ImeEnableRequest, ImeHint, ImePurpose, ImeRequest, ImeRequestData, Window,
};

/// Enable text input or update the candidate window area without restarting composition.
pub fn enable_ime(window: &dyn Window, position: Position, size: Size) {
    let data = ImeRequestData::default().with_cursor_area(position, size);
    let request = if window.ime_capabilities().is_some() {
        ImeRequest::Update(data)
    } else {
        let capabilities = ImeCapabilities::new()
            .with_cursor_area()
            .with_hint_and_purpose();
        let data = data.with_hint_and_purpose(ImeHint::NONE, ImePurpose::Normal);
        ImeRequest::Enable(
            ImeEnableRequest::new(capabilities, data)
                .expect("IME capabilities match the initial cursor area, hint and purpose"),
        )
    };
    // Some backends do not support IME; keep text input best-effort as before.
    let _ = window.request_ime_update(request);
}

/// Disable text input when no editable field is focused.
pub fn disable_ime(window: &dyn Window) {
    let _ = window.request_ime_update(ImeRequest::Disable);
}
