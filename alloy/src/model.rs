//! The model crate provides functionality for loading and manipulating 3D models.

mod color;

#[doc(inline)]
pub use color::*;
pub mod clip;
pub mod obj;
pub mod projection;
pub mod transform;
