use crate::logger::*;

use super::{
    layout::VertexLayout,
    mesh_error::MeshError
};

#[derive(Debug)]
pub struct MeshData {
    vertices: Vec<f32>,
    indices: Option<Vec<u32>>,
    layout: VertexLayout,
    vertex_count: usize
}

impl MeshData {
    pub fn new(vertices: Vec<f32>, indices: Option<Vec<u32>>, layout: VertexLayout) -> Result<Self, MeshError> {
        if layout.stride() == 0 { return Err(MeshError::EmptyLayout); }
        if layout.attribute().iter().any(|a| {
            a.components == 0 || a.components > 4
        }) { return Err(MeshError::InvalidAttributeSize); }

        let components = layout.stride() / std::mem::size_of::<f32>();

        if vertices.is_empty() || vertices.len() % components != 0 {
            log(LevelOfLog::Error, format!("Invalid Vertex data: {:?}", vertices));
            return Err(MeshError::InvalidVertexData);
        }

        let vertex_count = vertices.len() / components;

        if let Some(ref indices) = indices {
            for &index in indices {
                if index as usize >= vertex_count {
                    return Err(MeshError::InvalidIndex(index as usize));
                }
            }
        }

        Ok(Self {
            vertices,
            indices,
            layout,
            vertex_count
        })
    }

    pub fn vertices(&self) -> &[f32] { &self.vertices }
    pub fn indices(&self) -> Option<&[u32]> { self.indices.as_deref() }
    pub fn layout(&self) -> &VertexLayout { &self.layout }
    pub fn vertex_count(&self) -> usize { self.vertex_count }
    pub fn index_count(&self) -> usize { self.indices.as_ref().map_or(0, Vec::len) }
}