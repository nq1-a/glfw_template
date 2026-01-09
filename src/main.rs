use std::env;
use std::rc::Rc;

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
mod object;
mod render;
mod util;

use crate::{
    object::{
        mesh::Mesh,
        model::Model,
        context::RenderContext,
        Render,
    },
    util::ident_mat4,
};

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
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CCW);
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    // Create context
    let mut context: RenderContext = RenderContext::new(1280, 720);

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

    unsafe {context.create_program(VERT_SHADER, FRAG_SHADER);}

    // Load meshes
    const cube_vertex_data: [f32; 8 * 3] = [
        -0.5, -0.5, 0.5,
        0.5, -0.5, 0.5,
        0.5, 0.5, 0.5,
        -0.5, 0.5, 0.5,
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, 0.5, -0.5,
        -0.5, 0.5, -0.5,
    ];

    const cube_index_data: [u32; 36] = [
        0, 2, 3, 0, 1, 2,
        0, 5, 1, 0, 4, 5,
        5, 2, 1, 5, 6, 2,
        4, 7, 6, 4, 6, 5,
        3, 2, 6, 3, 6, 7,
        4, 3, 7, 4, 0, 3,
    ];

    let cube_mesh: Rc<Mesh> = unsafe {
        Mesh::new(&cube_vertex_data, &cube_index_data, 12)
    };

    // Create objects
    unsafe {
        for i in 1..11 {
            for j in 1..11 {
                for k in 1..11 {
                    context.add(Box::new(Model::new(
                        glm::vec3(i as f32 * 1.1, j as f32 * 1.1, k as f32 * 1.1),
                        cube_mesh.clone()
                    )));
                }
            }
        }
    }

    // Initialization of variables
    let args: Vec<String> = env::args().collect();
    let delay: f32 = 1. / (args.get(1)
        .unwrap_or(&String::from("120."))
        .parse::<f32>()
        .unwrap_or(120.)
    + 0.5);
    
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

        // println!("{}", (1./dt) as i32);

        // Fix window size
        unsafe {
            let (width, height) = window.get_size();
            let last_width: i32 = context.frame_w;
            let last_height: i32 = context.frame_h;

            if last_width != width
            || last_height != height {
                gl::Viewport(0, 0, width, height);
                context.frame_w = width;
                context.frame_h = height;
                println!("{}x{}", last_width, last_height);
                println!("  {}x{}", width, height);
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
                    context.camera.rotate(
                        0.4 * dt * (x - mouse_x) as f32,
                        0.4 * dt * (mouse_y - y) as f32
                    );

                    mouse_x = x;
                    mouse_y = y;
                }

                _ => {}
            }
        }

        // Movement
        if keys[Key::W as usize] {context.camera.move_front(3. * dt);}
        if keys[Key::A as usize] {context.camera.move_right(-3. * dt);}
        if keys[Key::S as usize] {context.camera.move_front(-3. * dt);}
        if keys[Key::D as usize] {context.camera.move_right(3. * dt);}

        // Rendering
        unsafe {
            // Background
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            // Draw objects
            context.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
        last_time = current_time;
    }
}
