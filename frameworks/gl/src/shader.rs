//!
use std::ffi::CStr;

use crate::error::Result;

/// The type of shader.
pub enum ShaderKind {
  /// Represents a vertex shader.
  Vertex = crate::c::GL_VERTEX_SHADER as _,
  /// Represents a fragment shader.
  Fragment = crate::c::GL_FRAGMENT_SHADER as _,
  /// Represents a geometry shader.
  Geometry = crate::c::GL_GEOMETRY_SHADER as _,
  /// Represents a compute shader.
  Compute = crate::c::GL_COMPUTE_SHADER as _,
}

/// A raw handle to the underlying Shader object.
pub struct Shader(crate::c::GLuint);

impl Shader {
  /// Creates a new shader handle.
  pub fn new(shader: ShaderKind) -> Self {
    let value = unsafe { crate::c::glCreateShader(shader as _) };
    Self(value)
  }

  /// Creates a new shader with full checking for errors.
  pub fn new_checked(shader: ShaderKind) -> Result<Self> {
    let value = unsafe { crate::c::glCreateShader(shader as _) };
    if value == 0 {
      match crate::error::Error::last_error() {
        Some(err) => Err(err),
        None => Ok(Self(value)),
      }
    } else {
      Ok(Self(value))
    }
  }

  /// Adds the specified shader source to the shader.
  /// This is just a thin wrapper around [`glShaderSource`]
  ///
  /// # Parameters
  ///
  /// * `source` - The source code as a null-terminated [`CStr`]
  ///
  /// [`glShaderSource`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
  pub fn shader_source_cstr(&mut self, source: &CStr) {
    unsafe {
      crate::c::glShaderSource(self.0, 1, &source.as_ptr(), std::ptr::null());
    }
  }

  /// Adds the specified shader source to the shader.
  /// This is just a thin wrapper around [`glShaderSource`]
  ///
  /// # Parameters
  ///
  /// * `source` - The source code of the shader
  ///
  /// [`glShaderSource`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
  pub fn shader_source(&mut self, source: &str) {
    let length = source.len() as _;
    let str_ptr = source.as_ptr() as *const std::ffi::c_char;
    unsafe { crate::c::glShaderSource(self.0, 1, &str_ptr, &length) };
  }

  /// Adds the specified shader sources to the shader.
  ///
  /// This function is optimized to handle up to 8 sources without requiring
  /// using dynamic memory allocation.
  ///
  /// # Parameters
  ///
  /// * `sources` - a collection of sources
  pub fn shader_sources(&mut self, sources: &[&str]) {
    match sources.len() {
      0 => (),
      1 => self.shader_source(sources[0]),
      2 => self.shader_sources_fixed::<2>(sources),
      3..=4 => self.shader_sources_fixed::<4>(sources),
      5..=8 => self.shader_sources_fixed::<8>(sources),
      _ => self.shader_sources_dynamic(sources),
    }
  }

  fn shader_sources_dynamic(&mut self, sources: &[&str]) {
    let lengths: Vec<_> = sources.iter().map(|s| s.len() as _).collect();
    let ptrs: Vec<_> = sources
      .iter()
      .map(|s| s.as_ptr() as *const std::ffi::c_char)
      .collect();
    unsafe {
      crate::c::glShaderSource(self.0, sources.len() as _, ptrs.as_ptr(), lengths.as_ptr());
    }
  }

  fn shader_sources_fixed<const N: usize>(&mut self, sources: &[&str]) {
    let mut ptrs: [*const std::ffi::c_char; N] = [std::ptr::null(); N];
    let mut sizes: [crate::c::GLsizei; N] = [0; N];
    for (i, source) in sources.iter().enumerate() {
      ptrs[i] = source.as_ptr() as *const std::ffi::c_char;
      sizes[i] = source.len() as _;
    }
    unsafe {
      crate::c::glShaderSource(self.0, N as _, ptrs.as_ptr(), sizes.as_ptr());
    }
  }

  /// Attempts to compile the shader.
  ///
  /// This must be called after adding shader source(s).
  pub fn compile(&mut self) -> Result<()> {
    unsafe { crate::c::glCompileShader(self.0) }
    if self.compile_status() {
      Ok(())
    } else {
      Err(crate::error::Error::ShaderError(self.info_log()))
    }
  }

  /// Gets the compile status for the shader.
  /// This is a thin wrapper around [`glGetShaderiv`]
  ///
  /// Returns `true` if the shader was compiled successfully.
  ///
  /// [`glGetShaderiv`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShader.xhtml
  #[inline]
  pub fn compile_status(&self) -> bool {
    let mut status = 0;
    unsafe {
      crate::c::glGetShaderiv(self.0, crate::c::GL_COMPILE_STATUS, &mut status);
    }
    status != 0
  }

  /// Checks if this shader is flagged for deletion.
  /// This is a thin wrapper around [`glGetShaderiv`]
  ///
  /// Returns `true` if the shader is flagged for deletion.
  ///
  /// [`glGetShaderiv`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShader.xhtml
  #[inline]
  pub fn delete_status(&self) -> bool {
    let mut status = 0;
    unsafe {
      crate::c::glGetShaderiv(self.0, crate::c::GL_DELETE_STATUS, &mut status);
    }
    status != 0
  }

