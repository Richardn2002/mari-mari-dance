use miniquad::*;

pub struct InitParams<'a> {
    pub model: &'a mari_formats::Model,
    pub texture: &'a mari_formats::TextureRGBA8,
    pub ramp_texture: &'a mari_formats::TextureRGBA8,
    pub sdw_texture: &'a mari_formats::TextureRGBA8,
}
pub struct Toon {
    index_cnt: usize,
    bindings: Bindings,
    pipeline: Pipeline,

    light_pos_in_model_space: [f32; 3],
}

impl<'init> crate::Renderer<'init> for Toon {
    type InitParams = InitParams<'init>;

    fn new(ctx: &mut Box<dyn RenderingBackend>, params: InitParams) -> Self {
        let InitParams {
            model,
            texture,
            ramp_texture,
            sdw_texture,
        } = params;

        let mut interleaved_buffer =
            Vec::<f32>::with_capacity(model.vertices.len() + model.uvs.len() + model.normals.len());
        for i in 0..model.vertices.len() / 3 {
            interleaved_buffer.extend_from_slice(&model.vertices[3 * i..3 * i + 3]);
            interleaved_buffer.extend_from_slice(&model.uvs[2 * i..2 * i + 2]);
            interleaved_buffer.extend_from_slice(&model.normals[3 * i..3 * i + 3]);
        }
        let texture = ctx.new_texture_from_rgba8(texture.width, texture.height(), &texture.data);
        let ramp_texture = ctx.new_texture_from_rgba8(
            ramp_texture.width,
            ramp_texture.height(),
            &ramp_texture.data,
        );
        let sdw_texture =
            ctx.new_texture_from_rgba8(sdw_texture.width, sdw_texture.height(), &sdw_texture.data);

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
            images: vec![texture, ramp_texture, sdw_texture],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: include_str!("shaders/toon-vert.glsl"),
                    fragment: include_str!("shaders/toon-frag.glsl"),
                },
                ShaderMeta {
                    images: vec![
                        "tex".to_string(),
                        "rmp_tex".to_string(),
                        "sdw_tex".to_string(),
                    ],
                    uniforms: UniformBlockLayout {
                        uniforms: vec![
                            UniformDesc::new("mvp", UniformType::Mat4),
                            UniformDesc::new("lightPosModelSpace", UniformType::Float3),
                        ],
                    },
                },
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
                VertexAttribute::new("in_norm", VertexFormat::Float3),
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

            light_pos_in_model_space: [0.0, 0.0, 1.0],
        }
    }

    fn render(&self, ctx: &mut Box<dyn RenderingBackend>, mvp: &[f32; 16]) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let mut uniform = [0.0; 19];
        uniform[..16].copy_from_slice(mvp);
        uniform[16..].copy_from_slice(&self.light_pos_in_model_space);
        ctx.apply_uniforms(UniformsSource::table(&uniform));

        ctx.draw(0, self.index_cnt as i32, 1);
    }
}

impl Toon {
    /// set light pos in the MODEL space
    pub fn set_light_pos(&mut self, p: &[f32; 3]) {
        self.light_pos_in_model_space = *p;
    }
}
