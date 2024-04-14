//! The error module provides definitions of OpenGL errors, and other utilities
//! around operating with them.

/// An error that may be returned from an OpenGL operation.
///
/// This is normally retrieved by calling [`glGetError`], which is mapped to
/// [`Error::last_error`].
///
/// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Error {
  InvalidEnum,
  InvalidValue,
  InvalidOperation,
  StackOverflow,
  StackUnderflow,
  OutOfMemory,
  InvalidFramebufferOperation,
}

impl Error {
  /// Returns the last error that occurred by calling [`glGetError`].
  ///
  /// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
  #[inline]
  pub fn last_error() -> Option<Self> {
    let error = unsafe { crate::c::glGetError() };
    if error == crate::c::GL_NO_ERROR {
      None
    } else {
      Some(unsafe { Self::from_gl_status_unchecked(error) })
    }
  }

  /// Creates a new error from a GLEW error code.
  ///
  /// # Arguments
  ///
  /// * `code` - The error code to create the error from
  pub fn from_gl_status(code: std::ffi::c_uint) -> Option<Self> {
    match code {
      crate::c::GL_INVALID_ENUM => Some(Self::InvalidEnum),
      crate::c::GL_INVALID_VALUE => Some(Self::InvalidValue),
      crate::c::GL_INVALID_OPERATION => Some(Self::InvalidOperation),
      crate::c::GL_STACK_OVERFLOW => Some(Self::StackOverflow),
      crate::c::GL_STACK_UNDERFLOW => Some(Self::StackUnderflow),
      crate::c::GL_OUT_OF_MEMORY => Some(Self::OutOfMemory),
      crate::c::GL_INVALID_FRAMEBUFFER_OPERATION => Some(Self::InvalidFramebufferOperation),
      _ => None,
    }
  }

  /// Creates a new error from a GLEW error code without checking
  ///
  /// # Arguments
  ///
  /// * `code` - The error code to create the error from
  ///
  /// # Safety
  ///
  /// The error code must be one of the valid error codes.
  pub unsafe fn from_gl_status_unchecked(code: std::ffi::c_uint) -> Self {
    match code {
      crate::c::GL_INVALID_ENUM => Self::InvalidEnum,
      crate::c::GL_INVALID_VALUE => Self::InvalidValue,
      crate::c::GL_INVALID_OPERATION => Self::InvalidOperation,
      crate::c::GL_STACK_OVERFLOW => Self::StackOverflow,
      crate::c::GL_STACK_UNDERFLOW => Self::StackUnderflow,
      crate::c::GL_OUT_OF_MEMORY => Self::OutOfMemory,
      crate::c::GL_INVALID_FRAMEBUFFER_OPERATION => Self::InvalidFramebufferOperation,
      _ => unreachable!(),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidEnum => "Invalid Enum",
      Self::InvalidValue => "Invalid Value",
      Self::InvalidOperation => "Invalid Operation",
      Self::StackOverflow => "Stack Overflow",
      Self::StackUnderflow => "Stack Underflow",
      Self::OutOfMemory => "Out of Memory",
      Self::InvalidFramebufferOperation => "Invalid Framebuffer Operation",
    }
    .fmt(f)
  }
}

impl std::error::Error for Error {}

/// A specialized [`Result`] type for OpenGL operations.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

/// A utility function for calling an OpenGL function and checking for errors
/// using [`glGetError`].
///
/// # Arguments
///
/// * `f` - The function to call
///
/// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
pub fn check<F: Fn() -> T, T>(f: F) -> Result<T> {
  let result = f();
  let error = unsafe { crate::c::glGetError() };
  if error == crate::c::GL_NO_ERROR {
    Ok(result)
  } else {
    Err(unsafe { Error::from_gl_status_unchecked(error) })
  }
}
