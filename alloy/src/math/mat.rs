//! Module containing [matrix] definitions.
//!
//! Matrices can come in 2x2, 3x3, or 4x4 forms, with a base of [`f32`], and in
//! both owning and non-owning variations.
//!
//! Owning-variations are named with a full `MatrixN`, whereas non-owned
//! types are defined as `MatN`, where `N` is the number of rows and columns.
//!
//! Below is a complete table of all the defined types for this module:
//!
//! | Owning       | Non-Owning |
//! |--------------|------------|
//! | [`Matrix2`]  | [`Mat2`]   |
//! | [`Matrix3`]  | [`Mat3`]   |
//! | [`Matrix4`]  | [`Mat4`]   |
//!
//! [matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
//! [`f32`]: https://doc.rust-lang.org/std/primitive.f32.html
mod mat4;

pub(crate) mod col4;

#[doc(inline)]
pub use mat4::*;
