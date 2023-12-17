//! This crate provides a wrapper around the underlying OpenAL implementation.
//!
//! The raw generated bindings can be found in the [`c`] module, or a more
//! hierarchical binding may be leveraged from the rest of this crate.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

use std::ffi::CStr;

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
  pub use openal_sys::*;
}

/// Representation of an Audio device.
pub struct Device(*mut c::ALCdevice);

impl Device {
  /// Acquires and returns context to the builtin/default device.
  pub fn open_default() -> Self {
    let result = unsafe { c::alcOpenDevice(std::ptr::null_mut()) };

    Device(result)
  }

  /// Opens a device with the given name.
  pub fn open(_name: &CStr) -> Self {
    todo!()
  }

  /// Closes this device
  pub fn close(&mut self) {
    let _ = unsafe { c::alcCloseDevice(self.0) };
  }

  /// Queries whether OpenAL supports a device-specific extension
  pub fn has_extension(&self, name: &CStr) -> bool {
    unsafe { c::alcIsExtensionPresent(self.0, name.as_ptr()) != (c::AL_FALSE as i8) }
  }
}

impl Drop for Device {
  fn drop(&mut self) {
    self.close()
  }
}

/// Queries whether OpenAL supports a global extension
pub fn has_extension(name: &CStr) -> bool {
  // SAFETY:
  //   This function is only "unsafe" because it is a C function, but this is
  //   called correctly as per the AL public API.
  unsafe { c::alcIsExtensionPresent(std::ptr::null_mut(), name.as_ptr()) != (c::AL_FALSE as i8) }
}

/// Gets a string for a named constant
pub fn get_string(constant: i32) -> &'static CStr {
  // SAFETY:
  //   The safety here is delegated to `c::alcGetString`, which guarantees that
  //   a string is returned and not null.
  unsafe { CStr::from_ptr(c::alcGetString(std::ptr::null_mut(), constant)) }
}