  /// Gets the info log for the shader.
  pub fn info_log(&self) -> String {
    let mut length = 0;
    unsafe {
      crate::c::glGetShaderiv(self.0, crate::c::GL_INFO_LOG_LENGTH, &mut length);
    }

    let mut buffer = vec![0; length as usize];
    unsafe {
      crate::c::glGetShaderInfoLog(
        self.0,
        length,
        std::ptr::null_mut(),
        buffer.as_mut_ptr() as *mut _,
      );
    }

    String::from_utf8(buffer).unwrap()
  }

  /// Takes ownership of the underlying shader object. It becomes the
  /// responsibility of the caller to call [`glDeleteShader`] on the returned
  /// shader object.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the shader object is deleted after calling
  /// this function, otherwise there will be a resource-leak.
  ///
  /// [`glDeleteShader`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml
  #[inline]
  pub unsafe fn release(self) -> crate::c::GLuint {
    let value = self.0;
    std::mem::forget(self);
    value
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe { crate::c::glDeleteShader(self.0) }
  }
}

/// A handle to a uniform location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniformLocation(crate::c::GLint);

/// A handle to an attribute location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttribLocation(crate::c::GLint);

/// A self-disposing handle to a Program object.
pub struct Program(crate::c::GLuint);

impl Program {
  /// Creates a new program handle.
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let value = unsafe { crate::c::glCreateProgram() };
    Self(value)
  }

  /// Creates a new program with full checking for errors.
  pub fn new_checked() -> Result<Self> {
    let value = unsafe { crate::c::glCreateProgram() };
    if value == 0 {
      match crate::error::Error::last_error() {
        Some(err) => Err(err),
        None => Ok(Self(value)),
      }
    } else {
      Ok(Self(value))
    }
  }

  /// Attaches the specified shader to the program.
  /// This is a thin wrapper around [`glAttachShader`].
  ///
  /// # Parameters
  ///
  /// * `shader` - The shader to attach to the program
  ///
  /// [`glAttachShader`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml
  #[inline(always)]
  pub fn attach_shader(&mut self, shader: &Shader) {
    unsafe { crate::c::glAttachShader(self.0, shader.0) };
  }

  /// Links the program, returning whether the link was successful.
  pub fn link_program(&mut self) -> Result<()> {
    unsafe { crate::c::glLinkProgram(self.0) };
    if self.link_status() {
      Ok(())
    } else {
      Err(crate::error::Error::ShaderError(self.info_log()))
    }
  }

  /// Gets the link status for the program.
  /// This is a thin wrapper around [`glGetProgramiv`]
  ///
  /// Returns `true` if the program was linked successfully.
  ///
  /// [`glGetProgramiv`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgram.xhtml
  #[inline]
  pub fn link_status(&self) -> bool {
    let mut status = 0;
    unsafe {
      crate::c::glGetProgramiv(self.0, crate::c::GL_LINK_STATUS, &mut status);
    }
    status != 0
  }

  /// Gets the info log for the program.
  /// This is a thin wrapper around [`glGetProgramInfoLog`]
  ///
  /// [`glGetProgramInfoLog`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInfoLog.xhtml
  pub fn info_log(&self) -> String {
    let mut length = 0;
    unsafe {
      crate::c::glGetProgramiv(self.0, crate::c::GL_INFO_LOG_LENGTH, &mut length);
    }

    let mut buffer = vec![0; length as usize];
    unsafe {
      crate::c::glGetProgramInfoLog(
        self.0,
        length,
        std::ptr::null_mut(),
        buffer.as_mut_ptr() as *mut _,
      );
    }

    String::from_utf8(buffer).unwrap()
  }

  /// Takes ownership of the underlying program object. It becomes the
  /// responsibility of the caller to call [`glDeleteProgram`] on the returned
  /// program object.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the program object is deleted after calling
  /// this function, otherwise there will be a resource-leak.
  ///
  /// [`glDeleteProgram`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml
  #[inline]
  pub unsafe fn release(self) -> crate::c::GLuint {
    let value = self.0;
    std::mem::forget(self);
    value
  }

  /// Binds the program.
  /// This is a thin wrapper around [`glUseProgram`]
  ///
  /// [`glUseProgram`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml
  #[inline(always)]
  pub fn bind(&self) {
    unsafe { crate::c::glUseProgram(self.0) }
  }

  /// Unbinds the program.
  pub fn reset_binding() {
    unsafe { crate::c::glUseProgram(0) }
  }

  /// Gets the location of the specified uniform.
  pub fn uniform_location(&self, name: &str) -> UniformLocation {
    let name = std::ffi::CString::new(name).unwrap();
    self.uniform_location_cstr(&name)
  }

  /// Gets the location of the specified uniform.
  pub fn uniform_location_cstr(&self, name: &CStr) -> UniformLocation {
    let uniform = unsafe { crate::c::glGetUniformLocation(self.0, name.as_ptr()) };
    UniformLocation(uniform)
  }

  /// Gets the location of the specified attribute.
  pub fn attribute_location(&self, name: &str) -> AttribLocation {
    let name = std::ffi::CString::new(name).unwrap();
    self.attribute_location_cstr(&name)
  }

  /// Gets the location of the specified attribute.
  pub fn attribute_location_cstr(&self, name: &CStr) -> AttribLocation {
    let attrib = unsafe { crate::c::glGetAttribLocation(self.0, name.as_ptr()) };
    AttribLocation(attrib)
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe { crate::c::glDeleteProgram(self.0) }
  }
}
