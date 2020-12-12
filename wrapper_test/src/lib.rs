pub use golem;

pub fn log(txt: impl Into<&'static str>) {
    let txt = txt.into();
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", txt);
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&txt.into());
}

pub type InitFn<Model> = fn(&App) -> Model;
pub type UpdateFn<Model> = fn(&App, &mut Model);
pub type ViewFn<Model> = fn(&App, &Model);

pub struct Builder<Model> {
    init_fn: InitFn<Model>,
    update_fn: UpdateFn<Model>,
    view_fn: ViewFn<Model>,
}
impl<Model: 'static> Builder<Model> {
    pub fn init(init_fn: InitFn<Model>) -> Builder<Model> {
        Builder {
            init_fn: init_fn,
            update_fn: |_, _| {},
            view_fn: |_, _| {},
        }
    }
    pub fn update(self, update_fn: UpdateFn<Model>) -> Builder<Model> {
        Builder { update_fn, ..self }
    }
    pub fn view(self, view_fn: ViewFn<Model>) -> Builder<Model> {
        Builder { view_fn, ..self }
    }
    pub fn run(self) {
        App::new().run(self);
    }
}

#[cfg(target_arch = "wasm32")]
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    window::Window,
};

#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    window::Window,
    PossiblyCurrent, WindowedContext,
};

pub struct App {
    pub draw: golem::Context,
    pub shader_version: String,

    el: Option<EventLoop<()>>,

    #[cfg(target_arch = "wasm32")]
    window: Window,
    #[cfg(not(target_arch = "wasm32"))]
    windowed_context: WindowedContext<PossiblyCurrent>,
}

impl App {
    pub fn new() -> App {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowExtWebSys;
            use winit::window::WindowBuilder;

            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title("A fantastic window!")
                .build(&event_loop)
                .unwrap();

            let canvas = window.canvas();

            {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();
                body.append_child(&canvas)
                    .expect("Append canvas to HTML body");
            }

            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            let context = golem::glow::Context::from_webgl2_context(webgl2_context);

            App {
                draw: golem::Context::from_glow(context).unwrap(),
                el: Some(event_loop),
                window: window,
                shader_version: "#version 300 es".to_string(),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use glutin::window::WindowBuilder;

            let event_loop = EventLoop::new();

            let wb = WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(wb, &event_loop)
                .unwrap();

            let windowed_context = unsafe { windowed_context.make_current().unwrap() };

            let context = unsafe {
                golem::glow::Context::from_loader_function(|s| {
                    windowed_context.get_proc_address(s) as *const _
                })
            };

            App {
                draw: golem::Context::from_glow(context).unwrap(),
                el: Some(event_loop),
                windowed_context: windowed_context,
                shader_version: "#version 410".to_string(),
            }
        }
    }

    pub fn run<Model: 'static>(mut self, builder: Builder<Model>) {
        let Builder {
            init_fn,
            update_fn,
            view_fn,
        } = builder;
        let mut model = init_fn(&self);
        let event_loop = self.el.take().unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    update_fn(&self, &mut model);

                    #[cfg(not(target_arch = "wasm32"))]
                    self.windowed_context.window().request_redraw();
                    #[cfg(target_arch = "wasm32")]
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    view_fn(&self, &model);

                    #[cfg(not(target_arch = "wasm32"))]
                    self.windowed_context.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    #[cfg(not(target_arch = "wasm32"))]
                    WindowEvent::Resized(physical_size) => {
                        self.windowed_context.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
