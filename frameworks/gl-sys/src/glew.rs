//! GLEW bindings for Rust.
//!
//! All functions are either direct references, or very lightweight wrappers
//! around C functions.

#![allow(clippy::missing_safety_doc)]

/// A small macro that enables calling GLEW-loaded functions.
///
/// In GLEW, it creates `__`-prefixed functions with the same name as the GL
/// functions, but using `glew` instead of `gl`. This is because GLEW loads
/// function-pointers to each of these function as they are available.
///
/// This macro is used to call these functions in a cleaner way, but still
/// unfortunately requires some massaging of the `__glew*` -> `gl*` names.
macro_rules! call_glew_fn {
  ($i:ident($($arg:expr),*)) => {
    {
      let pfn = unsafe { (::glew_sys::$i).unwrap_unchecked() };
      unsafe { pfn($($arg),*) }
    }
  }
}

pub use glew_sys::GL_COLOR_BUFFER_BIT;
pub use glew_sys::GL_DEPTH_BUFFER_BIT;
pub use glew_sys::GL_STENCIL_BUFFER_BIT;

pub use glew_sys::glClear;
pub use glew_sys::glClearAccum;
pub use glew_sys::glClearColor;
pub use glew_sys::glClearDepth;
pub use glew_sys::glClearStencil;

pub use glew_sys::glDisable;
pub use glew_sys::glEnable;
pub use glew_sys::glGetBooleanv;
pub use glew_sys::glGetDoublev;
pub use glew_sys::glGetError;
pub use glew_sys::glGetFloatv;
pub use glew_sys::glGetIntegerv;
pub use glew_sys::glGetString;

#[inline(always)]
pub unsafe fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
  call_glew_fn!(__glewGetStringi(name, index))
}

pub use glew_sys::glIsEnabled;

// Shaders

mod buffers;
mod consts;
mod consts_exts;
mod cull;
mod drawing;
mod frame_buffer;
mod program;
mod render_buffer;
mod shader;
mod textures;
mod types;
mod vertex_array;

pub use buffers::*;
pub use consts::*;
pub use consts_exts::*;
pub use cull::*;
pub use drawing::*;
pub use frame_buffer::*;
pub use program::*;
pub use render_buffer::*;
pub use shader::*;
pub use textures::*;
pub use types::*;
pub use vertex_array::*;
