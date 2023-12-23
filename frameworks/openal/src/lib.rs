//! This crate provides a wrapper around the underlying OpenAL implementation.
//!
//! The raw generated bindings can be found in the [`c`] module, or a more
//! hierarchical binding may be leveraged from the rest of this crate.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

use std::ffi::{c_char, CStr};

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

/// An error type that this library may raise.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Error {
  /// An error raised when invalid device name is provided.
  InvalidName,

  /// An error raised when an invalid enumeration value is provided to an API call.
  InvalidEnum,

  /// An error raised when an invalid value is provided to an API call.
  InvalidValue,

  /// An error raised when a requested operation is not valid.
  InvalidOperation,

  /// An error raised when out-of-memory conditions occur.
  OutOfMemory,

  /// An unknown error. This can only happen if the OpenAL version is out-of-date.
  Unknown(c::ALenum),
}

impl std::fmt::Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidName => "AL_INVALID_NAME",
      Self::InvalidEnum => "AL_INVALID_ENUM",
      Self::InvalidValue => "AL_INVALID_VALUE",
      Self::InvalidOperation => "AL_INVALID_OPERATION",
      Self::OutOfMemory => "AL_OUT_OF_MEMORY",
      Self::Unknown(_) => "AL_UNKNOWN",
    }
    .fmt(f)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidName => {
        "AL_INVALID_NAME: a bad name (ID) was passed to an OpenAL function".fmt(f)
      }
      Self::InvalidEnum => {
        "AL_INVALID_ENUM: an invalid enum value was passed to an OpenAL function".fmt(f)
      }
      Self::InvalidValue => {
        "AL_INVALID_VALUE: an invalid value was passed to an OpenAL function".fmt(f)
      }
      Self::InvalidOperation => "AL_INVALID_OPERATION: the requested operation is not valid".fmt(f),
      Self::OutOfMemory => {
        "AL_OUT_OF_MEMORY: the requested operation resulted in OpenAL running out of memory".fmt(f)
      }
      Self::Unknown(v) => write!(f, "Unknown OpenAL error code: {:x}", v),
    }
  }
}

impl Error {
  /// Creates an error from an [`ALenum`] value.
  ///
  /// # Arguments
  ///
  /// * `e` - the [`ALenum`] value representing an error status.
  ///
  /// [`ALenum`]: c::ALenum
  pub fn from_enum(e: c::ALenum) -> Option<Self> {
    if e as u32 == c::AL_NO_ERROR {
      None
    } else {
      // SAFETY: We check the safety condition immediately above.
      Some(unsafe { Self::from_enum_unchecked(e) })
    }
  }

  /// Creates this an [`Error`] object from an [`ALenum`] without checking
  /// whether it is [`AL_NO_ERROR`] first.
  ///
  /// # Arguments
  ///
  /// * `e` - the enum value representing an error.
  ///
  /// # Safety
  ///
  /// It is the responsibility of the caller to ensure that `e` is not
  /// [`AL_NO_ERROR`] -- otherwise this can return an error state representing
  /// no error.
  ///
  /// [`ALenum`]: c::ALenum
  /// [`AL_NO_ERROR`]: c::AL_NO_ERROR
  pub unsafe fn from_enum_unchecked(e: c::ALenum) -> Self {
    match e as u32 {
      c::AL_INVALID_VALUE => Self::InvalidValue,
      c::AL_INVALID_NAME => Self::InvalidName,
      c::AL_INVALID_ENUM => Self::InvalidEnum,
      c::AL_INVALID_OPERATION => Self::InvalidOperation,
      c::AL_OUT_OF_MEMORY => Self::OutOfMemory,
      _ => Self::Unknown(e),
    }
  }

  /// Checks the global state for the last error result.
  ///
  /// This effectively just calls [`alGetError`] and converts it directly into
  /// a [`Result`] object.
  ///
  /// [`alGetError`]: c::alGetError
  pub fn last_error() -> Result<()> {
    let underlying = unsafe { c::alGetError() };

    if underlying != c::AL_NO_ERROR as i32 {
      // SAFETY: We check that this is not AL_NO_ERROR above.
      Err(unsafe { Self::from_enum_unchecked(underlying) })
    } else {
      Ok(())
    }
  }
}

