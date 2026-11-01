use std::{
    ptr::null,
    collections::HashMap,
    mem::transmute_copy,
    ffi::{
        CStr, c_char,
        c_void
    }
};
use super::{
    buffers::*,
    mesh::GLMesh,
    windows::wgl::*
};
use crate::{
    Window, graphics::{
        backend::GraphicsBackend, mesh::*, opengl::{
            constants::*,
            gdi32
        }
    }, library::Library, logger
};

#[cfg(target_os = "windows")]
const OPENGL: &str = "opengl32.dll";

type GlClearColor = unsafe extern "system" fn (f32, f32, f32, f32);
type GlViewport = unsafe extern "system" fn (i32, i32, i32, i32);
type GlClear = unsafe extern "system" fn (u32);

type GlGetString = unsafe extern "system" fn (u32) -> *const u8;
type GlGetError = unsafe extern "system" fn () -> u32;

type GlCreateShader = unsafe extern "system" fn (u32) -> u32;
type GlDeleteShader = unsafe extern "system" fn (u32);
type GlAttachShader = unsafe extern "system" fn (u32, u32);
type GlShaderSource = unsafe extern "system" fn (u32, i32, *const *const c_char, *const i32);
type GlCompileShader = unsafe extern "system" fn (u32);
type GlGetShaderIv = unsafe extern "system" fn (u32, u32, *mut i32);
type GlGetShaderInfoLog = unsafe extern "system" fn (u32, i32, *mut i32, *mut c_char);

type GlCreateProgram = unsafe extern "system" fn () -> u32;
type GlLinkProgram = unsafe extern "system" fn (u32);
type GlGetProgramIv = unsafe extern "system" fn (u32, u32, *mut i32);
type GlGetProgramInfoLog = unsafe extern "system" fn (u32, i32, *mut i32, *mut c_char);
type GlDeleteProgram = unsafe extern "system" fn (u32);
type GlUseProgram = unsafe extern "system" fn (u32);

type GlGenVertexArrays = unsafe extern "system" fn (i32, *mut u32);
type GlBindVertexArray = unsafe extern "system" fn (u32);
type GlDeleteVertexArrays = unsafe extern "system" fn (i32, *const u32);

type GlGenBuffers = unsafe extern "system" fn (i32, *mut u32);
type GlBindBuffer = unsafe extern "system" fn (u32, u32);
type GlBufferData = unsafe extern "system" fn (u32, isize, *const c_void, u32);
type GlDeleteBuffers = unsafe extern "system" fn (i32, *const u32);

type GlEnableVertexAttribArray = unsafe extern "system" fn (u32);
type GlVertexAttribPointer = unsafe extern "system" fn (u32, i32, u32, u8, i32, *const c_void);

type GlDrawArrays = unsafe extern "system" fn (u32, i32, i32);
type GlDrawElements = unsafe extern "system" fn (u32, i32, u32, *const c_void);

pub struct GL {
    window: Window,
    color: [f32; 4],

    meshes: HashMap<u32, GLMesh>,
    next_id_mesh: u32,

    pub(crate) clear_color: GlClearColor,
    pub(crate) viewport: GlViewport,
    pub(crate) clear: GlClear,

    pub(crate) get_string: GlGetString,
    pub(crate) get_error: GlGetError,

    pub(crate) create_shader: GlCreateShader,
    pub(crate) delete_shader: GlDeleteShader,
    pub(crate) shader_source: GlShaderSource,
    pub(crate) attach_shader: GlAttachShader,
    pub(crate) compile_shader: GlCompileShader,
    pub(crate) get_shader_iv: GlGetShaderIv,
    pub(crate) get_shader_info_log: GlGetShaderInfoLog,

    pub(crate) create_program: GlCreateProgram,
    pub(crate) link_program: GlLinkProgram,
    pub(crate) get_program_iv: GlGetProgramIv,
    pub(crate) get_program_info_log: GlGetProgramInfoLog,
    pub(crate) delete_program: GlDeleteProgram,
    pub(crate) use_program: GlUseProgram,

    pub(crate) gen_vertex_arrays: GlGenVertexArrays,
    pub(crate) bind_vertex_array: GlBindVertexArray,
    pub(crate) delete_vertex_arrays: GlDeleteVertexArrays,

