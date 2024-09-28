//! Module containing [Euclidean vector] definitions.
//!
//! Vectors can come in 2, 3, or 4-component forms, with a base of [`f32`],
//! [`i32`], or [`u32`], and in both owning and non-owning variations.
//! A non-owning variation allows for "views" of slices of primitive types,
//! and enable a common type that can be inherited with [`Deref`], or accessed
//! via [`Borrow`] or [`AsRef`].
//!
//! Owning-variations are named with a full `VectorN`, whereas non-owned
//! types are defined as `VecN`, where `N` is the number of components. Below is
//! a complete table of all the defined types for this module:
//!
//! | Base Type | Owning       | Non-Owning |
//! |-----------|--------------|------------|
//! | [`f32`]   | [`Vector2`]  | [`Vec2`]   |
//! | [`i32`]   | [`Vector2i`] | [`Vec2i`]  |
//! | [`u32`]   | [`Vector2u`] | [`Vec2u`]  |
//! | [`f32`]   | [`Vector3`]  | [`Vec3`]   |
//! | [`i32`]   | [`Vector3i`] | [`Vec3i`]  |
//! | [`u32`]   | [`Vector3u`] | [`Vec3u`]  |
//! | [`f32`]   | [`Vector4`]  | [`Vec4`]   |
//! | [`i32`]   | [`Vector4i`] | [`Vec4i`]  |
//! | [`u32`]   | [`Vector4u`] | [`Vec4u`]  |
//!
//! # SIMD Operations
//!
//! [SIMD] support is an incubating feature, but will be provided as 4x
//! operations of vectors at a time. For example, a `dot4` implementation can
//! take 8 vectors at once to produce 4 simultaneous dot-products.
//!
//! **Note:** It's common for engines to miss-characterize 4-component vectors
//! as needing to be SIMD-ified, since 4-components aligns with a `float32x4`
//! data-structure mental-model; however this is frequently an incorrect
//! interpretation of single-core parallelism concepts, and often pessimizes
//! systems by forcing additional unnecessary instructions.
//!
//! Parallelizing the data and the algorithms can be benchmarked to yield far
//! better returns. The goal of this incubating feature is to model the latter
//! by enabling `Vector4x4` and `Vector4` types that fit with standard SIMD
//! intent.
//!
//! [SIMD]: https://en.wikipedia.org/wiki/SIMD
//! [Euclidean vector]: https://en.wikipedia.org/wiki/Euclidean_vector
//! [`Borrow`]: std::borrow::Borrow
//! [`Deref`]: std::ops::Deref
mod vec2;
mod vec2i;
mod vec2u;

pub(crate) mod errors;

#[doc(inline)]
pub use errors::VecError;

#[doc(inline)]
pub use vec2::*;
#[doc(inline)]
pub use vec2i::*;
#[doc(inline)]
pub use vec2u::*;

mod vec3;
mod vec3i;
mod vec3u;

#[doc(inline)]
pub use vec3::*;
#[doc(inline)]
pub use vec3i::*;
#[doc(inline)]
pub use vec3u::*;

mod vec4;
mod vec4i;
mod vec4u;

#[doc(inline)]
pub use vec4::*;
#[doc(inline)]
pub use vec4i::*;
#[doc(inline)]
pub use vec4u::*;
