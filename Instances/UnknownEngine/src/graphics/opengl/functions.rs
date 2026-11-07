use crate::{
    library::Library, logger::*
};
use std::{
    ffi::{
        c_char, c_void
    }, 
    mem::transmute_copy
};

#[cfg(target_os = "windows")]
use super::windows::wgl::wgl_get_proc_address;

#[cfg(target_os = "windows")]
const OPENGL: &str = "opengl32.dll";

type GlClearColor = unsafe extern "system" fn (red: f32, green: f32, blue: f32, alpha: f32);
type GlClear = unsafe extern "system" fn (mask: u32);
type GlViewport = unsafe extern "system" fn (x: i32, y: i32, width: i32, height: i32);

type GlGetString = unsafe extern "system" fn (addr: u32) -> *const u8;
type GlGetError = unsafe extern "system" fn () -> u32;

type GlCreateShader = unsafe extern "system" fn (shader_type: u32) -> u32;
type GlShaderSource = unsafe extern "system" fn (shader: u32, count: i32,
    source: *const *const c_char, length: *const i32);
type GlCompileShader = unsafe extern "system" fn (shader: u32);
type GlAttachShader = unsafe extern "system" fn (program: u32, shader: u32);
type GlDeleteShader = unsafe extern "system" fn (shader: u32);
type GlGetShaderIv = unsafe extern "system" fn (shader: u32, pname: u32, params: *mut i32);
type GlGetShaderInfoLog = unsafe extern "system" fn (shader: u32, max_lenght: i32,
    lenght: *mut i32, info_log: *mut c_char);

type GlCreateProgram = unsafe extern "system" fn () -> u32;
type GlLinkProgram = unsafe extern "system" fn (program: u32);
type GlUseProgram = unsafe extern "system" fn (program: u32);
type GlDeleteProgram = unsafe extern "system" fn (program: u32);
type GlGetProgramIv = unsafe extern "system" fn (program: u32, pname: u32, params: *mut i32);
type GlGetProgramInfoLog = unsafe extern "system" fn (program: u32, max_lenght: i32,
    lenght: *mut i32, info_log: *mut c_char);

type GlGetUniformLocation = unsafe extern "system" fn (program: u32, name: *const c_char) -> i32;
type GlUniform1f = unsafe extern "system" fn (location: i32, value: f32);
type GlUniform3f = unsafe extern "system" fn (location: i32, x: f32, y: f32, z: f32);
type GlUniformMatrix4fv = unsafe extern "system" fn (location: i32, count: i32, transpose: u8, value: *const f32);

type GlGenVertexArray = unsafe extern "system" fn (n: i32, arrays: *mut u32);
type GlBinfVertexArray = unsafe extern "system" fn (array: u32);
type GlDeleteVertexArray = unsafe extern "system" fn (n: i32, arrays: *const u32);

type GlGenBuffers = unsafe extern "system" fn (n: i32, buffers: *mut u32);
type GlBindBuffer = unsafe extern "system" fn (target: u32, buffer: u32);
type GlBufferData = unsafe extern "system" fn (target: u32, size: isize, data: *const c_void, usage: u32);
type GlDeleteBuffers = unsafe extern "system" fn (n: i32, buffers: *const u32);

type GlEnableVertexAttribArray = unsafe extern "system" fn (index: u32);
type GlDisableVertexAttribArray = unsafe extern "system" fn (index: u32);
type GlVertexAttribPointer = unsafe extern "system" fn (index: u32, size: i32, etype: u32,
    normalized: u8, stride: i32, pointer: *const c_void);

type GlDrawArrays = unsafe extern "system" fn (mode: u32, first: i32, count: i32);
type GlDrawElements = unsafe extern "system" fn (mode: u32, count: i32, etype: u32, indices: *const c_void);

pub struct GLFunctions {
    pub(crate) clear_color: GlClearColor,
    pub(crate) clear: GlClear,
    pub(crate) viewport: GlViewport,

    pub(crate) get_string: GlGetString,
    pub(crate) get_error: GlGetError,

    pub(crate) create_shader: GlCreateShader,
    pub(crate) shader_source: GlShaderSource,
    pub(crate) compile_shader: GlCompileShader,
    pub(crate) attach_shader: GlAttachShader,
    pub(crate) delete_shader: GlDeleteShader,
    pub(crate) get_shader_iv: GlGetShaderIv,
    pub(crate) get_shader_info_log: GlGetShaderInfoLog,

    pub(crate) create_program: GlCreateProgram,
    pub(crate) link_program: GlLinkProgram,
    pub(crate) use_program: GlUseProgram,
    pub(crate) delete_program: GlDeleteProgram,
    pub(crate) get_program_iv: GlGetProgramIv,
    pub(crate) get_program_info_log: GlGetProgramInfoLog,

    pub(crate) get_uniform_location: GlGetUniformLocation,
    pub(crate) uniform_1f: GlUniform1f,
    pub(crate) uniform_3f: GlUniform3f,
    pub(crate) uniform_matrix_4fv: GlUniformMatrix4fv,

