use std::convert::TryInto;

use golem::*;
use nalgebra_glm as glm;

pub struct FpsCounter {
    prev_time: std::time::Instant,
    frame_count: usize,
    digit_render: DigitRender,
    fps: usize,
}

impl FpsCounter {
    pub fn new(ctx: &Context) -> FpsCounter {
        FpsCounter {
            prev_time: std::time::Instant::now(),
            frame_count: 0usize,
            digit_render: DigitRender::new(ctx),
            fps: 0,
        }
    }

    pub fn count(&mut self) {
        self.frame_count += 1;
        if self.prev_time.elapsed() >= std::time::Duration::new(1, 0) {
            self.prev_time = std::time::Instant::now();
            self.fps = self.frame_count;
            self.frame_count = 0;
        }
    }

    pub fn draw(&mut self, p_matrix: &glm::Mat4) -> Result<(), GolemError> {
        let ones = self.fps % 10;
        let tens = self.fps / 10;
        let ones_matrix = p_matrix
            * glm::translation(&glm::vec3(0.9 + 0.08, 0.9, 0.0))
            * glm::scaling(&glm::vec3(0.1 * 3.0 / 5.0, 0.1, 1.0));
        let tens_matrix = p_matrix
            * glm::translation(&glm::vec3(0.9, 0.9, 0.0))
            * glm::scaling(&glm::vec3(0.1 * 3.0 / 5.0, 0.1, 1.0));
        self.digit_render.draw(ones, &ones_matrix)?;
        self.digit_render.draw(tens, &tens_matrix)?;
        Ok(())
    }
}

pub struct DigitRender {
    vb: VertexBuffer,
    eb: ElementBuffer,
    shader: ShaderProgram,
}

impl DigitRender {
    pub fn new(ctx: &Context) -> DigitRender {
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
                ],
                fragment_input: &[
                    Attribute::new("vert_uv", AttributeType::Vector(Dimension::D2)),
                ],
                uniforms: &[
                    Uniform::new("matrix", UniformType::Matrix(Dimension::D4)),
                    Uniform::new("num", UniformType::Scalar(NumberType::Int)),
                ],
                vertex_shader: r#" void main() {
                    gl_Position = matrix * vec4(vert_position, -1.0, 1.0);
                    vert_uv = vert_position;
                }"#,
                fragment_shader: r#"
                const float[150] array = float[](
                    1., 1., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1., 1., 0., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
                    1., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 1., 0., 1., 1., 0., 0., 1., 0., 0., 1., 0., 1., 1., 0., 1., 1., 0., 1.,
                    1., 0., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1.,
                    1., 0., 1., 0., 0., 1., 1., 0., 0., 0., 0., 1., 0., 0., 1., 0., 0., 1., 1., 0., 1., 0., 0., 1., 1., 0., 1., 0., 0., 1.,
                    1., 1., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1.
                );
                void main() {
                    vec2 a = vert_uv * vec2(3., 5.);
                    a.x += float(num * 3);
                    a.y = 5. - a.y;
                    float color = array[int(a.x) + int(a.y) * 30];
                    if (color == 0.) discard;
                    gl_FragColor = vec4(1.);
                }"#,
            },
        )
        .unwrap();

        DigitRender { vb, eb, shader }
    }

    // size, position, p_matrix
    pub fn draw(&mut self, num: usize, matrix: &glm::Mat4) -> Result<(), GolemError> {
        self.shader.bind();

        self.shader.set_uniform(
            "matrix",
            UniformValue::Matrix4(glm::value_ptr(matrix).try_into().unwrap()),
        )?;
        self.shader
            .set_uniform("num", UniformValue::Int(num as i32))?;

        unsafe {
            self.shader
                .draw(&self.vb, &self.eb, 0..4, GeometryMode::TriangleStrip)?;
        }

        Ok(())
    }
}
