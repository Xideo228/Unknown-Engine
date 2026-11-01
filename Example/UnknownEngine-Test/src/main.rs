use unknown_engine::*;
use unknown_engine::graphics::*;

fn main() {
    let settings= WindowSettings::default();
    let mut window = WindowManager::create(settings);

    let mut renderer = GraphicsFactory::create(GraphicsAPI::OpenGL, window);

    println!("{}", renderer.get_string(0x1F02));
    renderer.set_clear_color(0.5, 0.0, 1.0, 1.0);

    let vertex = "#version 330 core\n
    \nlayout (location = 0) in vec3 a_position;
    \nlayout (location = 1) in vec3 a_color;\n
    \nout vec3 v_color;
    \nvoid main() {\n
    \tgl_Position = vec4(a_position, 1.0);\n
    \tv_color = a_color;
    \n}";

    let fragment = "#version 330 core\n
    \nin vec3 v_color;\n
    \nout vec4 FragColor;\n
    \nvoid main() {\n
    \tFragColor = vec4(v_color, 1.0);
    \n}";

    let program = renderer.create_shader_program(vertex, fragment).expect("Failed to create shader");

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

        renderer.use_program(program);
        renderer.draw_mesh(mesh);

        renderer.end_frame();
        window.poll_event();
    }

    renderer.destroy_mesh(mesh);
}