mod renderers;

use miniquad::*;

pub use renderers::Default;
pub use renderers::DefaultInitParams;

pub use renderers::Textured;
pub use renderers::TexturedInitParams;

pub trait Renderer<'init> {
    type InitParams;

    fn new(ctx: &mut Box<dyn RenderingBackend>, params: Self::InitParams) -> Self;
    /// `mvp` is a mat4 in column-major order
    ///
    /// depth test is LESS, so after `mvp`, nearer vertices shall have smaller z values.
    fn render(&self, ctx: &mut Box<dyn RenderingBackend>, mvp: &[f32; 16]);
}
