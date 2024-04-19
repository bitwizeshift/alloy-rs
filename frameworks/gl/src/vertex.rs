//!

use crate::c::GLuint;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Target {
  ArrayBuffer = crate::c::GL_ARRAY_BUFFER as _,
  ElementArrayBuffer = crate::c::GL_ELEMENT_ARRAY_BUFFER as _,
  UniformBuffer = crate::c::GL_UNIFORM_BUFFER as _,
  PixelPackBuffer = crate::c::GL_PIXEL_PACK_BUFFER as _,
  PixelUnpackBuffer = crate::c::GL_PIXEL_UNPACK_BUFFER as _,
  CopyReadBuffer = crate::c::GL_COPY_READ_BUFFER as _,
  CopyWriteBuffer = crate::c::GL_COPY_WRITE_BUFFER as _,
  TransformFeedbackBuffer = crate::c::GL_TRANSFORM_FEEDBACK_BUFFER as _,
  ShaderStorageBuffer = crate::c::GL_SHADER_STORAGE_BUFFER as _,
  DispatchIndirectBuffer = crate::c::GL_DISPATCH_INDIRECT_BUFFER as _,
  DrawIndirectBuffer = crate::c::GL_DRAW_INDIRECT_BUFFER as _,
  QueryBuffer = crate::c::GL_QUERY_BUFFER as _,
  TextureBuffer = crate::c::GL_TEXTURE_BUFFER as _,
  AtomicCounterBuffer = crate::c::GL_ATOMIC_COUNTER_BUFFER as _,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Usage {
  StreamDraw = crate::c::GL_STREAM_DRAW as _,
  StreamRead = crate::c::GL_STREAM_READ as _,
  StreamCopy = crate::c::GL_STREAM_COPY as _,
  StaticDraw = crate::c::GL_STATIC_DRAW as _,
  StaticRead = crate::c::GL_STATIC_READ as _,
  StaticCopy = crate::c::GL_STATIC_COPY as _,
  DynamicDraw = crate::c::GL_DYNAMIC_DRAW as _,
  DynamicRead = crate::c::GL_DYNAMIC_READ as _,
  DynamicCopy = crate::c::GL_DYNAMIC_COPY as _,
}

pub enum Access {
  Read = crate::c::GL_READ_ONLY as _,
  Write = crate::c::GL_WRITE_ONLY as _,
  ReadWrite = crate::c::GL_READ_WRITE as _,
}

///
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BufferObject {
  id: GLuint,
  target: Target,
  usage: Usage,
  size: usize,
}

impl BufferObject {
  /// Generate a new buffer object.
  pub fn new<T>(target: Target, usage: Usage, buffer: &[T]) -> Self {
    let mut out = 0;
    unsafe { crate::c::glGenBuffers(1, &mut out) };
    debug_assert_ne!(out, 0);

    let size = std::mem::size_of_val(buffer);
    let ptr = buffer.as_ptr() as *const std::ffi::c_void;
    unsafe { crate::c::glBufferData(target as _, size as isize, ptr, usage as _) }

    Self {
      id: out,
      target,
      usage,
      size,
    }
  }

  #[inline(always)]
  pub fn bind(&self) {
    self.bind_as_target(self.target);
  }

  /// Bind the buffer object.
  #[inline]
  pub fn bind_as_target(&self, target: Target) {
    unsafe { crate::c::glBindBuffers(target as _, self.id) };
  }

  pub fn data<T>(&self, buffer: &[T]) {
    let size = std::mem::size_of_val(buffer);
    let ptr = buffer.as_ptr() as *const std::ffi::c_void;
    unsafe { crate::c::glBufferData(self.target as _, size as isize, ptr, self.usage as _) }
  }

  /// Set the data of the buffer object.
  pub fn sub_data<T>(&self, offset: usize, buffer: &[T]) {
    let size = std::mem::size_of_val(buffer);
    let ptr = buffer.as_ptr() as *const std::ffi::c_void;
    unsafe { crate::c::glBufferSubData(self.target as _, offset as _, size as _, ptr) };
  }

  /// Map the buffer object.
  pub fn map(&mut self, access: Access) -> *mut std::ffi::c_void {
    unsafe { crate::c::glMapBuffer(self.target as _, access as _) }
  }

  /// Unmap the buffer object.
  pub fn unmap(&self, target: GLuint) -> bool {
    unsafe { crate::c::glUnmapBuffer(target) == 1 }
  }
}

// impl Drop for BufferObject {
//   fn drop(&mut self) {
//     call_glew_fn!(__glewDeleteBuffers(1, &self.0));
//   }
// }

///
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct VertexArray(GLuint);

impl Drop for VertexArray {
  fn drop(&mut self) {
    unsafe { crate::c::glDeleteVertexArrays(1, &self.0) };
  }
}

impl VertexArray {
  /// Generate a new vertex array.
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let mut out = 0;
    unsafe { crate::c::glGenVertexArrays(1, &mut out) };
    Self(out)
  }

  /// Bind the vertex array.
  pub fn bind(&self) {
    unsafe { crate::c::glBindVertexArray(self.0) };
  }

  /// Unbind the vertex array.
  pub fn unbind() {
    unsafe { crate::c::glBindVertexArray(0) };
  }
}
