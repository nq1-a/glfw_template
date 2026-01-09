use std::rc::Rc;

use nalgebra_glm as glm;

use crate::{Mesh, object::Render};

use crate::render::add_uniform;
use crate::util::ident_mat4;

#[derive(Debug)]
pub struct Model<'a> {
    cframe: glm::Mat4,
    pub mesh: Rc<Mesh<'a>>,
}

impl Model<'_> {
    pub fn new(position: glm::Vec3, mesh: Rc<Mesh>) -> Model {
        let mut model: Model = Model {
            cframe: ident_mat4(),
            mesh
        };

        model.cframe = glm::translate(&model.cframe, &position);
        model
    }
}

impl Render for Model<'_> {
    unsafe fn render(&self, shader: gl::types::GLuint) {
        add_uniform(shader, self.cframe, "model");

        gl::BindVertexArray(self.mesh.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.mesh.index_count() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }
} 
