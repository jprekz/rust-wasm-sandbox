use wrapper_test::{golem::*, log, App, Builder};

pub fn main() {
    Builder::init(init).update(update).view(view).run();
}

struct Model {
    shader: ShaderProgram,
    vb: VertexBuffer,
    eb: ElementBuffer,
    indices: Vec<u32>,
}

fn init(app: &App) -> Model {
    let ctx = &app.draw;

    #[rustfmt::skip]
    let vertices = [
        0.0, 0.5,           0.5, 1.0, 0.5, 1.0,
        -0.5, -0.5,         0.0, 0.0, 0.5, 1.0,
        0.5, -0.5,          1.0, 0.0, 0.5, 1.0,
    ];
    let indices = [0, 1, 2];
    let mut shader = ShaderProgram::new(
        ctx,
        ShaderDescription {
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(Dimension::D2)),
                Attribute::new("vert_color", AttributeType::Vector(Dimension::D4)),
            ],
            fragment_input: &[Attribute::new(
                "frag_color",
                AttributeType::Vector(Dimension::D4),
            )],
            uniforms: &[],
            vertex_shader: r#" void main() {
                gl_Position = vec4(vert_position, 0, 1);
                frag_color = vert_color;
            }"#,
            fragment_shader: r#" void main() {
                gl_FragColor = frag_color;
            }"#,
        },
    )
    .unwrap();

    let mut vb = VertexBuffer::new(ctx).unwrap();
    let mut eb = ElementBuffer::new(ctx).unwrap();
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();

    Model {
        shader,
        vb,
        eb,
        indices: indices.to_vec(),
    }
}

fn update(_app: &App, _model: &mut Model) {
    log("update");
}

fn view(app: &App, model: &Model) {
    log("view");

    let ctx = &app.draw;

    ctx.set_clear_color(0.1, 0.2, 0.3, 1.0);
    ctx.clear();
    unsafe {
        model
            .shader
            .draw(
                &model.vb,
                &model.eb,
                0..model.indices.len(),
                GeometryMode::Triangles,
            )
            .unwrap();
    }
}
