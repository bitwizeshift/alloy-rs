use super::types::*;

pub unsafe fn glCreateShader(shader_type: GLenum) -> GLuint {
  call_glew_fn!(__glewCreateShader(shader_type))
}

pub unsafe fn glShaderSource(
  shader: GLuint,
  count: GLsizei,
  string: *const *const GLchar,
  length: *const GLint,
) {
  call_glew_fn!(__glewShaderSource(shader, count, string, length));
}

pub unsafe fn glCompileShader(shader: GLuint) {
  call_glew_fn!(__glewCompileShader(shader));
}

pub unsafe fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
  call_glew_fn!(__glewGetShaderiv(shader, pname, params));
}

pub unsafe fn glGetShaderInfoLog(
  shader: GLuint,
  buf_size: GLsizei,
  length: *mut GLsizei,
  info_log: *mut GLchar,
) {
  call_glew_fn!(__glewGetShaderInfoLog(shader, buf_size, length, info_log));
}

pub unsafe fn glDeleteShader(shader: GLuint) {
  call_glew_fn!(__glewDeleteShader(shader));
}
