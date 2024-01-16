use winit::{window::WindowId, event::WindowEvent};
use crate::event::{mouse::handle_mouse_event, state::{get_event_state, PEvent}};

use self::state::set_event_state;


pub mod state;
pub mod mouse;

pub fn handle_event (_id: WindowId, event: WindowEvent) {
    match event {
        WindowEvent::MouseInput { state, button, .. } => handle_mouse_event(state, button),


        WindowEvent::CursorMoved { position, .. } => {
            set_event_state! {
                mouse_x = position.x as f32;
                mouse_y = position.y as f32;
            }
            if let Some(handler) = get_event_state().get_handler(PEvent::PMouseMoved){ handler() }
        }
        _ => ()
    }
}