    pub(crate) gen_buffers: GlGenBuffers,
    pub(crate) bind_buffer: GlBindBuffer,
    pub(crate) buffer_data: GlBufferData,
    pub(crate) delete_buffers: GlDeleteBuffers,

    pub(crate) enable_vertex_attrib_array: GlEnableVertexAttribArray,
    pub(crate) vertex_attrib_pointer: GlVertexAttribPointer,

    pub(crate) draw_arrays: GlDrawArrays,
    pub(crate) draw_elements: GlDrawElements
}

impl GL {
    pub fn new(win: Window) -> Self {
        let opengl32 = Library::new(OPENGL).expect("Failed to load library");

        Self {
            window: win,
            color: [1.0, 0.0, 0.0, 1.0],

            meshes: HashMap::new(),
            next_id_mesh: 0,

            clear: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glClear\0"),
            viewport: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glViewport\0"),
            clear_color: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glClearColor\0"),

            get_string: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGetString\0"),
            get_error: load_function(&opengl32, &mut |name| {wgl_get_proc_address(name) }, b"glGetError\0"),

            create_shader: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glCreateShader\0"),
            delete_shader: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glDeleteShader\0"),
            shader_source: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glShaderSource\0"),
            attach_shader: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glAttachShader\0"),
            compile_shader: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glCompileShader\0"),
            get_shader_iv: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGetShaderiv\0"),
            get_shader_info_log: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGetShaderInfoLog\0"),

            create_program: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glCreateProgram\0"),
            link_program: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glLinkProgram\0"),
            get_program_iv: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGetProgramiv\0"),
            get_program_info_log: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGetProgramInfoLog\0"),
            delete_program: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glDeleteProgram\0"),
            use_program: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glUseProgram\0"),

            gen_vertex_arrays: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGenVertexArrays\0"),
            bind_vertex_array: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glBindVertexArray\0"),
            delete_vertex_arrays: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glDeleteVertexArrays\0"),

            gen_buffers: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glGenBuffers\0"),
            bind_buffer: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glBindBuffer\0"),
            buffer_data: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glBufferData\0"),
            delete_buffers: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glDeleteBuffers\0"),

            enable_vertex_attrib_array: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glEnableVertexAttribArray\0"),
            vertex_attrib_pointer: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glVertexAttribPointer\0"),

            draw_arrays: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glDrawArrays\0"),
            draw_elements: load_function(&opengl32, &mut |name| { wgl_get_proc_address(name) }, b"glDrawElements\0")
        }
    }

    fn compile_shader(&self, shader_type: u32, source: String) -> Result<u32, String> {
        use std::ffi::CString;

        let source = CString::new(source).map_err(|_| "Shader source contains null byte".to_string())?;

        unsafe {
            let shader = (self.create_shader)(shader_type);
            if shader == 0 { return Err("Failed to create Shader".into()); }

            let source_ptr = source.as_ptr();

            (self.shader_source)(shader, 1, &source_ptr, std::ptr::null());
            (self.compile_shader)(shader);

            let mut status = 0;

            (self.get_shader_iv)(shader, GL_COMPILE_STATUS, &mut status);

            if status == 0 {
                let error = self.get_shader_error(shader);
                (self.delete_shader)(shader);
                return  Err(error)
            }
            Ok(shader)
        }
    }

    fn get_shader_error(&self, shader: u32) -> String {
        unsafe {
            let mut length = 0;

            (self.get_shader_iv)(shader, GL_INFO_LOG_LENGTH, &mut length);

            if length <= 1 { return "Unknown shader compilation error".into() }

            let mut buffer = vec![0u8; length as usize];

            (self.get_shader_info_log)(shader, length, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut c_char);

            CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
        }
    }

    pub fn create_shader_program(&self, vertex_source: &str, fragment_source: &str) -> Result<u32, String> {
        let vertex = self.compile_shader(GL_VERTEX_SHADER, vertex_source.to_string())?;
        let fragment = match self.compile_shader(GL_FRAGMENT_SHADER, fragment_source.to_string()) {
            Ok(shader) => shader,
            Err(error) => {
                unsafe { (self.delete_shader)(vertex); }
                return Err(error);
            }
        };

        unsafe {
            let program = (self.create_program)();

            if program == 0 {
                (self.delete_shader)(fragment);
                (self.delete_shader)(vertex);

                return Err("Failed to create shader program".into());
            }

            (self.attach_shader)(program, vertex);
            (self.attach_shader)(program, fragment);
            (self.link_program)(program);

            let mut status = 0;

            (self.get_program_iv)(program, GL_LINK_STATUS, &mut status);

            (self.delete_shader)(vertex);
            (self.delete_shader)(fragment);

            if status == 0 {
                let error = self.get_program_error(program);
                (self.delete_program)(program);

                return Err(error);
            }

            Ok(program)
        }
    }

