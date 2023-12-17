//! This crate provides a wrapper around the underlying Vulkan implementation.
//!
//! The raw generated bindings can be found in the [`c`] module, or a more
//! hierarchical binding may be leveraged from the rest of this crate.
#![deny(missing_docs)]
// #![deny(unused_results)]
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
  pub use vulkan_sys::*;
}

/// Counts the number of extension properties available.
pub fn count_extension_properties() -> usize {
  let mut count: u32 = 0;

  // SAFETY: This is only unsafe since it's a C call
  let r = unsafe {
    c::vkEnumerateInstanceExtensionProperties(
      std::ptr::null(),
      &mut count as *mut u32,
      std::ptr::null_mut(),
    );
  };
  let _ = r;
  count as usize
}
