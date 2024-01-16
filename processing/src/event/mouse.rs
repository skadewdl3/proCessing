use winit::event::{ElementState, MouseButton};

use crate::event::state::get_event_state;

use super::state::{PMouseButton, PEvent, set_event_state};

pub fn handle_mouse_event (state: ElementState, button: MouseButton) {
    set_event_state! {
        mouse_button = PMouseButton::from(button);
    }
    match state {
        ElementState::Pressed => {
            if let Some(handler) = get_event_state().get_handler(PEvent::PMousePressed) { handler() }
        }

        ElementState::Released => {
            if let Some(handler) = get_event_state().get_handler(PEvent::PMouseReleased) { handler() }
        }
    }
}

#[no_mangle]
pub extern "C" fn mouseX () -> f32 {
    let state = get_event_state();
    state.mouse_x
}

#[no_mangle]
pub extern "C" fn mouseY () -> f32 {
    let state = get_event_state();
    state.mouse_y
}

#[no_mangle]
pub extern "C" fn mouseButton () -> PMouseButton {
    let state = get_event_state();
    state.mouse_button
}