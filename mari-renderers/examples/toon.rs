use mari_renderers::Renderer;

use glam::*;
use miniquad::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "Usage: {} <obj_file> <tex_file> <rmp_tex_file> <sdw_tex_file>",
            args[0]
        );
        std::process::exit(1);
    }

    let obj_file = File::open(&args[1])?;
    let obj_reader = BufReader::new(obj_file);
    let tex_file = File::open(&args[2])?;
    let tex_reader = BufReader::new(tex_file);
    let rmp_tex_file = File::open(&args[3])?;
    let rmp_tex_reader = BufReader::new(rmp_tex_file);
    let rmp_tex = mari_formats::TextureRGBA8::new_from_png(rmp_tex_reader)?;
    let sdw_tex_file = File::open(&args[4])?;
    let sdw_tex_reader = BufReader::new(sdw_tex_file);
    let sdw_tex = mari_formats::TextureRGBA8::new_from_png(sdw_tex_reader)?;

    let scene = mari_formats::Scene::new_with_model_and_texture(
        mari_formats::Model::new_from_obj(obj_reader)?,
        mari_formats::TextureRGBA8::new_from_png(tex_reader)?,
    );

    miniquad::start(conf::Conf::default(), move || {
        Box::new(Stage::new(scene, &rmp_tex, &sdw_tex))
    });

    Ok(())
}
struct Stage {
    cam_pos: Vec3,
    cam_dir: Vec3,
    m: Mat4,
    vp: Mat4,
    renderer: mari_renderers::Toon,
    ctx: Box<dyn RenderingBackend>,
}

impl Stage {
    pub fn new(
        scene: mari_formats::Scene,
        ramp_texture: &mari_formats::TextureRGBA8,
        sdw_texture: &mari_formats::TextureRGBA8,
    ) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let renderer = mari_renderers::Toon::new(
            &mut ctx,
            mari_renderers::ToonInitParams {
                model: &scene.actors.values().nth(0).unwrap().body,
                texture: scene.textures.values().nth(0).unwrap(),
                ramp_texture,
                sdw_texture,
            },
        );

        let mut new_self = Stage {
            cam_pos: Vec3::Z,
            cam_dir: -Vec3::Z,
            m: Mat4::from_translation(glam::Vec3 {
                x: 0.0,
                y: -1.2,
                z: 0.0,
            }),
            vp: Mat4::IDENTITY,
            renderer,
            ctx,
        };

        new_self.update_vp();
        new_self
    }

    fn update_vp(&mut self) {
        self.vp = Mat4::perspective_rh_gl(
            60.0f32.to_radians(),
            window::screen_size().0 / window::screen_size().1,
            0.01,
            5.0,
        ) * Mat4::look_to_rh(self.cam_pos, self.cam_dir, Vec3::Y);
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let true_right = self.cam_dir.cross(Vec3::Y);
        if match keycode {
            KeyCode::Up => {
                self.cam_dir = Mat3::from_axis_angle(true_right, 0.05) * self.cam_dir;
                true
            }
            KeyCode::Down => {
                self.cam_dir = Mat3::from_axis_angle(true_right, -0.05) * self.cam_dir;
                true
            }
            KeyCode::Left => {
                self.cam_dir = Mat3::from_axis_angle(Vec3::Y, 0.05) * self.cam_dir;
                true
            }
            KeyCode::Right => {
                self.cam_dir = Mat3::from_axis_angle(Vec3::Y, -0.05) * self.cam_dir;
                true
            }
            _ => false,
        } {
            self.update_vp();
            return;
        }

        let true_front = Vec3::Y.cross(true_right);
        self.cam_pos += 0.05
            * match keycode {
                KeyCode::W => true_front,
                KeyCode::S => -true_front,
                KeyCode::A => -true_right,
                KeyCode::D => true_right,
                _ => Vec3::ZERO,
            };
        self.update_vp();
        let cam_pos_in_model_space = (self.m.inverse() * self.cam_pos.extend(1.0)).xyz();
        self.renderer
            .set_light_pos(&cam_pos_in_model_space.to_array());
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.0, 0.0, 0.0, 1.0)),
            depth: Some(1.0),
            stencil: None,
        });

        self.renderer
            .render(&mut self.ctx, &(self.vp * self.m).to_cols_array());

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}
