//! This module provides a grouping for all math operations and types.

pub mod simd;

mod angle;
mod quaternion;

pub use angle::*;

pub mod mat;
pub mod vec;

#[doc(inline)]
pub use quaternion::*;

mod euler;

#[doc(inline)]
pub use euler::*;
