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
    unsafe fn render(&self) {
        gl::BindVertexArray(self.mesh.vao);
        add_uniform(self.mesh.program, self.cframe, "model");

        gl::DrawArrays(
            gl::TRIANGLES,
            0,
            self.mesh.data_size() as i32
        );

        gl::BindVertexArray(0);
    }
} 
