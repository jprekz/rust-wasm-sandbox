use glow::*;

pub fn main() {
    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(target_arch = "wasm32")]
        let (gl, event_loop, window, shader_version) = {
            use winit::{event_loop::EventLoop, window::WindowBuilder};

            let event_loop = EventLoop::new();

            let window = WindowBuilder::new()
                .with_title("A fantastic window!")
                .build(&event_loop)
                .unwrap();

            use winit::platform::web::WindowExtWebSys;

            let canvas = window.canvas();

            let wwindow = web_sys::window().unwrap();
            let document = wwindow.document().unwrap();
            let body = document.body().unwrap();

            body.append_child(&canvas)
                .expect("Append canvas to HTML body");

            use wasm_bindgen::JsCast;

            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            (
                glow::Context::from_webgl2_context(webgl2_context),
                event_loop,
                window,
                "#version 300 es",
            )
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(feature = "window-glutin")]
        let (gl, event_loop, windowed_context, shader_version) = {
            let el = glutin::event_loop::EventLoop::new();
            let wb = glutin::window::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(wb, &el)
                .unwrap();
            let windowed_context = windowed_context.make_current().unwrap();
            let context = unsafe {
                glow::Context::from_loader_function(|s| {
                    windowed_context.get_proc_address(s) as *const _
                })
            };
            (context, el, windowed_context, "#version 410")
        };

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        #[cfg(feature = "window-glutin")]
        {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
                match event {
                    Event::LoopDestroyed => {
                        return;
                    }
                    Event::MainEventsCleared => {
                        windowed_context.window().request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        println!("redraw requested");
                        gl.clear(glow::COLOR_BUFFER_BIT);
                        gl.draw_arrays(glow::TRIANGLES, 0, 3);
                        windowed_context.swap_buffers().unwrap();
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            windowed_context.resize(*physical_size);
                        }
                        WindowEvent::CloseRequested => {
                            gl.delete_program(program);
                            gl.delete_vertex_array(vertex_array);
                            *control_flow = ControlFlow::Exit
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        }

        #[cfg(not(feature = "window-glutin"))]
        {
            use winit::{
                event::{Event, WindowEvent},
                event_loop::ControlFlow,
            };

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
                match event {
                    Event::LoopDestroyed => {
                        return;
                    }
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        web_sys::console::log_1(&"redraw requested".into());
                        gl.clear(glow::COLOR_BUFFER_BIT);
                        gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            gl.delete_program(program);
                            gl.delete_vertex_array(vertex_array);
                            *control_flow = ControlFlow::Exit
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }
}
