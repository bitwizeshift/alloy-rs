//! The core module provides base definitions that are used throughout the engine.

mod align;
mod aligned_array;
mod clamped;
mod ieee754;
mod slice;

pub mod hash;
pub mod hint;

#[doc(inline)]
pub use align::*;

#[doc(inline)]
pub use ieee754::*;

#[doc(inline)]
pub use aligned_array::*;

#[doc(inline)]
pub use slice::*;

#[doc(inline)]
pub use clamped::*;

#[doc(inline)]
pub use astd::Uuid;
