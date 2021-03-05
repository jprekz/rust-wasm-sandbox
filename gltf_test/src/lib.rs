#![allow(dead_code)]

mod fps_counter;
mod golem_gltf;
mod time;

use blinds::*;
use fps_counter::FpsCounter;
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
    let mut mouse_dragging = false;
    let mut mouse_location = glm::zero::<glm::Vec2>();

    let mut m_matrix: glm::Mat4 = glm::identity();

    let mut v_matrix = make_v_matrix(scroll_absolute);

    let mut p_matrix = make_p_matrix(window_size);

    let mut fps_counter = FpsCounter::new(ctx);

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
                Event::PointerInput(e) => {
                    if e.button() == MouseButton::Left {
                        mouse_dragging = e.is_down();
                    }
                }
                Event::PointerMoved(e) => {
                    let mouse_location_n = glm::make_vec2(e.location().as_ref());
                    let mouse_moved = if mouse_dragging {
                        mouse_location_n - mouse_location
                    } else {
                        glm::zero()
                    };
                    m_matrix = glm::rotate_x(&m_matrix, mouse_moved.y / 100.0);
                    m_matrix = glm::rotate_y(&m_matrix, mouse_moved.x / 100.0);
                    mouse_location = mouse_location_n;
                    v_matrix = make_v_matrix(scroll_absolute);
                }
                _ => {}
            }
        }

        fps_counter.count();

        ctx.set_clear_color(0.1, 0.2, 0.3, 1.0);
        ctx.clear();

        ctx.set_depth_test_mode(Some(depth::DepthTestMode::default()));
        let mvp_matrix = p_matrix * v_matrix * m_matrix;
        gltf_model.draw(&mvp_matrix)?;
        ctx.set_depth_test_mode(None);
        fps_counter.draw(&p_matrix)?;

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
