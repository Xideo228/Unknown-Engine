use super::managers::*;

pub struct AssetManager {
    mesh: MeshManager,
    shader: ShaderManager,
    texture: TextureManager,
    material: MaterialManager
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            mesh: MeshManager::new(),
            shader: ShaderManager::new(),
            texture: TextureManager::new(),
            material: MaterialManager::new()
        }
    }

    pub fn mesh(&self) -> &MeshManager { &self.mesh }
    pub fn shader(&self) -> &ShaderManager { &self.shader }
    pub fn texture(&self) -> &TextureManager { &self.texture }
    pub fn material(&self) -> &MaterialManager { &self.material }
}