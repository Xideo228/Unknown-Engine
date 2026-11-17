use super::macro_value::ShaderMacroValue;
use std::{
    collections::HashMap,
    path::PathBuf
};

#[derive(Debug, Clone, Default)]
pub struct ShaderPreproccessorConfig {
    defines: HashMap<String, Option<String>>,
    include: Vec<PathBuf>
}

impl ShaderPreproccessorConfig {
    pub fn define(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.defines.insert(name.into(), Some(value.into()));
    }

    pub fn undefine(&mut self, name: impl Into<String>) {
        self.defines.insert(name.into(), None);
    }

    pub fn add_include_directory(&mut self, path: impl Into<PathBuf>) {
        self.include.push(path.into());
    }
}