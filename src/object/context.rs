use gl;
use gl::types::GLuint;
use nalgebra_glm as glm;

use crate::camera::Camera;
use crate::object::Render;
use crate::render::{add_uniform, build_shader};

pub struct RenderContext {
    pub objects: Vec<Box<dyn Render>>,
    pub camera: Camera,
    pub shader: GLuint,
    pub frame_h: i32,
    pub frame_w: i32,
    vert_shader: GLuint,
    frag_shader: GLuint,
}

impl RenderContext {
    pub fn new(w: i32, h: i32) -> RenderContext {
        let mut cam = Camera::new(
            glm::vec3(0., 0., 10.),
            -1.570796, 0.
        );

        RenderContext {
            objects: Vec::new(),
            camera: cam,
            shader: 0,
            frame_h: h,
            frame_w: w,
            vert_shader: 0,
            frag_shader: 0,
        }
    }

    pub fn add(&mut self, renderable: Box<dyn Render>) {
        self.objects.push(renderable);
    }

    pub unsafe fn create_program(&mut self, vert: &str, frag: &str) -> GLuint {
        let program: GLuint = gl::CreateProgram();
        self.vert_shader = build_shader(program, vert, gl::VERTEX_SHADER);
        self.frag_shader = build_shader(program, frag, gl::FRAGMENT_SHADER);
        gl::LinkProgram(program);
        gl::UseProgram(program);
       
        // Finalize
        self.shader = program;
        program
    }

    pub unsafe fn draw(&self) {
        let view: glm::Mat4 = self.camera.make_view(); 
        let projection: glm::Mat4 = glm::perspective((self.frame_w as f32) / (self.frame_h as f32), 0.7853982, 0.1, 100.0);

        add_uniform(self.shader, view, "view");
        add_uniform(self.shader, projection, "projection");

        for obj in &self.objects {
            obj.render(self.shader);
        }
    }
}
