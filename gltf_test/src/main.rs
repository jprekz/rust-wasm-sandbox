use wrapper_test::{event::*, golem::*, log, App, Builder};

pub fn main() {
    Builder::init(init)
        .event(event)
        .update(update)
        .view(view)
        .run();
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

struct Model {
    shader: ShaderProgram,
    primitives: Vec<Primitive>,
    window_size: PhysicalSize<u32>,
    scroll_absolute: f32,
}

fn init(app: &App) -> Model {
    let ctx = &app.draw;

    let (document, buffers, images) = {
        let data = include_bytes!("test.glb");
        gltf::import_slice(data).unwrap()
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
    )
    .unwrap();

    shader.bind();

    Model {
        shader,
        primitives,
        window_size: app.window_size(),
        scroll_absolute: 1.0,
    }
}

fn event(app: &App, model: &mut Model, event: &Event) {
    match event {
        Event::MouseWheel {
            delta: MouseScrollDelta::LineDelta(_h, v),
            ..
        } => {
            model.scroll_absolute += v;
            log(format!("{}", model.scroll_absolute));
        }
        Event::Resized(physical_size) => {
            model.window_size = *physical_size;
            app.draw
                .set_viewport(0, 0, physical_size.width, physical_size.height);
            log(format!("{}", physical_size.height));
        }
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model) {}

fn view(app: &App, model: &Model) {
    let ctx = &app.draw;
    let shader = &model.shader;

    ctx.set_clear_color(0.1, 0.2, 0.3, 1.0);
    ctx.clear();

    shader
        .set_uniform(
            "resolution",
            UniformValue::Vector2([
                model.window_size.width as f32,
                model.window_size.height as f32,
            ]),
        )
        .unwrap();
    let zoom_factor = 1.1_f32.powf(model.scroll_absolute);
    shader
        .set_uniform("zoom_factor", UniformValue::Float(zoom_factor))
        .unwrap();

    for primitive in &model.primitives {
        unsafe {
            shader
                .draw(
                    &primitive.vb,
                    &primitive.eb,
                    0..primitive.indices_len,
                    primitive.mode,
                )
                .unwrap();
        }
    }
}
