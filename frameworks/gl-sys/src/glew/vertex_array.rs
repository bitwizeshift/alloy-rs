use super::types::*;

#[inline(always)]
pub unsafe fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
  call_glew_fn!(__glewGenVertexArrays(n, arrays));
}

#[inline(always)]
pub unsafe fn glBindVertexArray(array: GLuint) {
  call_glew_fn!(__glewBindVertexArray(array));
}

#[inline(always)]
pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
  call_glew_fn!(__glewDeleteVertexArrays(n, arrays));
}

#[inline(always)]
pub unsafe fn glBindBuffer(target: GLenum, buffer: GLuint) {
  call_glew_fn!(__glewBindBuffer(target, buffer));
}
