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

    let mut scroll_absolute = 1.0f32;
    let mut window_size = glm::make_vec2(window.size().as_ref()) * window.scale_factor();

    loop {
        while let Some(event) = events.next_event().await {
            use blinds::event::*;

            match event {
                Event::Resized(size) => {
                    window_size =
                        glm::make_vec2(size.logical_size().as_ref()) * window.scale_factor();
                    ctx.set_viewport(0, 0, window_size.x as u32, window_size.y as u32);
                }
                Event::ScaleFactorChanged(scale) => {
                    window_size = glm::make_vec2(window.size().as_ref()) * scale.scale_factor();
                    ctx.set_viewport(0, 0, window_size.x as u32, window_size.y as u32);
                }
                Event::ScrollInput(ScrollDelta::Lines(delta)) => {
                    scroll_absolute += delta.y;
                }
                _ => {}
            }
        }

        ctx.set_clear_color(0.1, 0.2, 0.3, 1.0);
        ctx.clear();

        gltf_model.draw(window_size.into(), scroll_absolute)?;

        window.present();
    }
}
