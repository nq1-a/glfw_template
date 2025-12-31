use gl;
use nalgebra_glm as glm;

use glfw::{
    Action,
    Context,
    fail_on_errors,
    Key,
    WindowEvent,
    WindowMode,
};

mod shader;

use crate::shader::build_shader;

type Vertex = [f32; 3];

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Make window
    let (mut window, events) = glfw.create_window(
        400,
        400,
        "hello world",
        WindowMode::Windowed
    ).unwrap();

    window.make_current();
    window.set_key_polling(true);

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

    // Initialize shaders
    const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;

        uniform mat4 transform;

        void main() {
            gl_Position = transform * vec4(pos.x, pos.y, pos.z, 1.0);
        }
    "#;

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;

        void main() {
            final_color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    unsafe {
        // Create program
        let program = gl::CreateProgram();

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

    // Create data for rendering
    const triangle: [Vertex; 36] = [
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
            size_of_val(&triangle) as isize,
            triangle.as_ptr().cast(),
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

    // Main loop
    let last_width: i32 = 0;
    let last_height: i32 = 0;

    while !window.should_close() {
        // Fix window size
        unsafe {
            let (width, height) = window.get_size();
            if last_width != width || last_height != height {
                gl::Viewport(0, 0, width, height);
            }
        }

        // Events
        for (_, ev) in glfw::flush_messages(&events) {
            match ev {
                WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    window.set_should_close(true)
                }

                _ => {}
            }
        }

        // Rendering
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
