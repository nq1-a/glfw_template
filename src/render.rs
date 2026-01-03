use std::ffi::CString;

use gl;

use gl::types::{
    GLenum,
    GLuint,
};

use nalgebra_glm::Mat4;

use crate::util::cstr_from;

pub unsafe fn build_shader(program: GLuint, shader: &str, stype: GLenum) -> GLuint {
    let glshader = gl::CreateShader(stype);
    
    gl::ShaderSource(
        glshader,
        1,
        &(shader.as_bytes().as_ptr().cast()),
        &(shader.len().try_into().unwrap()),
    );

    gl::CompileShader(glshader);
    gl::AttachShader(program, glshader);
    gl::DeleteShader(glshader);

    glshader
}

pub unsafe fn add_uniform(program: GLuint, mat: Mat4, mtype: &str) {
    let mat_type: CString = cstr_from(mtype);
    let mat_loc = gl::GetUniformLocation(program, mat_type.as_ptr());
    assert_ne!(mat_loc, -1);
    gl::UniformMatrix4fv(mat_loc, 1, gl::FALSE, mat.as_ptr());
}
