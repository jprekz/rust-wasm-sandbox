use blinds::*;
use golem::{
    Attribute, AttributeType, Context,
    Dimension::{D2, D4},
    ElementBuffer, GeometryMode, GolemError, ShaderDescription, ShaderProgram, VertexBuffer,
};

pub fn main() {
    run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}

async fn app(
    window: Window,
    ctx: golem::glow::Context,
    mut events: EventStream,
) -> Result<(), GolemError> {
    let ctx = &Context::from_glow(ctx)?;

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
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_color", AttributeType::Vector(D4)),
            ],
            fragment_input: &[Attribute::new("frag_color", AttributeType::Vector(D4))],
            uniforms: &[],
            vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
            fragment_shader: r#" void main() {
            gl_FragColor = frag_color;
        }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();
    ctx.set_clear_color(0.1, 0.2, 0.3, 1.0);
    ctx.clear();

    unsafe {
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
    }

    window.present();

    loop {
        events.next_event().await;
    }
}
