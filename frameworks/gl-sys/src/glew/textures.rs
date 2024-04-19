use super::types::*;

pub use glew_sys::glBindTexture as bind_texture;
pub use glew_sys::glDeleteTextures as delete_textures;
pub use glew_sys::glGenTextures as gen_textures;
pub use glew_sys::glTexImage1D as tex_image_1d;
pub use glew_sys::glTexImage2D as tex_image_2d;
pub use glew_sys::glTexParameterf as tex_parameter_f;
pub use glew_sys::glTexParameteri as tex_parameter_i;
pub use glew_sys::glTexSubImage2D as tex_sub_image_2d;

pub unsafe fn glActiveTexture(texture: GLenum) {
  call_glew_fn!(__glewActiveTexture(texture));
}
