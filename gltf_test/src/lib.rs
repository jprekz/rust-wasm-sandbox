mod fps_counter;
mod golem_gltf;

use blinds::*;
use golem::*;
use nalgebra_glm as glm;

use wasm_bindgen::prelude::*;
#[wasm_bindgen(start)]
pub fn start() {
    let settings = Settings {
        resizable: true,
        ..Settings::default()
    };
    run(settings, |window, events| async move {
        app(window, events).await.unwrap()
    });
}

fn context_from_blinds(window: &Window) -> Result<Context, GolemError> {
    #[cfg(not(target_arch = "wasm32"))]
    let glow_ctx = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
    #[cfg(target_arch = "wasm32")]
    let glow_ctx = glow::Context::from_webgl1_context(window.webgl_context());

    Context::from_glow(glow_ctx)
}

#[cfg(target_arch = "wasm32")]
async fn js_fetch(url: impl AsRef<str>) -> Vec<u8> {
    use js_sys::Uint8Array;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, Response};

    let opts = RequestInit::new();
    let request = Request::new_with_str_and_init(url.as_ref(), &opts).unwrap();
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();
    let arrbuff_value = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
    Uint8Array::new(&arrbuff_value).to_vec()
}

async fn app(window: Window, mut events: EventStream) -> Result<(), GolemError> {
    let ctx = &context_from_blinds(&window)?;

    let mut gltf_model = {
        #[cfg(target_arch = "wasm32")]
        {
            let data = js_fetch("test.glb").await;
            golem_gltf::Gltf::load_slice(&data, ctx).unwrap()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            golem_gltf::Gltf::load("test.glb", ctx).unwrap()
        }
    };

    window.present();

    let mut scroll_absolute = 10.0f32;
    let mut window_size = glm::make_vec2(window.size().as_ref()) * window.scale_factor();

    let m_matrix: glm::Mat4 = glm::identity();

    let mut v_matrix = make_v_matrix(scroll_absolute);

    let mut p_matrix = make_p_matrix(window_size);

    let mut prev_time = std::time::Instant::now();
    let mut frame_count = 0usize;

    loop {
        while let Some(event) = events.next_event().await {
            use blinds::event::*;

            match event {
                Event::Resized(size) => {
                    window_size =
                        glm::make_vec2(size.logical_size().as_ref()) * window.scale_factor();
                    ctx.set_viewport(0, 0, window_size.x as u32, window_size.y as u32);
                    p_matrix = make_p_matrix(window_size);
                }
                Event::ScaleFactorChanged(scale) => {
                    window_size = glm::make_vec2(window.size().as_ref()) * scale.scale_factor();
                    ctx.set_viewport(0, 0, window_size.x as u32, window_size.y as u32);
                    p_matrix = make_p_matrix(window_size);
                }
                Event::ScrollInput(ScrollDelta::Lines(delta)) => {
                    scroll_absolute -= delta.y;
                    v_matrix = make_v_matrix(scroll_absolute);
                }
                _ => {}
            }
        }

        frame_count += 1;
        if prev_time.elapsed() >= std::time::Duration::new(1, 0) {
            prev_time = std::time::Instant::now();
            println!("{}", frame_count);
            frame_count = 0;
        }

        ctx.set_clear_color(0.1, 0.2, 0.3, 1.0);
        ctx.clear();

        let mvp_matrix = p_matrix * v_matrix * m_matrix;
        gltf_model.draw(&mvp_matrix)?;

        window.present();
    }
}

fn make_p_matrix(window_size: glm::Vec2) -> glm::Mat4 {
    glm::perspective(
        window_size.x / window_size.y,
        std::f32::consts::PI / 2.0,
        0.1,
        100.0,
    )
}

fn make_v_matrix(scroll_absolute: f32) -> glm::Mat4 {
    glm::look_at(
        &glm::vec3(0.0, 0.0, 1.1_f32.powf(scroll_absolute)),
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 1.0, 0.0),
    )
}
