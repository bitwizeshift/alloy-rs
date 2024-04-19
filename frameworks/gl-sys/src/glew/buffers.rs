use super::types::*;

#[inline(always)]
pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
  call_glew_fn!(__glewGenBuffers(n, buffers));
}

#[inline(always)]
pub unsafe fn glBindBuffers(target: GLenum, buffer: GLuint) {
  call_glew_fn!(__glewBindBuffer(target, buffer));
}

#[inline(always)]
pub unsafe fn glBufferData(
  target: GLenum,
  size: GLsizeiptr,
  data: *const std::ffi::c_void,
  usage: GLenum,
) {
  call_glew_fn!(__glewBufferData(target, size, data, usage));
}

#[inline(always)]
pub unsafe fn glBufferSubData(
  target: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  data: *const std::ffi::c_void,
) {
  call_glew_fn!(__glewBufferSubData(target, offset, size, data));
}

#[inline(always)]
pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
  call_glew_fn!(__glewDeleteBuffers(n, buffers));
}

#[inline(always)]
pub unsafe fn glMapBuffer(target: GLenum, access: GLenum) -> *mut std::ffi::c_void {
  call_glew_fn!(__glewMapBuffer(target, access))
}

#[inline(always)]
pub unsafe fn glUnmapBuffer(target: GLenum) -> GLboolean {
  call_glew_fn!(__glewUnmapBuffer(target))
}

#[inline(always)]
pub unsafe fn glGetBufferSubData(
  target: GLenum,
  offset: GLintptr,
  size: GLsizeiptr,
  data: *mut std::ffi::c_void,
) {
  call_glew_fn!(__glewGetBufferSubData(target, offset, size, data));
}

#[inline(always)]
pub unsafe fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
  call_glew_fn!(__glewGetBufferParameteriv(target, pname, params));
}

#[inline(always)]
pub unsafe fn glGetBufferPointerv(
  target: GLenum,
  pname: GLenum,
  params: *mut *mut std::ffi::c_void,
) {
  call_glew_fn!(__glewGetBufferPointerv(target, pname, params));
}

#[inline(always)]
pub unsafe fn glIsBuffer(buffer: GLuint) -> GLboolean {
  call_glew_fn!(__glewIsBuffer(buffer))
}

#[inline(always)]
pub unsafe fn glFlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) {
  call_glew_fn!(__glewFlushMappedBufferRange(target, offset, length));
}

#[inline(always)]
pub unsafe fn glMapBufferRange(
  target: GLenum,
  offset: GLintptr,
  length: GLsizeiptr,
  access: GLuint,
) -> *mut std::ffi::c_void {
  call_glew_fn!(__glewMapBufferRange(target, offset, length, access))
}
