use miniquad::*;

pub struct InitParams<'a> {
    pub model: &'a mari_formats::Model,
}

pub struct Default {
    index_cnt: usize,
    bindings: Bindings,
    pipeline: Pipeline,
}

impl<'init> crate::Renderer<'init> for Default {
    type InitParams = InitParams<'init>;

    fn new(ctx: &mut Box<dyn RenderingBackend>, params: InitParams) -> Self {
        let model = params.model;

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&model.vertices),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&model.mesh),
        );
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: include_str!("shaders/default-vert.glsl"),
                    fragment: include_str!("shaders/default-frag.glsl"),
                },
                ShaderMeta {
                    images: vec![],
                    uniforms: UniformBlockLayout {
                        uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
                    },
                },
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::new("in_pos", VertexFormat::Float3)],
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
