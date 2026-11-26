//use unknown_engine::graphics::shader::ShaderSource;
use unknown_engine::platforms::*;
use unknown_engine::logger::*;
use unknown_engine::graphics::{
    *, mesh::*, shader::*
};

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    let mut renderer = GraphicsFactory::create(GraphicsAPI::OpenGL, window);

    log(LevelOfLog::Info, format!("{}", renderer.get_string(0x1F02)));
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);

    let shader_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    //let vertex = ShaderSource::from_file(ShaderStage::Vertex, shader_path.join("./assets/shaders/basic.vert")).expect("Failed to load Vertex shader");
    //let fragment = ShaderSource::from_file(ShaderStage::Fragment, shader_path.join("./assets/shaders/basic.frag")).expect("Failed to load Fragment shader");

    //let program = renderer.create_shader(&vertex, &fragment).expect("Failed to create shader");

    let vertices = vec![
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
         0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
         0.5,  0.5, 0.0, 0.0, 0.0, 1.0,
        -0.5,  0.5, 0.0, 1.0, 1.0, 1.0
    ];

    let indices = vec![
        0, 1, 2,
        2, 3, 0
    ];

    let layout = VertexLayout::new(vec![
        VertexAttribute::new(3),
        VertexAttribute::new(3)
    ]);

    let data = MeshData::new(vertices, Some(indices), layout).expect("Failed to create data for Mesh");
    let mesh = renderer.create_mesh(&data).expect("Failed to create Mesh");

    while window.is_running() {
        renderer.begin_frame();

        //renderer.use_shader(program);
        renderer.draw_mesh(mesh);

        renderer.end_frame();
        window.poll_event();
    }

    renderer.destroy_mesh(mesh);
    //renderer.destroy_shader(program);
}