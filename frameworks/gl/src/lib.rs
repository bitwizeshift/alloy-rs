//! This crate exposes OpenGL bindings in a more idiomatic Rust format.
//!
//! OpenGL has an idiosynchratic way of testing for errors, often making use of
//! utilities like [`glGetError`] to get if the last call has failed. To account
//! for this, this library contains 3 different conventions for function naming.
//!
//! * `*_unchecked`: `unsafe` functions that _should_ test for an error in most
//!   cases, but are left untested to avoid the overhead for when the caller
//!   already knows an error-condition has been met.
//! * `*` (unprefixed): safe functions that either test for errors implicitly,
//!   or don't bother need to test at all since the functionality is reasonably
//!   safe.
//! * `*_checked`: safe functions that _don't need_ to test for errors, but are
//!   still _technically_ fallible and could be checked. Often this is useful
//!   for detecting failures after cases like a [`glGet*`] -- which is normally
//!   reasonably safe to assume will succeed, but may fail from a bad context
//!   profile.
//!
//! [`glGet*`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGet.xhtml
//! [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml

/// This module provides all that raw definitions of the C OpenGL library.
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
  pub use gl_sys::*;
}

mod capability;
mod clear;
mod cull;
pub mod error;
pub mod info;
pub mod shader;
pub mod vertex;

#[doc(inline)]
pub use capability::*;
#[doc(inline)]
pub use clear::*;
#[doc(inline)]
pub use cull::*;
