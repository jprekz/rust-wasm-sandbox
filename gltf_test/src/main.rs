use blinds::*;
use golem::*;
use nalgebra_glm as glm;

fn context_from_blinds(window: &Window) -> Result<Context, GolemError> {
    #[cfg(not(target_arch = "wasm32"))]
    let glow_ctx = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
    #[cfg(target_arch = "wasm32")]
    let glow_ctx = glow::Context::from_webgl1_context(window.webgl_context());

    Context::from_glow(glow_ctx)
}

pub fn main() {
    let settings = Settings {
        resizable: true,
        ..Settings::default()
    };
    run(settings, |window, events| async move {
        app(window, events).await.unwrap()
    });
}

async fn app(window: Window, mut events: EventStream) -> Result<(), GolemError> {
    let ctx = &context_from_blinds(&window)?;

    let (document, buffers, images) = {
        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::Uint8Array;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{Request, RequestInit, Response};

            let opts = RequestInit::new();
            let request = Request::new_with_str_and_init("test.glb", &opts).unwrap();
            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap();
            let resp: Response = resp_value.dyn_into().unwrap();
            let arrbuff_value = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
            let typebuff = Uint8Array::new(&arrbuff_value).to_vec();
            gltf::import_slice(&typebuff).unwrap()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            gltf::import("test.glb").unwrap()
        }
    };
    println!("#buffers: {}", buffers.len());
    println!("#images: {}", images.len());

    let mut primitives = Vec::new();
    for node in document.nodes() {
        if let Some(mesh) = node.mesh() {
            println!("mesh: {:?}", mesh.name());
            for primitive in mesh.primitives() {
                println!("  primitive:");
                let _material = primitive.material();
                primitives.push(Primitive::from_gltf_primitive(
                    ctx,
                    &primitive,
                    &buffers[0].0,
                ));
            }
        }
    }

    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[Attribute::new(
                "vert_position",
                AttributeType::Vector(Dimension::D3),
            )],
            fragment_input: &[Attribute::new(
                "frag_color",
                AttributeType::Vector(Dimension::D4),
            )],
            uniforms: &[
                Uniform::new(
                    "resolution",
                    UniformType::Vector(NumberType::Float, Dimension::D2),
                ),
                Uniform::new("zoom_factor", UniformType::Scalar(NumberType::Float)),
            ],
            vertex_shader: r#" void main() {
                vec2 projection = resolution / min(resolution.x, resolution.y);
                gl_Position = vec4(vert_position.xy * zoom_factor / projection, 0, 1);
                frag_color = vec4(0.5, 0.5, 0.5, 1);
            }"#,
            fragment_shader: r#" void main() {
                gl_FragColor = frag_color;
            }"#,
        },
    )?;

    shader.bind();

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

        shader.set_uniform("resolution", UniformValue::Vector2(window_size.into()))?;
        let zoom_factor = 1.1_f32.powf(scroll_absolute);
        shader.set_uniform("zoom_factor", UniformValue::Float(zoom_factor))?;

        for primitive in &primitives {
            unsafe {
                shader.draw(
                    &primitive.vb,
                    &primitive.eb,
                    0..primitive.indices_len,
                    primitive.mode,
                )?;
            }
        }

        window.present();
    }
}

struct Primitive {
    vb: VertexBuffer,
    eb: ElementBuffer,
    indices_len: usize,
    mode: GeometryMode,
}
impl Primitive {
    fn from_gltf_primitive(ctx: &Context, primitive: &gltf::Primitive, buffer: &[u8]) -> Primitive {
        let reader = primitive.reader(|_| Some(buffer));

        let mut vertices = Vec::new();
        if let Some(pos) = reader.read_positions() {
            for p in pos.into_iter() {
                vertices.extend_from_slice(&p);
            }
        }
        let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
        let mut vb = VertexBuffer::new(ctx).unwrap();
        let mut eb = ElementBuffer::new(ctx).unwrap();
        vb.set_data(&vertices);
        eb.set_data(&indices);

        let mode = match primitive.mode() {
            gltf::mesh::Mode::Points => GeometryMode::Points,
            gltf::mesh::Mode::Lines => GeometryMode::Lines,
            gltf::mesh::Mode::LineLoop => GeometryMode::LineLoop,
            gltf::mesh::Mode::LineStrip => GeometryMode::LineStrip,
            gltf::mesh::Mode::Triangles => GeometryMode::Triangles,
            gltf::mesh::Mode::TriangleStrip => GeometryMode::TriangleStrip,
            gltf::mesh::Mode::TriangleFan => GeometryMode::TriangleFan,
        };

        Primitive {
            vb,
            eb,
            indices_len: indices.len(),
            mode,
        }
    }
}
