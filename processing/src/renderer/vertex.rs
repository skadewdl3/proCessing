#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new (x: f32, y: f32, z: f32) -> Self {
        let point = Vertex {
            position: [x, y, z]
        };
        println!("bruh: {:?}", point);
        point
    }
}


macro_rules! normalized_vtx {

    ($x:expr, $y:expr) => {{
        
        crate::renderer::vertex::Vertex::new(
            $x as f32 / crate::renderer::state::get_renderer_state().width.expect("Width of window has not been set") as f32 * 2.0 - 1.0,
            -($y as f32 / crate::renderer::state::get_renderer_state().height.expect("Height of window has not been set") as f32 * 2.0 - 1.0),
            0.0
        )
    }};

	($x:expr, $y:expr, $z:expr) => {{

        crate::core::vertex::Vertex::new(
            $x as f32 / crate::renderer::state::get_renderer_state().width.expect("Width of window has not been set") as f32 * 2.0 - 1.0,
            -($y as f32 / crate::renderer::state::get_renderer_state().height.expect("Height of window has not been set") as f32 * 2.0 - 1.0),
            $z as f32
        ) 
    }};
}

use bytemuck::{Pod, Zeroable};
pub(crate) use normalized_vtx;