/// Checks the result of an OpenAL operation, returning the output as a [`Result`].
macro_rules! check_true {
  (c::$func:ident($($arg:expr$(,)?)+) -> $out:expr) => {
    if unsafe { $crate::c::$func($($arg,)+) } == $crate::c::ALC_TRUE as $crate::c::ALCboolean {
      Ok($out)
    } else if let Err(err) = $crate::Error::last_error() {
      Err(err)
    } else {
      Ok($out)
    }
  };
  ($device:ident.$func:ident($($arg:expr$(,)?)+) -> $out:expr) => {
    if unsafe { $crate::c::$func($($arg,)+) } == $crate::c::ALC_TRUE as $crate::c::ALCboolean {
      Ok($out)
    } else if let Err(err) = $device.last_error() {
      Err(err)
    } else {
      Ok($out)
    }
  };
}

macro_rules! check_not_null {
  (c::$func:ident($($arg:expr$(,)?)+) -> $out:expr) => {
    let ptr = unsafe { $crate::c::$func($($arg,)+) };
    if !ptr.is_null() {
      Ok($out(ptr))
    } else {
      Err(unsafe { Error::last_error().unwrap_err_unchecked() })
    }
  };
  ($device:ident.$func:ident($($arg:expr$(,)?)+) -> $out:expr) => {
    let ptr = !unsafe { $crate::c::$func($($arg,)+) };
    if ptr.is_null() {
      Ok($out(ptr))
    } else {
      Err(unsafe { $device.last_error().unwrap_err_unchecked() })
    }
  };
}

/// A specialized [`Result`] type for OpenAL operations.
///
/// This type is broadly used across the [`openal`] crate for any operation which
/// may produce an error.
///
/// [`openal`]: crate
/// [`Result`]: ::std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

/// Representation of an Audio device.
pub struct Device(*mut c::ALCdevice);

/// An iterator for viewing all available OpenAL device names.
pub struct AvailableDeviceIter<'a> {
  ptr: *const c_char,
  _phantom: std::marker::PhantomData<&'a c_char>,
}

impl<'a> Iterator for AvailableDeviceIter<'a> {
  type Item = &'a CStr;
  fn next(&mut self) -> Option<Self::Item> {
    // SAFETY: We check for null first before we dereference.
    if self.ptr.is_null() || unsafe { *self.ptr == '\0' as i8 } {
      None
    } else {
      // SAFETY: OpenAL Guarantees that available devices is returned as a
      // buffer of nul-terminated strings, with the last string itself being
      // nul-terminated twice.
      let next = unsafe {
        let v = CStr::from_ptr(self.ptr);
        self.ptr = self.ptr.add(v.to_bytes().len() + 1);
        v
      };

      Some(next)
    }
  }
}

impl Device {
  /// Acquires and returns context to the builtin/default device.
  pub fn open_default() -> Result<Self> {
    check_not_null! { c::alcOpenDevice(std::ptr::null_mut()) -> Device }
  }

  /// Opens a context to the named playback device.
  ///
  /// # Arguments
  ///
  /// * `name` - the name of the device
  pub fn open(name: &CStr) -> Result<Self> {
    check_not_null! { c::alcOpenDevice(name.as_ptr()) -> Device }
  }

  /// Closes this device
  pub fn close(&mut self) -> Result<()> {
    check_true! { c::alcCloseDevice(self.0) -> () }
  }

  /// Returns the last error that this device encountered, if one exists.
  pub fn last_error(&self) -> Result<()> {
    let underlying = unsafe { c::alcGetError(self.0) };

    if underlying != c::AL_NO_ERROR as i32 {
      // SAFETY: We check that this is not AL_NO_ERROR above.
      Err(unsafe { Error::from_enum_unchecked(underlying) })
    } else {
      Ok(())
    }
  }

  /// Returns an iterator that iterates over all available OpenAL device name
  /// strings.
  pub fn available_device_names() -> AvailableDeviceIter<'static> {
    let data = get_string(c::ALC_DEVICE_SPECIFIER as i32);
    AvailableDeviceIter {
      ptr: data.as_ptr(),
      _phantom: std::marker::PhantomData,
    }
  }

  /// Queries whether OpenAL supports a device-specific extension
  pub fn has_extension(&self, name: &CStr) -> bool {
    unsafe { c::alcIsExtensionPresent(self.0, name.as_ptr()) != (c::AL_FALSE as i8) }
  }
}

impl Drop for Device {
  fn drop(&mut self) {
    let _ = self.close();
  }
}

/// Attributes that can be provided to the [`Context`] creation process.
#[derive(Default)]
pub struct Attributes {
  frequency: Option<c::ALint>,
  refresh: Option<c::ALint>,
  sync: Option<c::ALint>,
  mono_sources: Option<c::ALint>,
  stereo_sources: Option<c::ALint>,
}

