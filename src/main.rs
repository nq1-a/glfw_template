use std::env;

use gl;
use nalgebra_glm as glm;

use gl::types::GLuint;

use glfw::{
    Action,
    Context,
    fail_on_errors,
    Key,
    WindowEvent,
    WindowMode,
};

mod camera;
mod render;
mod util;

use crate::camera::Camera;
use crate::render::{add_uniform, build_shader};
use crate::util::ident_mat4;

type Vertex = [f32; 3];

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Make window
    let (mut window, events) = glfw.create_window(
        1280,
        720,
        "hello world",
        WindowMode::Windowed
    ).unwrap();

    window.make_current();
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    // Load GL
    gl::load_with(|s| window
        .get_proc_address(s)
        .map(|p| p as *const _)
        .unwrap()
    );

    unsafe {
        //gl::Enable(gl::CULL_FACE);
        //gl::CullFace(gl::BACK);
        //gl::FrontFace(gl::CCW);
    }

    // Create camera
    let mut cam = Camera::new(
        glm::vec3(0., 0., 10.),
        -1.570796, 0.
    );

    // Initialize shaders
    const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;
        
        void main() {
            gl_Position = projection * view * model * vec4(pos, 1.0);
        }
    "#;

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;

        void main() {
            final_color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program: GLuint;

    unsafe {
        // Create program
        program = gl::CreateProgram();

        // Build shaders
        build_shader(program, VERT_SHADER, gl::VERTEX_SHADER);
        build_shader(program, FRAG_SHADER, gl::FRAGMENT_SHADER);

        // Use program
        gl::LinkProgram(program);
        gl::UseProgram(program);
    }

    // Generate objects
    unsafe {
        // Vertex array object
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        
        // Vertex buffer object
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo)
    }

    const vertex_data: [Vertex; 36] = [
        [-0.5,  -0.5,   -0.5],
        [0.5,   -0.5,   -0.5],
        [0.5,   0.5,    -0.5],
        [0.5,   0.5,    -0.5],
        [-0.5,  0.5,    -0.5],
        [-0.5,  -0.5,   -0.5],
        [-0.5,  -0.5,   0.5],
        [0.5,   -0.5,   0.5],
        [0.5,   0.5,    0.5],
        [0.5,   0.5,    0.5],
        [-0.5,  0.5,    0.5],
        [-0.5,  -0.5,   0.5],
        [-0.5,  0.5,    0.5],
        [-0.5,  0.5,    -0.5],
        [-0.5,  -0.5,   -0.5],
        [-0.5,  -0.5,   -0.5],
        [-0.5,  -0.5,   0.5],
        [-0.5,  0.5,    0.5],
        [0.5,   0.5,    0.5],
        [0.5,   0.5,    -0.5],
        [0.5,   -0.5,   -0.5],
        [0.5,   -0.5,   -0.5],
        [0.5,   -0.5,   0.5],
        [0.5,   0.5,    0.5],
        [-0.5,  -0.5,   -0.5],
        [0.5,   -0.5,   -0.5],
        [0.5,   -0.5,   0.5],
        [0.5,   -0.5,   0.5],
        [-0.5,  -0.5,   0.5],
        [-0.5,  -0.5,   -0.5],
        [-0.5,  0.5,    -0.5],
        [0.5,   0.5,    -0.5],
        [0.5,   0.5,    0.5],
        [0.5,   0.5,    0.5],
        [-0.5,  0.5,    0.5],
        [-0.5,  0.5,    -0.5],
    ];

    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertex_data) as isize,
            vertex_data.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        gl::EnableVertexAttribArray(0);
    }

    // Initialization of variables
    let args: Vec<String> = env::args().collect();
    let delay: f32 = 1. / (args.get(1)
        .unwrap_or(&String::from("120."))
        .parse::<f32>()
        .unwrap_or(120.)
    + 0.5);
    
    let (mut last_width, mut last_height) = window.get_size();
    let mut last_time: f64 = 0.;
    let mut mouse_x: f64 = 0.;
    let mut mouse_y: f64 = 0.;

    let mut keys: [bool; 1024] = [false; 1024];


    // Main loop
    while !window.should_close() {
        // Get delta time
        let current_time: f64 = window.glfw.get_time();
        let dt: f32 = (current_time - last_time) as f32;

        if dt < delay {continue;}

        //println!("{}", (1./dt) as i32);
        last_time = current_time;

        // Fix window size
        unsafe {
            let (width, height) = window.get_size();

            if last_width != width
            || last_height != height {
                gl::Viewport(0, 0, width, height);
                last_width = width;
                last_height = height;
                println!("{}x{}", last_width, last_height);
            }
        }

        // Events
        for (_, ev) in glfw::flush_messages(&events) {
            match ev {
                WindowEvent::Key(key, _, action, _) => {
                    keys[key as usize] = action == Action::Press;

                    if key == Key::Escape && action == Action::Release {
                        window.set_should_close(true);
                    }
                }

                WindowEvent::CursorPos(x, y) => {
                    cam.rotate(
                        dt * (x - mouse_x) as f32,
                        dt * (mouse_y - y) as f32
                    );

                    mouse_x = x;
                    mouse_y = y;
                }

                _ => {}
            }
        }

        // Movement
        if keys[Key::W as usize] {cam.move_front(3. * dt);}
        if keys[Key::A as usize] {cam.move_right(-3. * dt);}
        if keys[Key::S as usize] {cam.move_front(-3. * dt);}
        if keys[Key::D as usize] {cam.move_right(3. * dt);}

        // Rendering
        unsafe {
            // Create matrices
            let mut model: glm::Mat4 = ident_mat4();
            let mut view: glm::Mat4 = cam.make_view(); 
            let projection: glm::Mat4 = glm::perspective((last_width as f32) / (last_height as f32), 0.7853982, 0.1, 100.0);

            // Assign matrices
            add_uniform(program, model, "model");
            add_uniform(program, view, "view");
            add_uniform(program, projection, "projection");

            // Background
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            // Draw objects
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
