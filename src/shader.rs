#![allow(unsafe_op_in_unsafe_fn)]

use gl;

use gl::types::{
    GLenum,
    GLuint,
};

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

    return glshader;
}
