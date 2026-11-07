use std::{
    collections::HashMap,
    ffi::{
        CStr,
        c_char,
    }
};
use super::{
    gdi32,
    buffers::*,
    constants::*,
    mesh::GLMesh,
    shader::*,
    functions::GLFunctions
};
use crate::{
    graphics::{
        backend::GraphicsBackend, mesh::*, shader::*,
    }, logger::*, platforms::Window
};

pub struct GL {
    pub(crate) window: Window,
    pub(crate) color: [f32; 4],

    pub(crate) meshes: HashMap<u32, GLMesh>,
    pub(crate) next_id_mesh: u32,

    pub(crate) shaders: HashMap<u32, GLShaderProgram>,
    pub(crate) next_id_shader: u32,

    pub(crate) functions: GLFunctions
}

impl GL {
    pub fn new(win: Window) -> Self {
        Self {
            window: win,
            color: [1.0, 1.0, 1.0, 1.0],

            meshes: HashMap::new(),
            next_id_mesh: 0,

            shaders: HashMap::new(),
            next_id_shader: 0,

            functions: GLFunctions::new()
        }
    }

    fn compile_shader(&self, shader_type: u32, source: String) -> Result<u32, String> {
        use std::ffi::CString;

        let source = CString::new(source).map_err(|_| "Shader source contains null byte".to_string())?;

        unsafe {
            let shader = (self.functions.create_shader)(shader_type);
            if shader == 0 { return Err("Failed to create Shader".into()); }

            let source_ptr = source.as_ptr();

            (self.functions.shader_source)(shader, 1, &source_ptr, std::ptr::null());
            (self.functions.compile_shader)(shader);

            let mut status = 0;

            (self.functions.get_shader_iv)(shader, GL_COMPILE_STATUS, &mut status);

            if status == 0 {
                let error = self.get_shader_error(shader);
                (self.functions.delete_shader)(shader);
                return  Err(error)
            }
            Ok(shader)
        }
    }

    fn get_shader_error(&self, shader: u32) -> String {
        unsafe {
            let mut length = 0;

            (self.functions.get_shader_iv)(shader, GL_INFO_LOG_LENGTH, &mut length);

            if length <= 1 { return "Unknown shader compilation error".into() }

            let mut buffer = vec![0u8; length as usize];

            (self.functions.get_shader_info_log)(shader, length, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut c_char);

            CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
        }
    }

    pub fn create_shader_program(&self, vertex_source: &str, fragment_source: &str) -> Result<u32, String> {
        let vertex = self.compile_shader(GL_VERTEX_SHADER, vertex_source.to_string())?;
        let fragment = match self.compile_shader(GL_FRAGMENT_SHADER, fragment_source.to_string()) {
            Ok(shader) => shader,
            Err(error) => {
                unsafe { (self.functions.delete_shader)(vertex); }
                return Err(error);
            }
        };

        unsafe {
            let program = (self.functions.create_program)();

            if program == 0 {
                (self.functions.delete_shader)(fragment);
                (self.functions.delete_shader)(vertex);

                return Err("Failed to create shader program".into());
            }

            (self.functions.attach_shader)(program, vertex);
            (self.functions.attach_shader)(program, fragment);
            (self.functions.link_program)(program);

            let mut status = 0;

            (self.functions.get_program_iv)(program, GL_LINK_STATUS, &mut status);

            (self.functions.delete_shader)(vertex);
            (self.functions.delete_shader)(fragment);

            if status == 0 {
                let error = self.get_program_error(program);
                (self.functions.delete_program)(program);

                return Err(error);
            }

            Ok(program)
        }
    }

    pub fn use_program(&self, program: u32) {
        unsafe { (self.functions.use_program)(program); }
    }

    fn get_program_error(&self, program: u32) -> String {
        unsafe {
            let mut lenght = 0;

            (self.functions.get_program_iv)(program, GL_INFO_LOG_LENGTH, &mut lenght);

            if lenght <= 1 { return "Unknown program compile error".into(); }

            let mut buffer = vec![0u8; lenght as usize];
            (self.functions.get_program_info_log)(program, lenght, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut c_char);

            CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
        }
    }

    pub fn draw_arrays(&self, mode: u32, count: i32) { unsafe { (self.functions.draw_arrays)(mode, 0, count); } }
    pub fn draw_elements(&self, count: i32) { unsafe { (self.functions.draw_elements)(GL_TRIANGLES, count, GL_UNSIGNED_INT, std::ptr::null()); } }

    pub fn check_error(&self) {
        unsafe {
            let error = (self.functions.get_error)();

            if error != 0 {
                log(LevelOfLog::Critical, format!("OpenGL error: 0x{:X}", error));
            }
        }
    }

}

