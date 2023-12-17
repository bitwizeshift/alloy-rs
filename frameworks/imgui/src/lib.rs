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
  #![allow(rustdoc::broken_intra_doc_links)]
  #![allow(clippy::all)]
  #[doc(inline)]
  pub use imgui_sys::*;
}

/// The context is the default setup for Imgui.
pub struct Context(*mut c::ImGuiContext);

impl Context {
  /// Constructs an ImGui context
  pub fn new() -> Self {
    unsafe { Self(c::igCreateContext(std::ptr::null_mut())) }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { c::igDestroyContext(self.0) }
  }
}
