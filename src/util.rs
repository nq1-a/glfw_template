use std::ffi::CString;

use nalgebra_glm as glm;

pub fn cstr_from(slice: &str) -> CString {
    return CString::new(slice)
        .map_err(|_| format!("no cstring for {}", slice))
        .unwrap();
}

pub fn ident_mat4() -> glm::Mat4 {
    return glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
}
