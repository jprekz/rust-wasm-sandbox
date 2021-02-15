use std::{convert::TryInto, num::NonZeroU32};

use crate::time::*;
use golem::*;
use nalgebra_glm as glm;

pub struct FpsCounter {
    prev_time: Instant,
    frame_count: usize,
    digit_render: DigitRender,
    fps: usize,
}

impl FpsCounter {
    pub fn new(ctx: &Context) -> FpsCounter {
        FpsCounter {
            prev_time: Instant::now(),
            frame_count: 0usize,
            digit_render: DigitRender::new(ctx),
            fps: 0,
        }
    }

    pub fn count(&mut self) {
        self.frame_count += 1;
        if self.prev_time.elapsed() >= Duration::new(1, 0) {
            self.prev_time = Instant::now();
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
    texture: Texture,
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
        #[rustfmt::skip]
        let bitmap = vec![
            1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1,
            1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1,
            1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
            1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1,
        ];
        let data: Vec<_> = bitmap
            .into_iter()
            .flat_map(|b| {
                if b == 1 {
                    &[255, 255, 255, 255]
                } else {
                    &[0, 0, 0, 0]
                }
            })
            .copied()
            .collect();
        let mut texture = Texture::new(ctx).unwrap();
        texture.set_image(Some(&data), 30, 5, ColorFormat::RGBA);
        texture.set_magnification(TextureFilter::Nearest).unwrap();

        let shader = ShaderProgram::new(
            ctx,
            ShaderDescription {
                vertex_input: &[Attribute::new(
                    "vert_position",
                    AttributeType::Vector(Dimension::D2),
                )],
                fragment_input: &[Attribute::new(
                    "frag_uv",
                    AttributeType::Vector(Dimension::D2),
                )],
                uniforms: &[
                    Uniform::new("matrix", UniformType::Matrix(Dimension::D4)),
                    Uniform::new("num", UniformType::Scalar(NumberType::Float)),
                    Uniform::new("tex", UniformType::Sampler2D),
                ],
                vertex_shader: r#" void main() {
                    gl_Position = matrix * vec4(vert_position, -1.0, 1.0);
                    frag_uv = vert_position;
                }"#,
                fragment_shader: r#" void main() {
                    vec2 a = frag_uv;
                    a.x /= 10.;
                    a.x += num / 10.;
                    a.y = 1. - a.y;
                    vec4 color = texture(tex, a);
                    if (color.a == 0.) discard;
                    gl_FragColor = color;
                }"#,
            },
        )
        .unwrap();

        DigitRender {
            vb,
            eb,
            texture,
            shader,
        }
    }

    pub fn draw(&mut self, num: usize, matrix: &glm::Mat4) -> Result<(), GolemError> {
        self.shader.bind();
        self.texture
            .set_active(unsafe { NonZeroU32::new_unchecked(1) });

        self.shader.set_uniform(
            "matrix",
            UniformValue::Matrix4(glm::value_ptr(matrix).try_into().unwrap()),
        )?;
        self.shader
            .set_uniform("num", UniformValue::Float(num as f32))?;
        self.shader.set_uniform("tex", UniformValue::Int(1))?;

        unsafe {
            self.shader
                .draw(&self.vb, &self.eb, 0..4, GeometryMode::TriangleStrip)?;
        }

        Ok(())
    }
}
