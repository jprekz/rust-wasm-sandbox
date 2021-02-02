use std::convert::TryInto;

use golem::*;
use nalgebra_glm as glm;

pub struct Gltf {
    shader: ShaderProgram,
    primitives: Vec<Primitive>,
}

impl Gltf {
    pub fn load(path: impl AsRef<std::path::Path>, ctx: &Context) -> Result<Gltf, gltf::Error> {
        let (document, buffers, images) = gltf::import(path)?;
        Self::load_impl(document, buffers, images, ctx)
    }

    pub fn load_slice(slice: impl AsRef<[u8]>, ctx: &Context) -> Result<Gltf, gltf::Error> {
        let (document, buffers, images) = gltf::import_slice(slice)?;
        Self::load_impl(document, buffers, images, ctx)
    }

    fn load_impl(
        document: gltf::Document,
        buffers: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
        ctx: &Context,
    ) -> Result<Gltf, gltf::Error> {
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

        let shader = ShaderProgram::new(
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
                uniforms: &[Uniform::new(
                    "mvp_matrix",
                    UniformType::Matrix(Dimension::D4),
                )],
                vertex_shader: r#" void main() {
                    gl_Position = mvp_matrix * vec4(vert_position, 1.0);
                    frag_color = vec4(0.5, 0.5, 0.5, 1);
                }"#,
                fragment_shader: r#" void main() {
                    gl_FragColor = frag_color;
                }"#,
            },
        )
        .unwrap();

        Ok(Gltf { shader, primitives })
    }

    pub fn draw(&mut self, mvp_matrix: &glm::Mat4) -> Result<(), GolemError> {
        self.shader.bind();

        self.shader.set_uniform(
            "mvp_matrix",
            UniformValue::Matrix4(glm::value_ptr(mvp_matrix).try_into().unwrap()),
        )?;

        for primitive in &self.primitives {
            unsafe {
                primitive.draw(&self.shader)?;
            }
        }
        Ok(())
    }
}

pub struct Primitive {
    vb: VertexBuffer,
    eb: ElementBuffer,
    indices_len: usize,
    mode: GeometryMode,
}
impl Primitive {
    pub fn from_gltf_primitive(
        ctx: &Context,
        primitive: &gltf::Primitive,
        buffer: &[u8],
    ) -> Primitive {
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

    pub unsafe fn draw(&self, shader: &ShaderProgram) -> Result<(), GolemError> {
        shader.draw(&self.vb, &self.eb, 0..self.indices_len, self.mode)
    }
}
