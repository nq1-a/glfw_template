pub mod mesh;
pub mod model;
pub mod registry;

pub type Vertex = [f32; 3];

pub trait Render {
    unsafe fn render(&self);
}
