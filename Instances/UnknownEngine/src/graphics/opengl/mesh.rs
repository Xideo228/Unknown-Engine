pub(crate) struct GLMesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: Option<u32>,
    pub vertex_count: i32,
    pub index_count: i32
}