impl GraphicsBackend for GL {
    fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) { self.color = [r, g, b, a]; }

    fn begin_frame(&mut self) {
        unsafe {
            (self.functions.clear_color)(self.color[0], self.color[1], self.color[2], self.color[3]);
            (self.functions.clear)(GL_COLOR_BUFFER_BIT);
        }
    }

    fn end_frame(&mut self) {
        unsafe { gdi32::SwapBuffers(self.window.hdc); }
    }

    fn get_string(&mut self, text: u32) -> &str {
        unsafe {
            let ptr = (self.functions.get_string)(text) as *const c_char;
            let c_str = CStr::from_ptr(ptr);
            match c_str.to_str() {
                Ok(result) => result,
                Err(_) => ""
            }
        }
    }

    fn create_shader(&mut self, vertex: &ShaderSource, fragment: &ShaderSource) -> Result<crate::graphics::shader::ShaderHandle, String> {
        if vertex.stage != ShaderStage::Vertex { return Err("Excepted a vertex shader source".into()); }
        if fragment.stage != ShaderStage::Fragment { return Err("Excepted a fragment shader source".into()); }

        let program = create_shader_program(&self, &vertex.source, &fragment.source)?;
        let handle = ShaderHandle::new(self.next_id_shader);

        self.shaders.insert(handle.id(), GLShaderProgram::new(program));
        self.next_id_shader += 1;

        Ok(handle)
    }
    
    fn destroy_shader(&mut self, handle: ShaderHandle) {
        let Some(shader) = self.shaders.remove(&handle.id()) else { return; };

        unsafe { (self.functions.delete_shader)(shader.id()); }
    }

    fn use_shader(&mut self, handle: ShaderHandle) {
        let Some(shader) = self.shaders.get(&handle.id()) else {
            log(LevelOfLog::Error, format!("Shader handle not found"));
            return;
        };

        let program = shader.id();

        use_shader(&self, program);
    }

    fn create_mesh(&mut self, data: &MeshData) -> Result<MeshHandle, String> {
        let vertex_count = i32::try_from(data.vertex_count()).map_err(|_| "Too many vertices".to_string())?;
        let index_count = i32::try_from(data.index_count()).map_err(|_| "Too many indices".to_string())?;

        let vao = create_vertex_array(&self);
        if vao == 0 { return  Err("Failed to create VAO".into()); }
        log(LevelOfLog::Info, format!("VAO: {}", vao));

        let vbo = create_buffer(&self);
        if vbo == 0 {
            delete_vertex_array(&self, vao);
            return Err("Failed to create VBO".into());
        }
        log(LevelOfLog::Info, format!("VBO: {}", vbo));

        bind_vertex_array(&self, vao);

        bind_buffer(&self, vbo);
        upload_vertex_data(&self, data.vertices());
        
        let stride = i32::try_from(data.layout().stride()).map_err(|_| "Vertex stride is too large".to_string())?;

        for (index, (attribute, offset)) in data
            .layout().attribute().iter().zip(data.layout().offset()).enumerate()
        { vertex_attribute(self, index as u32, attribute.components as i32, stride, offset); }

        let ebo = if let Some(indices) = data.indices() {
            let buffer = create_buffer(&self);

            if buffer == 0 {
                bind_vertex_array(&self, 0);
                delete_buffer(&self, vbo);
                delete_vertex_array(&self, vao);

                return Err("Failed to create EBO".into());
            }

            log(LevelOfLog::Info, format!("EBO: {}", buffer));

            bind_index_buffer(&self, buffer);
            upload_index_data(&self, indices);

            Some(buffer)
        } else {
            None
        };

        bind_vertex_array(&self, 0);

        let id = self.next_id_mesh;
        self.next_id_mesh = self.next_id_mesh.checked_add(1).ok_or_else(|| "Mesh handle limit reached".to_string())?;

        self.meshes.insert(id, GLMesh { vao, vbo, ebo, vertex_count, index_count });

        Ok(MeshHandle::new(id))
    }

    fn draw_mesh(&mut self, handle: MeshHandle) {
        let Some(mesh) = self.meshes.get(&handle.id()) else { return; };

        bind_vertex_array(&self, mesh.vao);

        if mesh.ebo.is_some() { self.draw_elements(mesh.index_count); }
        else { self.draw_arrays(GL_TRIANGLES, mesh.vertex_count); }

        bind_vertex_array(&self, 0);
    }

    fn destroy_mesh(&mut self, handle: MeshHandle) {
        let Some(mesh) = self.meshes.remove(&handle.id()) else { return; };

        bind_vertex_array(&self, 0);

        if let Some(ebo) = mesh.ebo { delete_buffer(&self, ebo); }

        delete_buffer(&self, mesh.vbo);
        delete_vertex_array(&self, mesh.vao);
    }
}

impl Drop for GL {
    fn drop(&mut self) {
        log(LevelOfLog::Warning, "DROPPED!!!".to_string());
    }
}