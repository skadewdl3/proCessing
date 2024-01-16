use std::{sync::{RwLock, Arc, RwLockReadGuard}, time::Instant};
use wgpu::{Device, Queue, Surface};
use winit::window::Window;
use crate::renderer::shader::Shader;
use lazy_static::lazy_static;

#[derive(Default)]
pub struct RendererState {
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub window: Option<Window>,
    pub surface: Option<Surface>,

    pub last_redraw_time: Option<Instant>,
    
    pub max_fps: u32,
    pub target_fps: u32,
    
    pub width: Option<f32>,
    pub height: Option<f32>,

    pub shaders: Vec<Shader>
}

lazy_static! {
    pub static ref RENDERER_STATE: Arc<RwLock<RendererState>> = Arc::new(RwLock::new(RendererState {
        target_fps: 60,
        max_fps: 60,
        ..Default::default()
    }));
}

pub fn get_renderer_state() -> RwLockReadGuard<'static, RendererState> {
    RENDERER_STATE.try_read().unwrap()
}

macro_rules! set_renderer_state {

    // base cases
    ($var:ident$(.$var2:ident)* = $value:expr;) => {
        {
            crate::renderer::state::RENDERER_STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)* = $value;
        }
    };
    ($var:ident$(.$var2:ident)*($value:expr$(,$other:expr)*);) => {
        {
            crate::renderer::state::RENDERER_STATE.try_write().expect("Could not write to RwLock").$var$(.$var2)*($value$(,$other)*);
        }
    };

    // expr - expr
    ($var:ident$(.$var2:ident)* = $value:expr; $($var3:ident$(.$var4:ident)* = $value2:expr;)*) => {
        set_renderer_state!{ $var$(.$var2)* = $value; };
        set_renderer_state!{ $($var3$(.$var4)* = $value2;)* };
    };

    // fn - expr
    ($var:ident$(.$var2:ident)*($value:expr$(,$other:expr)*); $($var3:ident$(.$var4:ident)* = $value2:expr;)*) => {
        set_renderer_state!{ $var$(.$var2)*($value$(,$other)*); };
        set_renderer_state!{ $($var3$(.$var4)* = $value2;)* };
    };

    // expr - fn
    ($var:ident$(.$var2:ident)* = $value:expr; $($var3:ident$(.$var4:ident)*($value2:expr$(,$other:expr)*);)*) => {
        set_renderer_state!{ $var$(.$var2)* = $value; };
        set_renderer_state!{ $($var3$(.$var4)*($value2$(,$other)*);)* };
    };  

    // fn - fn
    ($var:ident$(.$var2:ident)*($value:expr$(,$other:expr)*); $($var3:ident$(.$var4:ident)*($value2:expr$(,$other2:expr)*);)*) => {
        set_renderer_state!{ $var$(.$var2)*($value$(,$other)*); };
        set_renderer_state!{ $($var3$(.$var4)*($value2$(,$other2)*);)* };
    };  
}
pub(crate) use set_renderer_state;