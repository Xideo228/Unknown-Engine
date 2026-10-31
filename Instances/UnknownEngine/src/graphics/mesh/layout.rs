#[derive(Debug, Clone, Copy)]
pub struct VertexAttribute {
    pub components: u8
}

impl VertexAttribute {
    pub const fn new(components: u8) -> Self {
        Self { components }
    }

    pub const fn size(&self) -> usize {
        self.components as usize * std::mem::size_of::<f32>()
    }
}

#[derive(Debug, Clone)]
pub struct VertexLayout {
    attributes: Vec<VertexAttribute>,
    stride: usize
}

impl VertexLayout {
    pub fn new(attributes: Vec<VertexAttribute>) -> Self {
        let stride = attributes.iter().map(|attribute| attribute.size()).sum();

        Self { attributes, stride }
    }

    pub fn attribute(&self) -> &[VertexAttribute] { &self.attributes }
    pub fn stride(&self) -> usize { self.stride }

    pub fn offset(&self) -> impl Iterator<Item = usize> + '_ {
        let mut offset = 0;

        self.attributes.iter().map(move |attribute| {
            let current = offset;
            offset += attribute.size();
            current
        })
    }
}