use miniquad::*;

pub struct InitParams<'a> {
    pub model: &'a mari_formats::Model,
    pub texture: &'a mari_formats::TextureRGBA8,
}
pub struct Textured {
    index_cnt: usize,
    bindings: Bindings,
    pipeline: Pipeline,
}

impl<'init> crate::Renderer<'init> for Textured {
    type InitParams = InitParams<'init>;

    fn new(ctx: &mut Box<dyn RenderingBackend>, params: InitParams) -> Self {
        let InitParams { model, texture } = params;

        let mut interleaved_buffer =
            Vec::<f32>::with_capacity(model.vertices.len() + model.uvs.len());
        for i in 0..model.vertices.len() / 3 {
            interleaved_buffer.extend_from_slice(&model.vertices[3 * i..3 * i + 3]);
            interleaved_buffer.extend_from_slice(&model.uvs[2 * i..2 * i + 2]);
        }
        let texture = ctx.new_texture_from_rgba8(texture.width, texture.height(), &texture.data);

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&interleaved_buffer),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&model.mesh),
        );
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: include_str!("shaders/textured-vert.glsl"),
                    fragment: include_str!("shaders/textured-frag.glsl"),
                },
                ShaderMeta {
                    images: vec!["tex".to_string()],
                    uniforms: UniformBlockLayout {
                        uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
                    },
                },
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams {
                cull_face: CullFace::Back,
                depth_test: Comparison::Less,
                depth_write: true,
                ..PipelineParams::default()
            },
        );

        Self {
            index_cnt: model.mesh.len(),
            bindings,
            pipeline,
        }
    }

    fn render(&self, ctx: &mut Box<dyn RenderingBackend>, mvp: &[f32; 16]) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(UniformsSource::table(mvp));
        ctx.draw(0, self.index_cnt as i32, 1);
    }
}