    pub fn use_program(&self, program: u32) {
        unsafe { (self.use_program)(program); }
    }

    fn get_program_error(&self, program: u32) -> String {
        unsafe {
            let mut lenght = 0;

            (self.get_program_iv)(program, GL_INFO_LOG_LENGTH, &mut lenght);

            if lenght <= 1 { return "Unknown program compile error".into(); }

            let mut buffer = vec![0u8; lenght as usize];
            (self.get_program_info_log)(program, lenght, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut c_char);

            CStr::from_ptr(buffer.as_ptr() as *const c_char).to_string_lossy().into_owned()
        }
    }

    pub fn draw_arrays(&self, mode: u32, count: i32) { unsafe { (self.draw_arrays)(mode, 0, count); } }
    pub fn draw_elements(&self, count: i32) { unsafe { (self.draw_elements)(GL_TRIANGLES, count, GL_UNSIGNED_INT, null()); } }

    pub fn check_error(&self) {
        unsafe {
            let error = (self.get_error)();

            if error != 0 {
                logger::print(logger::LevelOfLog::Error, format!("OpenGL error: 0x{:X}", error));
            }
        }
    }

}

impl GraphicsBackend for GL {
    fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) { self.color = [r, g, b, a]; }

    fn begin_frame(&mut self) {
        unsafe {
            (self.clear_color)(self.color[0], self.color[1], self.color[2], self.color[3]);
            (self.clear)(GL_COLOR_BUFFER_BIT);
        }
    }

    fn end_frame(&mut self) {
        unsafe { gdi32::SwapBuffers(self.window.hdc); }
    }

    fn get_string(&mut self, text: u32) -> &str {
        unsafe {
            let ptr = (self.get_string)(text) as *const c_char;
            let c_str = CStr::from_ptr(ptr);
            match c_str.to_str() {
                Ok(result) => result,
                Err(_) => ""
            }
        }
    }

    fn create_shader(&mut self, _vertex_source: &str, _fragment_source: &str) { todo!() }

    fn create_mesh(&mut self, data: &MeshData) -> Result<MeshHandle, String> {
        let vertex_count = i32::try_from(data.vertex_count()).map_err(|_| "Too many vertices".to_string())?;
        let index_count = i32::try_from(data.index_count()).map_err(|_| "Too many indices".to_string())?;

        let vao = create_vertex_array(&self);
        if vao == 0 { return  Err("Failed to create VAO".into()); }
        logger::print(logger::LevelOfLog::Info, format!("VAO: {}", vao));

        let vbo = create_buffer(&self);
        if vbo == 0 {
            delete_vertex_array(&self, vao);
            return Err("Failed to create VBO".into());
        }
        logger::print(logger::LevelOfLog::Info, format!("VBO: {}", vbo));

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

            logger::print(logger::LevelOfLog::Info, format!("EBO: {}", buffer));

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
        logger::print(logger::LevelOfLog::Warning, "DROPPED!!!".to_string());
    }
}

fn load_function<T, F>(lib: &Library, loader: &mut F, name: &'static [u8]) -> T where F: FnMut(&'static [u8]) -> *const c_void {
    assert_eq!(size_of::<T>(), size_of::<*const c_void>(), "Invalid OpenGL function pointer size");

    let mut ptr = loader(name);
    let address = ptr as usize;

    if address <= 3 || address == usize::MAX { ptr = lib.get_function(name).expect("Failed to load OpenGL function"); } 
    if ptr.is_null() { panic!("Failed to load OpenGL function: {}", String::from_utf8_lossy(&name[..name.len() - 1])); }

    logger::print(logger::LevelOfLog::Info, format!("Succesfully loaded: {}", str::from_utf8(name).unwrap()));
    unsafe { transmute_copy(&ptr) }
}