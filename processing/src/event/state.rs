use lazy_static::lazy_static;
use winit::event::MouseButton;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};

// Defines all the various events that can be triggered
// and listened to
#[derive(Debug, Hash, PartialEq, Eq, Default, Copy, Clone)]
#[repr(C)]
pub enum PEvent {
    PMousePressed,
    PMouseReleased,
    PMouseMoved,
    #[default]
    NoEvent
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub enum PMouseButton {
    LeftButton,
    RightButton,
    MiddleButton,
    #[default]
    NoButton
}

impl PMouseButton {
    pub fn from (button: MouseButton) -> Self {
        match button {
            MouseButton::Left => PMouseButton::LeftButton,
            MouseButton::Right => PMouseButton::RightButton,
            MouseButton::Middle => PMouseButton::MiddleButton,
            MouseButton::Other(_) => PMouseButton::NoButton
        }
    }
}

// Type of callback function for event
// Currently, no data is passed to it as all
// event info is made available through
// functions like keyCode(), mouseButton(), etc
pub type PEventCallback = extern "C" fn() -> ();


#[derive(Default)]
pub struct EventState {
    pub setup: Option<PEventCallback>,
    pub draw: Option<PEventCallback>,

    pub mouse_x: f32,
    pub mouse_y: f32,
    pub key_code: u32,
    pub mouse_button: PMouseButton,

    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
    pub shift_pressed: bool,
    
    pub events: HashMap<PEvent, PEventCallback>
}

impl EventState {
    pub fn get_handler (&self, event: PEvent) -> Option<&PEventCallback> {
        self.events.get(&event)
    }
}

lazy_static! {
    pub static ref EVENT_STATE: Arc<RwLock<EventState>> = Arc::new(RwLock::new(EventState::default()));
}

pub fn get_event_state () -> RwLockReadGuard<'static, EventState> {
    EVENT_STATE.try_read().unwrap()
}



macro_rules! set_event_state {

    // base cases
    ($var:ident$(.$var2:ident)* = $value:expr;) => {
        {
            crate::event::state::EVENT_STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)* = $value;
        }
    };
    ($var:ident$(.$var2:ident)*($value:expr$(,$other:expr)*);) => {
        {
            crate::event::state::EVENT_STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)*($value$(,$other)*);
        }
    };

    // expr - expr
    ($var:ident$(.$var2:ident)* = $value:expr; $($var3:ident$(.$var4:ident)* = $value2:expr;)*) => {
        set_event_state!{ $var$(.$var2)* = $value; };
        set_event_state!{ $($var3$(.$var4)* = $value2;)* };
    };

    // fn - expr
    ($var:ident$(.$var2:ident)*($value:expr$(,$other:expr)*); $($var3:ident$(.$var4:ident)* = $value2:expr;)*) => {
        set_event_state!{ $var$(.$var2)*($value$(,$other)*); };
        set_event_state!{ $($var3$(.$var4)* = $value2;)* };
    };

    // expr - fn
    ($var:ident$(.$var2:ident)* = $value:expr; $($var3:ident$(.$var4:ident)*($value2:expr$(,$other:expr)*);)*) => {
        set_event_state!{ $var$(.$var2)* = $value; };
        set_event_state!{ $($var3$(.$var4)*($value2$(,$other)*);)* };
    };  

    // fn - fn
    ($var:ident$(.$var2:ident)*($value:expr$(,$other:expr)*); $($var3:ident$(.$var4:ident)*($value2:expr$(,$other2:expr)*);)*) => {
        set_event_state!{ $var$(.$var2)*($value$(,$other)*); };
        set_event_state!{ $($var3$(.$var4)*($value2$(,$other2)*);)* };
    };  
}
pub(crate) use set_event_state;
