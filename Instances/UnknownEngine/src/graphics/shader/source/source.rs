use std::path::PathBuf;

use crate::graphics::shader::{
    ShaderSourceFormat,
    ShaderStage
};

#[derive(Debug, Clone)]
pub enum ShaderSourceData {
    Glsl(String),
    SpirV(Vec<u32>),
    File {
        path: PathBuf,
        format: ShaderSourceFormat
    }
}

#[derive(Debug, Clone)]
pub struct ShaderSource {
    stage: ShaderStage,
    data: ShaderSourceData
}

impl ShaderSource {
    pub fn glsl(stage: ShaderStage, source: impl Into<String>) -> Self {
        Self {
            stage,
            data: ShaderSourceData::Glsl(source.into())
        }
    }

    pub fn spirv(stage: ShaderStage, bytecode: Vec<u32>) -> Self {
        Self {
            stage,
            data: ShaderSourceData::SpirV(bytecode)
        }
    }

    pub fn file(stage: ShaderStage, path: impl Into<PathBuf>, format: ShaderSourceFormat) -> Self {
        Self {
            stage,
            data: ShaderSourceData::File {
                path: path.into(),
                format
            }
        }
    }

    pub fn stage(&self) -> ShaderStage { self.stage }
    pub fn data(&self) -> &ShaderSourceData { &self.data }
}