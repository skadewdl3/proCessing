use std::time::{Instant, Duration};

use wgpu::{Instance, InstanceDescriptor, Backends, RequestAdapterOptions, DeviceDescriptor, SurfaceConfiguration, TextureUsages, PresentMode, Color, IndexFormat};
use winit::{event_loop::{EventLoopBuilder, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, monitor::MonitorHandle, event::{Event, WindowEvent}};

use crate::{renderer::{state::{get_renderer_state, set_renderer_state}, shader::ShaderBuilder, vertex::Vertex}, event::state::get_event_state, event::handle_event, renderer::vertex::normalized_vtx};

pub async fn start_event_loop () {
    let renderer_state = get_renderer_state();

    // Get dimensions for window from renderer state
    // as specified by createWindow call
    let width =
        renderer_state
        .width
        .expect("No width has been set. Call the createWindow() function to set a width.");

    let height =
        renderer_state
        .height
        .expect("No height has been set. Call the createWindow() function to set a height.");


    // Create the event loop and window instances
    let event_loop = EventLoopBuilder::new().build();
    let window =
        WindowBuilder::new()
        .with_title("processing") // get this title from renderer state later on
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)
        .expect("Error while creating window.");

    drop(renderer_state);

    // Get the maximum frame rate of monitor
    // to prevent user from setting a higher frame rate
    let monitors: Vec<MonitorHandle> = window.available_monitors().collect();
    if monitors.len() > 0 {
        let first_monitor =
            monitors
            .get(0)
            .expect("Could not get handle to monitor to check for frame rate.");

        if let Some(max_fps) = first_monitor.refresh_rate_millihertz() {
            set_renderer_state! {
                max_fps = max_fps;
            }
        }
    }

    let size = window.inner_size();
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });

    let surface =
        unsafe { instance.create_surface(&window) }
        .expect("Error while creating a surface for the window");

    let adapter =
        instance
        .request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            ..Default::default()
        })
        .await
        .expect("Errpr while gettnig adapter from the GPU");

    let (device, queue) =
        adapter
        .request_device(&DeviceDescriptor::default(), None)
        .await
        .expect("Could not get device and queue handle from the GPU");

    let surface_capabilities = surface.get_capabilities(&adapter);

    let format =
        *surface_capabilities
        .formats
        .get(0)
        .expect("Could not get texture format from the surface");

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: *surface_capabilities.alpha_modes.get(0).unwrap(),
        view_formats: vec![]
    };

    surface.configure(&device, &config);

    set_renderer_state! {
        device = Some(device);
        queue = Some(queue);
        window = Some(window);
        surface = Some(surface);
        last_redraw_time = Some(Instant::now());
    }


    let tgl_shader = ShaderBuilder::new()
        .with_content(include_str!("../shaders/rect.wgsl"))
        .with_label("Rectangle Shader")
        .with_vertex_buffer(vec![
            normalized_vtx!(0, 0),
            normalized_vtx!(100, 0),
            normalized_vtx!(0, 100),
            normalized_vtx!(100, 100),

        ])
        .with_index_buffer(vec![0, 1, 2, 2, 1, 3])
        .build();


    set_renderer_state! {
        shaders.push(tgl_shader);
    }
    

    event_loop.run(move |event, _, control_flow| {
        
        
        // set_renderer_state! {
        //     shaders = vec![
        //     ];
        // }

        let current_time = Instant::now();
        let delta = current_time.duration_since(get_renderer_state().last_redraw_time.unwrap());
        let target_delta = Duration::from_secs_f64(1.0 / get_renderer_state().target_fps as f64);

        if delta >=  target_delta {
            let draw = 
                get_event_state()
                .draw
                .expect("No draw function specified. Call the p_init() function to set a draw function.");
            draw();

            let renderer_state = get_renderer_state();
            let device = renderer_state.device.as_ref().expect("No device specified");
            let queue = renderer_state.queue.as_ref().expect("No queue specified");
            let surface = renderer_state.surface.as_ref().expect("No surface specified");

            let frame = surface.get_current_texture().unwrap();                
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None
                });

                // loop over shaders and draw them here
                for shader in &renderer_state.shaders {
                    rpass.set_pipeline(&shader.pipeline);
                    if let Some(vertex_buffer) = &shader.vertex_buffer {
                        rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    }
                    if let Some(index_buffer) = &shader.index_buffer {
                        rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
                    }

                    if shader.has_index_buffer {
                        rpass.draw_indexed(0..shader.draw_count, 0, 0..1)
                    }
                    else {
                        rpass.draw(0..shader.draw_count, 0..1);
                    }
                }

            }
        
            queue.submit(Some(encoder.finish()));
            frame.present();
            drop(renderer_state);

            set_renderer_state! {
                last_redraw_time = Some(current_time);
            }
        }
        // get_renderer_state().window.as_ref().unwrap().request_redraw();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent { window_id, event } => handle_event(window_id, event),
            _ => ()
        }

    });
}