impl Attributes {
  /// Creates a new, default Context
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the frequency for mixing in the output buffer, measured in Hz
  ///
  /// # Arguments
  ///
  /// * `frequency` - the frequency in hz.
  pub fn frequency(mut self, frequency: i32) -> Self {
    self.frequency = Some(frequency as c::ALint);
    self
  }

  /// Sets the refresh intervals, measured in Hz
  ///
  /// # Arguments
  ///
  /// * `refresh` - the refresh interval in hz.
  pub fn refresh(mut self, refresh: i32) -> Self {
    self.refresh = Some(refresh as c::ALint);
    self
  }

  /// Sets whether it should be a sychronous or asynchronous context
  ///
  /// # Arguments
  ///
  /// * `sync` - whether this should be a synchronous context
  pub fn sync(mut self, sync: bool) -> Self {
    self.sync = Some(sync as c::ALint);
    self
  }

  /// Hints how many mono-sources will be used by the context.
  ///
  /// This doesn’t restict the number you can use, this just makes it more
  /// efficient if you know ahead of time.
  ///
  /// # Arguments
  ///
  /// * `count` - the number of mono-sources to hint
  pub fn mono_sources(mut self, count: i32) -> Self {
    self.mono_sources = Some(count as c::ALint);
    self
  }

  /// Hints how many stereo-sources will be used by the context.
  ///
  /// This doesn’t restict the number you can use, this just makes it more
  /// efficient if you know ahead of time.
  ///
  /// # Arguments
  ///
  /// * `count` - the number of stereo-sources to hint
  pub fn stereo_sources(mut self, count: i32) -> Self {
    self.stereo_sources = Some(count as c::ALint);
    self
  }
}

///
pub struct Context(*mut c::ALCcontext);

unsafe impl ::foundation::Transparent for Context {
  type Wrapped = *mut c::ALCcontext;
}

impl AsRef<*mut c::ALCcontext> for Context {
  fn as_ref(&self) -> &*mut c::ALCcontext {
    &self.0
  }
}

impl AsMut<*mut c::ALCcontext> for Context {
  fn as_mut(&mut self) -> &mut *mut c::ALCcontext {
    &mut self.0
  }
}

impl foundation::Take<*mut c::ALCcontext> for Context {
  unsafe fn take(self) -> *mut c::ALCcontext {
    let result = self.0;
    let _ = std::mem::ManuallyDrop::new(self);
    result
  }
}

impl std::ops::Deref for Context {
  type Target = *mut c::ALCcontext;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for Context {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Context {
  /// Creates a new AL Context object for the given device, using any
  /// specified attributes.
  pub fn new(device: &Device, attributes: Attributes) -> Result<Self> {
    let mut attribute_list: [c::ALint; 11] = [0; 11];
    let mut index = 0;
    if let Some(frequency) = attributes.frequency {
      attribute_list[index] = c::ALC_FREQUENCY as _;
      attribute_list[index + 1] = frequency;
      index += 2;
    }
    if let Some(refresh) = attributes.refresh {
      attribute_list[index] = c::ALC_REFRESH as _;
      attribute_list[index + 1] = refresh;
      index += 2;
    }
    if let Some(sync) = attributes.sync {
      attribute_list[index] = c::ALC_SYNC as _;
      attribute_list[index + 1] = sync;
      index += 2;
    }
    if let Some(sync) = attributes.sync {
      attribute_list[index] = c::ALC_SYNC as _;
      attribute_list[index + 1] = sync;
      index += 2;
    }
    if let Some(mono_sources) = attributes.mono_sources {
      attribute_list[index] = c::ALC_MONO_SOURCES as _;
      attribute_list[index + 1] = mono_sources;
      index += 2;
    }
    if let Some(stereo_sources) = attributes.stereo_sources {
      attribute_list[index] = c::ALC_STEREO_SOURCES as _;
      attribute_list[index + 1] = stereo_sources;
    }
    let context = unsafe { c::alcCreateContext(device.0, attribute_list.as_ptr()) };
    Error::last_error().map(|_| Context(context))
  }

  /// Makes this context current.
  pub fn make_current(&self) -> Result<()> {
    check_true! { c::alcMakeContextCurrent(self.0) -> () }
  }

  /// Destroys this OpenAL Context
  ///
  /// # Safety
  ///
  /// Calling this function will drop the context directly.
  pub fn destroy(self) {
    unsafe { c::alcDestroyContext(self.0) }
    let _ = std::mem::ManuallyDrop::new(self);
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { c::alcDestroyContext(self.0) }
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
