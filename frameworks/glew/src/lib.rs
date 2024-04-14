//! This crate provides a wrapper around the underlying imgui implementation.
//!
//! The raw generated bindings can be found in the [`c`] module, or a more
//! hierarchical binding may be leveraged from the rest of this crate.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

/// This module provides all that raw definitions of the C Vulkan library.
pub mod c {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  #![allow(unused)]
  #![allow(missing_docs)]
  #![allow(unused_results)]
  #![allow(rust_2018_idioms)]
  #![allow(clippy::all)]
  #[doc(inline)]
  pub use glew_sys::*;
}

pub mod gl;

///
#[derive(Debug)]
pub struct Error(String);

impl Error {
  /// Creates a new error from a GLEW error code.
  pub fn from_glew_status(code: std::ffi::c_uint) -> Self {
    use std::ffi::CStr;
    let raw_message = unsafe { glew_sys::glewGetErrorString(code) as *const i8 };
    let message = unsafe { CStr::from_ptr(raw_message) }.to_string_lossy();

    Self(format!("{} (error code {})", message, code))
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl std::error::Error for Error {}

/// The instance of the GLEW library.
/// There should only ever be one instance of this.
pub struct Instance;

impl Instance {
  /// Initializes the GLEW library.
  pub fn new() -> Result<Self, Error> {
    unsafe { glew_sys::glewExperimental = glew_sys::GL_TRUE as _ };
    let result = unsafe { glew_sys::glewInit() };
    if result != glew_sys::GLEW_OK {
      panic!("Failed to initialize GLEW: {:?}", result);
    }
    Ok(Self)
  }
}
