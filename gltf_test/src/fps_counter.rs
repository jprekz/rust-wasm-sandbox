use golem::*;

pub struct FpsCounter {
    prev_time: std::time::Instant,
    frame_count: usize,
}

impl FpsCounter {
    fn new(ctx: &Context) -> FpsCounter {
        FpsCounter {
            prev_time: std::time::Instant::now(),
            frame_count: 0usize,
        }
    }

    fn count(&mut self) {
        self.frame_count += 1;
        if self.prev_time.elapsed() >= std::time::Duration::new(1, 0) {
            self.prev_time = std::time::Instant::now();
            println!("{}", self.frame_count);
            self.frame_count = 0;
        }
    }

    fn draw(&mut self) {}
}

pub struct DigitRender {
    vb: VertexBuffer,
    eb: ElementBuffer,
    shader: ShaderProgram,
}

impl DigitRender {
    fn new(ctx: &Context) -> DigitRender {
        let vertices = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];
        let indices = [0, 1, 2, 3];
        let mut vb = VertexBuffer::new(ctx).unwrap();
        let mut eb = ElementBuffer::new(ctx).unwrap();
        vb.set_data(&vertices);
        eb.set_data(&indices);
        let shader = ShaderProgram::new(
            ctx,
            ShaderDescription {
                vertex_input: &[
                    Attribute::new("vert_position", AttributeType::Vector(Dimension::D2)),
                    Attribute::new("vert_uv", AttributeType::Vector(Dimension::D2)),
                ],
                fragment_input: &[Attribute::new(
                    "f_vert_uv",
                    AttributeType::Vector(Dimension::D2),
                )],
                uniforms: &[Uniform::new("num", UniformType::Scalar(NumberType::Int))],
                vertex_shader: r#" void main() {
                    gl_Position = vec4(vert_position, 0.0, 1.0);
                    f_vert_uv = vert_uv;
                }"#,
                fragment_shader: r#" void main() {
                    gl_FragColor = vec3(f_vert_uv, 0.0);
                }"#,
            },
        )
        .unwrap();

        DigitRender { vb, eb, shader }
    }

    fn draw(&mut self, num: usize) -> Result<(), GolemError> {
        self.shader.bind();
        self.shader
            .set_uniform("num", UniformValue::Int(num as i32))?;

        unsafe {
            self.shader
                .draw(&self.vb, &self.eb, 0..4, GeometryMode::TriangleStrip)?;
        }

        Ok(())
    }
}
