//! This crate provides a statically-linked definition of the GLFW library
//! from source.
//!
//! The generated bindings are
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

pub mod c {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  #![allow(unused)]
  #![allow(missing_docs)]
  #![allow(unused_results)]
  #![allow(rust_2018_idioms)]
  #![allow(rustdoc::broken_intra_doc_links)]
  #[doc(inline)]
  pub use glfw_sys::*;
}

/// An error that may be returned from a GLFW operation.
///
/// These correspond 1:1 with
#[derive(Debug)]
pub enum Error {
  /// An error returned when GLFW has not been initialized.
  NotInitialized,

  /// An error returned when the current context is incorrect.
  NoCurrentContext,

  /// An error returned when an enumerated value is out of the range of valid
  /// inputs.
  InvalidEnum,

  /// An error returned when a value is outsode fh te range of valid inputs.
  InvalidValue,

  /// An error returned if GLFW runs out of memory.
  OutOfMemory,

  /// An error returned if an API has been disabled or is unavailable
  ApiUnavailable,

  /// An error returned if an API is available, but a specific version is
  /// unavailable (such as the OpenGL version)
  VersionUnavailable,

  /// A generic platform-specific error has occurred.
  PlatformError,

  /// A specific format is unavailable.
  FormatUnavailable,

  /// An error that occurs when no window context is available.
  NoWindowContext,

  /// An error outside of this enumeration has occurred.
  /// If this shows up, it means GLFW either didn't document an error-code, or
  /// it has added a new error code in an update.
  Unknown(core::ffi::c_int),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::NotInitialized => "",
      Error::NoCurrentContext => "",
      Error::InvalidEnum => "",
      Error::InvalidValue => "",
      Error::OutOfMemory => "",
      Error::ApiUnavailable => "",
      Error::VersionUnavailable => "",
      Error::PlatformError => "",
      Error::FormatUnavailable => "",
      Error::NoWindowContext => "",
      Error::Unknown(_) => "",
    }
    .fmt(f)
  }
}

impl std::error::Error for Error {}

impl Error {
  /// Constructs this Error from a GLFW status value.
  ///
  /// # Arguments
  ///
  /// * `code` - the GLFW status code
  pub fn from_glfw_status(code: core::ffi::c_int) -> Self {
    match code as u32 {
      c::GLFW_NOT_INITIALIZED => Self::NotInitialized,
      c::GLFW_NO_CURRENT_CONTEXT => Self::NoCurrentContext,
      c::GLFW_INVALID_ENUM => Self::InvalidEnum,
      c::GLFW_INVALID_VALUE => Self::InvalidValue,
      c::GLFW_OUT_OF_MEMORY => Self::OutOfMemory,
      c::GLFW_API_UNAVAILABLE => Self::ApiUnavailable,
      c::GLFW_VERSION_UNAVAILABLE => Self::VersionUnavailable,
      c::GLFW_PLATFORM_ERROR => Self::PlatformError,
      c::GLFW_FORMAT_UNAVAILABLE => Self::FormatUnavailable,
      c::GLFW_NO_WINDOW_CONTEXT => Self::NoWindowContext,
      _ => Self::Unknown(code),
    }
  }
}

/// A specialized [`Result`] type for GLFW operations.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

/// An instance of the GLFW context, of which there should only ever be at most
/// one of.
pub struct Context;

impl Context {
  /// Constructs a new [`Context`] object and returns the instance.
  pub fn new() -> Result<Self> {
    if unsafe { c::glfwInit() } == (c::GLFW_TRUE as core::ffi::c_int) {
      Ok(Context)
    } else {
      Err(Error::from_glfw_status(unsafe {
        c::glfwGetError(std::ptr::null_mut())
      }))
    }
  }

  /// Polls the [`Context`] for events.
  ///
  /// This calls [`glfwPollEvents`] implicitly
  ///
  /// [`glfwPollEvents`]: https://www.glfw.org/docs/3.3/group__window.html#ga37bd57223967b4211d60ca1a0bf3c832
  pub fn poll_events(&self) {
    unsafe { c::glfwPollEvents() }
  }

  /// Queries whether this context supports the Vulkan API.
  pub fn vulkan_supported(&self) -> bool {
    unsafe { c::glfwVulkanSupported() != 0 }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { c::glfwTerminate() };
  }
}

/// A wrapper type around [`GLFWwindow`].
///
/// [`GLFWwindow`]: c::GLFWwindow
pub struct Window(*mut c::GLFWwindow);

impl Window {
  /// Constructs this [`Window`] from an underlying [`c::GLFWwindow`] instance.
  ///
  /// This will panic if `window` is `null`.
  #[inline]
  pub fn from_c(window: *mut c::GLFWwindow) -> Self {
    debug_assert!(window != std::ptr::null_mut());
    Self(window)
  }

  /// Queries whether this Window should be closed.
  #[inline]
  pub fn should_close(&self) -> bool {
    unsafe { c::glfwWindowShouldClose(self.0) != 0 }
  }

  /// Gets a constant pointer to the underlying [`c::GLFWwindow`].
  pub fn ptr(&self) -> *const c::GLFWwindow {
    self.0
  }

  /// Gets a mutable pointer to the underlying [`c::GLFWwindow`].
  pub fn ptr_mut(&self) -> *mut c::GLFWwindow {
    self.0
  }

  /// Shows this window.
  #[inline]
  pub fn show(&self) {
    unsafe { c::glfwShowWindow(self.0) }
  }

  /// Swaps the underlying buffers of this window.
  #[inline]
  pub fn swap_buffers(&self) {
    unsafe { c::glfwSwapBuffers(self.0) }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe { c::glfwDestroyWindow(self.0) }
  }
}
