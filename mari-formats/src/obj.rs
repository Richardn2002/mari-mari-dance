use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Invalid(String),
    TooManyVertices,
}

pub struct Obj {
    /// compact storage of vertex x,y,z
    pub vertices: Vec<f32>,
    /// compact storage of mesh triangle indices
    pub mesh: Vec<u16>,
    /// compact storage of texture u,v
    pub uvs: Vec<f32>,
    /// compact storage of vertex normals, not normalized
    pub normals: Vec<f32>,
}

impl Obj {
    pub fn new<R: Read>(buf: BufReader<R>) -> Result<Self, Error> {
        let mut vertices = Vec::<f32>::new();
        let mut texture = Vec::<f32>::new();
        let mut mesh = Vec::<u16>::new();
        let mut normals = Vec::<f32>::new();

        for (i, line) in buf.lines().enumerate() {
            let line = line.map_err(Error::Io)?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    if parts.len() < 4 {
                        return Err(Error::Invalid(format!(
                            "{} @ line {}: Invalid vertex.",
                            line,
                            i + 1
                        )));
                    }
                    for idx in [1, 2, 3] {
                        let x = parts[idx].parse::<f32>().map_err(|e| {
                            Error::Invalid(format!("{} @ line {}: {}.", line, i + 1, e))
                        })?;

                        vertices.push(x);
                    }
                    if vertices.len() > (u16::MAX as usize + 1) * 3 {
                        return Err(Error::TooManyVertices);
                    }
                }

                "vt" => {
                    if parts.len() < 3 {
                        return Err(Error::Invalid(format!(
                            "{} @ line {}: Invalid texture binding.",
                            line,
                            i + 1
                        )));
                    }
                    for idx in [1, 2] {
                        let x = parts[idx].parse::<f32>().map_err(|e| {
                            Error::Invalid(format!("{} @ line {}: {}.", line, i + 1, e))
                        })?;

                        texture.push(x);
                    }
                }

                "vn" => {
                    if parts.len() < 4 {
                        return Err(Error::Invalid(format!(
                            "{} @ line {}: Invalid vertex normal.",
                            line,
                            i + 1
                        )));
                    }
                    for idx in [1, 2, 3] {
                        let x = parts[idx].parse::<f32>().map_err(|e| {
                            Error::Invalid(format!("{} @ line {}: {}.", line, i + 1, e))
                        })?;

                        normals.push(x);
                    }
                }

                "f" => {
                    if parts.len() < 4 {
                        return Err(Error::Invalid(format!(
                            "{} @ line {}: Invalid face.",
                            line,
                            i + 1
                        )));
                    }
                    for idx in [1, 2, 3] {
                        let index = parts[idx]
                            .split('/')
                            .collect::<Vec<_>>()
                            .first()
                            .ok_or(Error::Invalid(format!(
                                "{} @ line {}: Invalid face.",
                                line,
                                i + 1
                            )))?
                            .parse::<u16>()
                            .map_err(|e| {
                                Error::Invalid(format!("{} @ line {}: {}.", line, i + 1, e))
                            })?
                            .checked_sub(1)
                            .ok_or(Error::Invalid(format!(
                                "{} @ line {}: Zero index in face.",
                                line,
                                i + 1
                            )))?;

                        mesh.push(index);
                    }
                }

                _ => {}
            }
        }

        Ok(Obj {
            vertices,
            mesh,
            uvs: texture,
            normals,
        })
    }
}
