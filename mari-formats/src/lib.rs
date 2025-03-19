mod obj;
pub use obj::Error as ObjError;

use std::collections::HashMap;
use std::io::{BufReader, Read};

pub struct Model {
    /// compact storage of vertex x,y,z
    pub vertices: Vec<f32>,
    /// compact storage of mesh triangle indices, CCW as front-facing
    pub mesh: Vec<u16>,
    /// compact storage of texture u,v
    pub uvs: Vec<f32>,
    /// compact storage of vertex normals, normalized
    pub normals: Vec<f32>,
}

pub struct Actor {
    pub body: Model,
}

pub struct Scene {
    pub actors: HashMap<String, Actor>,
    pub textures: HashMap<String, TextureRGBA8>,
}

pub struct TextureRGBA8 {
    pub width: u16,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum ModelError {
    Obj(ObjError),
    InvalidNormal(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl std::error::Error for ModelError {}

impl Model {
    pub fn new_from_obj<R: Read>(buf: BufReader<R>) -> Result<Self, ModelError> {
        let mut obj = obj::Obj::new(buf).map_err(ModelError::Obj)?;

        for i in 0..obj.uvs.len() / 2 {
            // flip V to convert from OBJ space to OpenGL space
            obj.uvs[2 * i + 1] = 1.0 - obj.uvs[2 * i + 1]
        }

        for i in 0..obj.normals.len() / 3 {
            let (x, y, z) = (
                obj.normals[3 * i],
                obj.normals[3 * i + 1],
                obj.normals[3 * i + 2],
            );
            let l = (x * x + y * y + z * z).sqrt();
            if l == 0.0 {
                return Err(ModelError::InvalidNormal(format!(
                    "Normal #{} has length 0.",
                    i + 1
                )));
            }
            obj.normals[3 * i] = x / l;
            obj.normals[3 * i + 1] = y / l;
            obj.normals[3 * i + 2] = z / l;
        }

        let obj::Obj {
            vertices,
            mesh,
            uvs: texture,
            normals,
        } = obj;
        Ok(Self {
            vertices,
            mesh,
            uvs: texture,
            normals,
        })
    }

    pub fn repr(&self) -> String {
        format!(
            "A model with {} vertices, {} texture bindings, {} faces and {} vertex normals.",
            self.vertices.len() / 3,
            self.uvs.len() / 2,
            self.mesh.len() / 3,
            self.normals.len() / 3
        )
    }
}

impl Scene {
    pub fn new_with_model(model: Model) -> Self {
        Self {
            actors: HashMap::from([("Temari".to_string(), Actor { body: model })]),
            textures: HashMap::new(),
        }
    }

    pub fn new_with_model_and_texture(model: Model, texture: TextureRGBA8) -> Self {
        Self {
            actors: HashMap::from([("Temari".to_string(), Actor { body: model })]),
            textures: HashMap::from([("Temari".to_string(), texture)]),
        }
    }
}

#[derive(Debug)]
pub enum TextureError {
    Png(png::DecodingError),
    WidthTooLarge,
    HeightTooLarge,
}

impl std::fmt::Display for TextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl std::error::Error for TextureError {}

impl TextureRGBA8 {
    pub fn new_from_png<R: Read>(buf: BufReader<R>) -> Result<Self, TextureError> {
        let mut reader = png::Decoder::new(buf)
            .read_info()
            .map_err(TextureError::Png)?;
        let mut data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut data).map_err(TextureError::Png)?;

        let width: u16 = info
            .width
            .try_into()
            .map_err(|_| TextureError::WidthTooLarge)?;
        let _: u16 = info
            .width
            .try_into()
            .map_err(|_| TextureError::HeightTooLarge)?;
        data.resize(info.buffer_size(), 0);

        Ok(Self { width, data })
    }

    pub fn height(&self) -> u16 {
        (self.data.len() / 4 / self.width as usize) as u16
    }
}
