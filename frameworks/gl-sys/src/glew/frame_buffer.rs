use super::types::*;

pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
  call_glew_fn!(__glewGenFramebuffers(n, framebuffers));
}

pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
  call_glew_fn!(__glewBindFramebuffer(target, framebuffer));
}

pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
  call_glew_fn!(__glewDeleteFramebuffers(n, framebuffers));
}

pub unsafe fn glFramebufferTexture2D(
  target: GLenum,
  attachment: GLenum,
  textarget: GLenum,
  texture: GLuint,
  level: GLint,
) {
  call_glew_fn!(__glewFramebufferTexture2D(
    target, attachment, textarget, texture, level
  ));
}

pub unsafe fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
  call_glew_fn!(__glewCheckFramebufferStatus(target))
}

pub unsafe fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) {
  call_glew_fn!(__glewDrawBuffers(n, bufs));
}

pub use glew_sys::glReadBuffer;

pub unsafe fn glFramebufferRenderbuffer(
  target: GLenum,
  attachment: GLenum,
  renderbuffertarget: GLenum,
  renderbuffer: GLuint,
) {
  call_glew_fn!(__glewFramebufferRenderbuffer(
    target,
    attachment,
    renderbuffertarget,
    renderbuffer
  ));
}
