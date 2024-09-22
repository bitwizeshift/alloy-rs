//! This module provides a grouping for all math operations and types.

pub mod mat;
pub mod simd;
pub mod vec;

mod angle;
mod euler;
mod interpolate;
mod quaternion;

#[doc(inline)]
pub use angle::*;

#[doc(inline)]
pub use interpolate::*;

#[doc(inline)]
pub use quaternion::*;

#[doc(inline)]
pub use euler::*;
