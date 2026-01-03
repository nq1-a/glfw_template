use std::rc::Rc;

use gl;
use gl::types::GLuint;

use nalgebra_glm as glm;

use crate::Vertex;

#[derive(Debug)]
pub struct Mesh<'a> {
    pub vao: GLuint,
    vertex_data: &'a [Vertex],
    pub program: GLuint
}

impl<'a> Mesh<'a> {
    pub unsafe fn new(program: GLuint, vertex_data: &'a [Vertex]) -> Rc<Mesh<'a>> {
        let (mut vao, mut vbo) = (0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(vertex_data) as isize,
            vertex_data.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);

        Rc::new(Mesh {
            vao,
            vertex_data,
            program
        })
    }

    pub fn data_size(&self) -> usize {
        self.vertex_data.len()
    }
}
