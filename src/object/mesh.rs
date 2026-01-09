use std::rc::Rc;

use gl;
use gl::types::GLuint;

#[derive(Debug)]
pub struct Mesh<'a> {
    pub vao: GLuint,
    vertex_data: &'a [f32],
    index_data: &'a [u32],
}

impl<'a> Mesh<'a> {
    pub unsafe fn new(vertex_data: &'a [f32], index_data: &'a [u32], stride: i32) -> Rc<Mesh<'a>> {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        // VAO
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // VBO
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(vertex_data) as isize,
            vertex_data.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // EBO
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(index_data) as isize,
            index_data.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // Vertex attributes
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);

        // Return mesh
        Rc::new(Mesh {
            vao,
            vertex_data,
            index_data,
        })
    }

    pub fn index_count(&self) -> usize {
        self.index_data.len()
    }
}
