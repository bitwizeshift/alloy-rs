//! This module exposes OpenGL bindings that are wrangled by the Glew library.

/// An alias of `GLenum`.
pub type Enum = glew_sys::GLenum;

/// An alias of `GLint`.
pub type Int = glew_sys::GLint;

/// An alias of `GLuint`.
pub type Sizei = glew_sys::GLsizei;

mod clear;

#[doc(inline)]
pub use clear::*;