    pub(crate) gen_vertex_arrays: GlGenVertexArray,
    pub(crate) bind_vertex_array: GlBinfVertexArray,
    pub(crate) delete_vertex_arrays: GlDeleteVertexArray,

    pub(crate) gen_buffers: GlGenBuffers,
    pub(crate) bind_buffer: GlBindBuffer,
    pub(crate) buffer_data: GlBufferData,
    pub(crate) delete_buffers: GlDeleteBuffers,

    pub(crate) enable_vertex_attrib_array: GlEnableVertexAttribArray,
    pub(crate) disable_vertex_attrib_array: GlDisableVertexAttribArray,
    pub(crate) vertex_attrib_pointer: GlVertexAttribPointer,

    pub(crate) draw_arrays: GlDrawArrays,
    pub(crate) draw_elements: GlDrawElements,

    library: Library
}

impl GLFunctions {
    pub fn new() -> Self {
        let lib = Library::new(OPENGL).expect("Failed to load library");

        Self {
            clear_color: Self::load_function(&lib, b"glClearColor\0"),
            clear: Self::load_function(&lib, b"glClear\0"),
            viewport: Self::load_function(&lib, b"glViewport\0"),

            get_string: Self::load_function(&lib, b"glGetString\0"),
            get_error: Self::load_function(&lib, b"glGetError\0"),

            create_shader: Self::load_function(&lib, b"glCreateShader\0"),
            shader_source: Self::load_function(&lib, b"glShaderSource\0"),
            compile_shader: Self::load_function(&lib, b"glCompileShader\0"),
            attach_shader: Self::load_function(&lib, b"glAttachShader\0"),
            delete_shader: Self::load_function(&lib, b"glDeleteShader\0"),
            get_shader_iv: Self::load_function(&lib, b"glGetShaderiv\0"),
            get_shader_info_log: Self::load_function(&lib, b"glGetShaderInfoLog\0"),

            create_program: Self::load_function(&lib, b"glCreateProgram\0"),
            link_program: Self::load_function(&lib, b"glLinkProgram\0"),
            use_program: Self::load_function(&lib, b"glUseProgram\0"),
            delete_program: Self::load_function(&lib, b"glDeleteProgram\0"),
            get_program_iv: Self::load_function(&lib, b"glGetProgramiv\0"),
            get_program_info_log: Self::load_function(&lib, b"glGetProgramInfoLog\0"),

            get_uniform_location: Self::load_function(&lib, b"glGetUniformLocation\0"),
            uniform_1f: Self::load_function(&lib, b"glUniform1f\0"),
            uniform_3f: Self::load_function(&lib, b"glUniform3f\0"),
            uniform_matrix_4fv: Self::load_function(&lib, b"glUniformMatrix4fv\0"),

            gen_vertex_arrays: Self::load_function(&lib, b"glGenVertexArrays\0"),
            bind_vertex_array: Self::load_function(&lib, b"glBindVertexArray\0"),
            delete_vertex_arrays: Self::load_function(&lib, b"glDeleteVertexArrays\0"),

            gen_buffers: Self::load_function(&lib, b"glGenBuffers\0"),
            bind_buffer: Self::load_function(&lib, b"glBindBuffer\0"),
            buffer_data: Self::load_function(&lib, b"glBufferData\0"),
            delete_buffers: Self::load_function(&lib, b"glDeleteBuffers\0"),

            enable_vertex_attrib_array: Self::load_function(&lib, b"glEnableVertexAttribArray\0"),
            disable_vertex_attrib_array: Self::load_function(&lib, b"glDisableVertexAttribArray\0"),
            vertex_attrib_pointer: Self::load_function(&lib, b"glVertexAttribPointer\0"),

            draw_arrays: Self::load_function(&lib, b"glDrawArrays\0"),
            draw_elements: Self::load_function(&lib, b"glDrawElements\0"),
            
            library: lib
        }
    }

    fn load_function<T>(lib: &Library, name: &'static [u8]) -> T {
        assert_eq!(size_of::<T>(), size_of::<*const c_void>(), "OpenGL Function pointer size mismatch");

        #[cfg(target_os = "windows")]
        let mut ptr = wgl_get_proc_address(name);

        let address = ptr as usize;

        if address <= 3 || address == usize::MAX { ptr = lib.get_function(name).expect("Failed to load OpenGL function"); }
        if ptr.is_null() {
            let function_name = name.strip_suffix(&[0]).unwrap_or(name);
            let res = format!("Failed to load OpenGL function: {}", String::from_utf8_lossy(function_name));
            log(LevelOfLog::Critical, res.clone());
            panic!("{}", res);
        }

        let address = ptr as usize;
        let function_name = name.strip_suffix(&[0]).unwrap_or(name);

        log(LevelOfLog::Info, format!("Succesfully loaded OpenGL function: {} at address: 0x{:X}", String::from_utf8_lossy(function_name), address));
        unsafe { transmute_copy(&ptr) }
    }
}