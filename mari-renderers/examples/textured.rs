use mari_renderers::Renderer;

use glam::*;
use miniquad::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <obj_file> <tex_file>", args[0]);
        std::process::exit(1);
    }

    let obj_file = File::open(&args[1])?;
    let obj_reader = BufReader::new(obj_file);
    let tex_file = File::open(&args[2])?;
    let tex_reader = BufReader::new(tex_file);

    let scene = mari_formats::Scene::new_with_model_and_texture(
        mari_formats::Model::new_from_obj(obj_reader)?,
        mari_formats::TextureRGBA8::new_from_png(tex_reader)?,
    );

    miniquad::start(conf::Conf::default(), move || Box::new(Stage::new(scene)));

    Ok(())
}
struct Stage {
    cam_pos: Vec3,
    cam_dir: Vec3,
    renderer: mari_renderers::Textured,
    ctx: Box<dyn RenderingBackend>,
}

impl Stage {
    pub fn new(scene: mari_formats::Scene) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let renderer = mari_renderers::Textured::new(
            &mut ctx,
            mari_renderers::TexturedInitParams {
                model: &scene.actors.values().nth(0).unwrap().body,
                texture: scene.textures.values().nth(0).unwrap(),
            },
        );

        Stage {
            cam_pos: Vec3::Z,
            cam_dir: -Vec3::Z,
            renderer,
            ctx,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let true_right = self.cam_dir.cross(Vec3::Y);
        match keycode {
            KeyCode::Up => {
                self.cam_dir = Mat3::from_axis_angle(true_right, 0.05) * self.cam_dir;
                return;
            }
            KeyCode::Down => {
                self.cam_dir = Mat3::from_axis_angle(true_right, -0.05) * self.cam_dir;
                return;
            }
            KeyCode::Left => {
                self.cam_dir = Mat3::from_axis_angle(Vec3::Y, 0.05) * self.cam_dir;
                return;
            }
            KeyCode::Right => {
                self.cam_dir = Mat3::from_axis_angle(Vec3::Y, -0.05) * self.cam_dir;
                return;
            }
            _ => {}
        };

        let true_front = Vec3::Y.cross(true_right);
        self.cam_pos += 0.05
            * match keycode {
                KeyCode::W => true_front,
                KeyCode::S => -true_front,
                KeyCode::A => -true_right,
                KeyCode::D => true_right,
                _ => Vec3::ZERO,
            };
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.0, 0.0, 0.0, 1.0)),
            depth: Some(1.0),
            stencil: None,
        });

        let m = Mat4::from_translation(glam::Vec3 {
            x: 0.0,
            y: -1.2,
            z: 0.0,
        });
        let v = Mat4::look_to_rh(self.cam_pos, self.cam_dir, Vec3::Y);
        let p = Mat4::perspective_rh_gl(
            60.0f32.to_radians(),
            window::screen_size().0 / window::screen_size().1,
            0.01,
            5.0,
        );

        self.renderer
            .render(&mut self.ctx, &(p * v * m).to_cols_array());

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}
