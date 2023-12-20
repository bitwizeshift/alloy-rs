//! This alloc provides base definitions for allocator functionality.

mod allocation_callbacks;
mod allocator;
mod scope;

#[doc(inline)]
pub use allocation_callbacks::*;
#[doc(inline)]
pub use allocator::*;
#[doc(inline)]
pub use scope::*;
