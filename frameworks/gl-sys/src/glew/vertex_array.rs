use super::types::*;

pub unsafe fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
  call_glew_fn!(__glewGenVertexArrays(n, arrays));
}

pub unsafe fn glBindVertexArray(array: GLuint) {
  call_glew_fn!(__glewBindVertexArray(array));
}

pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
  call_glew_fn!(__glewDeleteVertexArrays(n, arrays));
}
