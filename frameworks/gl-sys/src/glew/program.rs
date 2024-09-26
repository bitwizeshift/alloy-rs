use super::types::*;

#[inline(always)]
pub unsafe fn glCreateProgram() -> GLuint {
  call_glew_fn!(__glewCreateProgram())
}

#[inline(always)]
pub unsafe fn glAttachShader(program: GLuint, shader: GLuint) {
  call_glew_fn!(__glewAttachShader(program, shader));
}

#[inline(always)]
pub unsafe fn glLinkProgram(program: GLuint) {
  call_glew_fn!(__glewLinkProgram(program));
}

#[inline(always)]
pub unsafe fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
  call_glew_fn!(__glewGetProgramiv(program, pname, params));
}

#[inline(always)]
pub unsafe fn glGetProgramInfoLog(
  program: GLuint,
  buf_size: GLsizei,
  length: *mut GLsizei,
  info_log: *mut GLchar,
) {
  call_glew_fn!(__glewGetProgramInfoLog(program, buf_size, length, info_log));
}

#[inline(always)]
pub unsafe fn glUseProgram(program: GLuint) {
  call_glew_fn!(__glewUseProgram(program));
}

#[inline(always)]
pub unsafe fn glDeleteProgram(program: GLuint) {
  call_glew_fn!(__glewDeleteProgram(program));
}

#[inline(always)]
pub unsafe fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
  call_glew_fn!(__glewGetAttribLocation(program, name))
}

#[inline(always)]
pub unsafe fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
  call_glew_fn!(__glewGetUniformLocation(program, name))
}

#[inline(always)]
pub unsafe fn glUniform1f(location: GLint, v0: f32) {
  call_glew_fn!(__glewUniform1f(location, v0));
}

#[inline(always)]
pub unsafe fn glUniform2f(location: GLint, v0: f32, v1: f32) {
  call_glew_fn!(__glewUniform2f(location, v0, v1));
}

#[inline(always)]
pub unsafe fn glUniform3f(location: GLint, v0: f32, v1: f32, v2: f32) {
  call_glew_fn!(__glewUniform3f(location, v0, v1, v2));
}

#[inline(always)]
pub unsafe fn glUniform4f(location: GLint, v0: f32, v1: f32, v2: f32, v3: f32) {
  call_glew_fn!(__glewUniform4f(location, v0, v1, v2, v3));
}

#[inline(always)]
pub unsafe fn glUniform1i(location: GLint, v0: GLint) {
  call_glew_fn!(__glewUniform1i(location, v0));
}

#[inline(always)]
pub unsafe fn glUniform2i(location: GLint, v0: GLint, v1: GLint) {
  call_glew_fn!(__glewUniform2i(location, v0, v1));
}

#[inline(always)]
pub unsafe fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
  call_glew_fn!(__glewUniform3i(location, v0, v1, v2));
}

#[inline(always)]
pub unsafe fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
  call_glew_fn!(__glewUniform4i(location, v0, v1, v2, v3));
}

#[inline(always)]
pub unsafe fn glUniform1fv(location: GLint, count: GLsizei, value: *const f32) {
  call_glew_fn!(__glewUniform1fv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform2fv(location: GLint, count: GLsizei, value: *const f32) {
  call_glew_fn!(__glewUniform2fv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform3fv(location: GLint, count: GLsizei, value: *const f32) {
  call_glew_fn!(__glewUniform3fv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform4fv(location: GLint, count: GLsizei, value: *const f32) {
  call_glew_fn!(__glewUniform4fv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
  call_glew_fn!(__glewUniform1iv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
  call_glew_fn!(__glewUniform2iv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
  call_glew_fn!(__glewUniform3iv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
  call_glew_fn!(__glewUniform4iv(location, count, value));
}

#[inline(always)]
pub unsafe fn glUniformMatrix2fv(
  location: GLint,
  count: GLsizei,
  transpose: bool,
  value: *const f32,
) {
  call_glew_fn!(__glewUniformMatrix2fv(
    location,
    count,
    transpose as _,
    value
  ));
}

#[inline(always)]
pub unsafe fn glUniformMatrix3fv(
  location: GLint,
  count: GLsizei,
  transpose: bool,
  value: *const f32,
) {
  call_glew_fn!(__glewUniformMatrix3fv(
    location,
    count,
    transpose as _,
    value
  ));
}

#[inline(always)]
pub unsafe fn glUniformMatrix4fv(
  location: GLint,
  count: GLsizei,
  transpose: bool,
  value: *const f32,
) {
  call_glew_fn!(__glewUniformMatrix4fv(
    location,
    count,
    transpose as _,
    value
  ));
}

#[inline(always)]
pub unsafe fn glVertexAttrib1f(index: GLuint, x: f32) {
  call_glew_fn!(__glewVertexAttrib1f(index, x));
}

#[inline(always)]
pub unsafe fn glVertexAttrib2f(index: GLuint, x: f32, y: f32) {
  call_glew_fn!(__glewVertexAttrib2f(index, x, y));
}

#[inline(always)]
pub unsafe fn glVertexAttrib3f(index: GLuint, x: f32, y: f32, z: f32) {
  call_glew_fn!(__glewVertexAttrib3f(index, x, y, z));
}

#[inline(always)]
pub unsafe fn glVertexAttrib4f(index: GLuint, x: f32, y: f32, z: f32, w: f32) {
  call_glew_fn!(__glewVertexAttrib4f(index, x, y, z, w));
}

#[inline(always)]
pub unsafe fn glVertexAttrib1fv(index: GLuint, values: *const f32) {
  call_glew_fn!(__glewVertexAttrib1fv(index, values));
}

#[inline(always)]
pub unsafe fn glVertexAttrib2fv(index: GLuint, values: *const f32) {
  call_glew_fn!(__glewVertexAttrib2fv(index, values));
}

#[inline(always)]
pub unsafe fn glVertexAttrib3fv(index: GLuint, values: *const f32) {
  call_glew_fn!(__glewVertexAttrib3fv(index, values));
}

#[inline(always)]
pub unsafe fn glVertexAttrib4fv(index: GLuint, values: *const f32) {
  call_glew_fn!(__glewVertexAttrib4fv(index, values));
}

#[inline(always)]
pub unsafe fn glVertexAttribPointer(
  index: GLuint,
  size: GLint,
  type_: GLenum,
  normalized: bool,
  stride: GLsizei,
  pointer: *const std::ffi::c_void,
) {
  call_glew_fn!(__glewVertexAttribPointer(
    index,
    size,
    type_,
    normalized as _,
    stride,
    pointer
  ));
}

#[inline(always)]
pub unsafe fn glEnableVertexAttribArray(index: GLuint) {
  call_glew_fn!(__glewEnableVertexAttribArray(index));
}

#[inline(always)]
pub unsafe fn glDisableVertexAttribArray(index: GLuint) {
  call_glew_fn!(__glewDisableVertexAttribArray(index));
}
