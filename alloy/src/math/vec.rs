//! Module containing vector definitions.
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
//! [`Borrow`]: std::borrow::Borrow
//! [`Deref`]: std::ops::Deref
mod vec2;
mod vec2i;
mod vec2u;

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

impl From<&'_ Vec2i> for Vector2 {
  #[inline]
  fn from(value: &'_ Vec2i) -> Self {
    Self {
      x: value.x() as f32,
      y: value.y() as f32,
    }
  }
}

impl From<&'_ Vec2u> for Vector2 {
  #[inline]
  fn from(value: &'_ Vec2u) -> Self {
    Self {
      x: value.x() as f32,
      y: value.y() as f32,
    }
  }
}

impl From<&'_ Vec3i> for Vector3 {
  #[inline]
  fn from(value: &'_ Vec3i) -> Self {
    Self {
      x: value.x() as f32,
      y: value.y() as f32,
      z: value.z() as f32,
    }
  }
}

impl From<&'_ Vec3u> for Vector3 {
  #[inline]
  fn from(value: &'_ Vec3u) -> Self {
    Self {
      x: value.x() as f32,
      y: value.y() as f32,
      z: value.z() as f32,
    }
  }
}

impl From<&'_ Vec4i> for Vector4 {
  #[inline]
  fn from(value: &'_ Vec4i) -> Self {
    Self {
      x: value.x() as f32,
      y: value.y() as f32,
      z: value.z() as f32,
      w: value.w() as f32,
    }
  }
}

impl From<&'_ Vec4u> for Vector4 {
  #[inline]
  fn from(value: &'_ Vec4u) -> Self {
    Self {
      x: value.x() as f32,
      y: value.y() as f32,
      z: value.z() as f32,
      w: value.w() as f32,
    }
  }
}
