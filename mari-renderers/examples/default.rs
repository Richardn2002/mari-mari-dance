use mari_renderers::Renderer;

use miniquad::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <obj_file>", args[0]);
        std::process::exit(1);
    }

    let obj_file = File::open(&args[1])?;
    let obj_reader = BufReader::new(obj_file);

    let scene = mari_formats::Scene::new_with_model(mari_formats::Model::new_from_obj(obj_reader)?);

    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new(scene)));

    Ok(())
}
struct Stage {
    renderer: mari_renderers::Default,
    ctx: Box<dyn RenderingBackend>,
}

impl Stage {
    pub fn new(scene: mari_formats::Scene) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let renderer = mari_renderers::Default::new(
            &mut ctx,
            mari_renderers::DefaultInitParams {
                model: &scene.actors.values().nth(0).unwrap().body,
            },
        );

        Stage { renderer, ctx }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::Clear {
            color: None,
            depth: Some(1.0),
            stencil: None,
        });

        self.renderer.render(
            &mut self.ctx,
            &(glam::Mat4::from_translation(glam::Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            }) * glam::Mat4::from_scale(glam::Vec3::from_array([1.0, 1.0, -1.0])))
            .to_cols_array(),
        );

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}
