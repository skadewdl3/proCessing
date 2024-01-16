use crate::renderer::state::{set_renderer_state, get_renderer_state};

#[no_mangle]
pub extern "C" fn createWindow (width: f32, height: f32) {
    set_renderer_state! {
        width = Some(width);
        height = Some(height);
    }
}

#[no_mangle]
pub extern "C" fn width () -> f32 {
    let state = get_renderer_state();
    state.width.expect("No width has been set in createWindow()")
}

#[no_mangle]
pub extern "C" fn height () -> f32 {
    let state = get_renderer_state();
    state.height.expect("No height has been set in createWindow()")
}