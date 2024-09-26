use super::types::*;

#[inline(always)]
pub unsafe fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
  call_glew_fn!(__glewGenRenderbuffers(n, renderbuffers));
}

#[inline(always)]
pub unsafe fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
  call_glew_fn!(__glewBindRenderbuffer(target, renderbuffer));
}

#[inline(always)]
pub unsafe fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
  call_glew_fn!(__glewDeleteRenderbuffers(n, renderbuffers));
}

#[inline(always)]
pub unsafe fn glRenderbufferStorage(
  target: GLenum,
  internal_format: GLenum,
  width: GLsizei,
  height: GLsizei,
) {
  call_glew_fn!(__glewRenderbufferStorage(
    target,
    internal_format,
    width,
    height
  ));
}
