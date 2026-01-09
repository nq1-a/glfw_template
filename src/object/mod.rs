use gl::types::GLuint;

pub mod context;
pub mod mesh;
pub mod model;

pub trait Render {
    unsafe fn render(&self, shader: GLuint);
}
