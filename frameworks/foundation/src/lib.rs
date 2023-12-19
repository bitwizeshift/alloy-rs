//! This crate provides core definitions that need to be shared between the
//! various wrapper utilities.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

use std::ffi::{c_char, CStr, CString};

/// Creates a compile-time [`CStr`] object from a string literal.
///
/// This will statically verify the invariant that the [`CStr`] does not contain
/// any embedded nul character, and will also append the trailing nul terminator.
///
/// # Examples
///
/// ```rust
/// # use foundation::cstr;
///
/// let cstr = cstr!("Hello world!");
/// ```
///
/// [`CStr`]: core::ffi::CStr
#[macro_export]
macro_rules! cstr {
  ($e:expr) => {{
    use core::ffi::CStr;

    const S: &'static str = concat!($e, "\0");
    const _IS_VALID_C_STR: [(); $e.len()] = [(); $crate::_null_terminated_length(S)];

    // SAFETY: `from_bytes_with_nul_unchecked` asserts at compile-time that there
    // cannot be any embedded nuls. This guarantees that safety is upheld, despite
    // being an unsafe API.
    const RESULT: &'static CStr = unsafe { &CStr::from_bytes_with_nul_unchecked(S.as_bytes()) };

    RESULT
  }};
}

/// Constructs a [`CString`] from a non null-terminated [`str`] reference.
///
/// # Arguments
///
/// * `string` - a standar rust string.
pub fn cstring_from_str(string: &str) -> CString {
  let mut bytes = string.as_bytes().to_owned();
  bytes.push(0);
  unsafe { CString::from_vec_unchecked(bytes) }
}

/// Constructs a [`CStr`] reference from a [`c_char`] slice which contains a
/// nul-terminator.
///
/// # Arguments
///
/// * `slice` - a slice of [`c_char`] that ends with a nul-terminator.
///
/// # Safety
///
/// This function requires that `slice` be nul-terminated.
#[inline]
pub const unsafe fn cstr_from_char_slice(slice: &[c_char]) -> &CStr {
  // SAFETY: c_char (i8) and u8 are interconvertible
  let s = unsafe { &*(slice as *const _ as *const [u8]) };

  CStr::from_bytes_with_nul_unchecked(s)
}

/// Computes the null-terminated string length of a C-string.
///
/// This function exists since the current FFI helpers are not `const` functions,
/// and this enables checking for the value at compile-time.
///
/// This is an implementation-detail of [`c_str!`], which simply delegates the
/// call to [`_null_terminated_length_b`].
///
/// # Arguments
///
/// * `s` - the string slice
#[doc(hidden)]
pub const fn _null_terminated_length(s: &str) -> usize {
  _null_terminated_length_b(s.as_bytes(), 0, 0)
}

/// Computes the null-terminated string length of a C-string.
///
/// This counts each independent non-null byte, and will stop executing when it
/// exceeds
#[doc(hidden)]
pub const fn _null_terminated_length_b(bytes: &[u8], index: usize, count: usize) -> usize {
  if bytes[index] == 0 {
    count
  } else {
    _null_terminated_length_b(bytes, index + 1, count + 1)
  }
}

/// Prepares a suitably aligned pointer for elements of type `T` to write output
/// to, which will construct a vector from it.
///
/// # Arguments
///
/// * `size` - the number of elements to create storage for
/// * `f` - the function to populate the storage with data.
///
/// # Safety
///
/// This function has the same safety contract as `f`; if `f` is safe for writing
/// only `size` entries in the created pointer, then this function will be safe.
pub unsafe fn read_transparent_out_vec<T, F>(size: usize, f: F) -> Vec<T>
where
  T: Transparent,
  T::Wrapped: Sized,
  F: Fn(*mut T::Wrapped) -> *mut T::Wrapped,
{
  let layout = std::alloc::Layout::array::<T>(size).expect("overflow cannot happen");

  // SAFETY: The layout specification is generated from the `array`
  // constructor function; this can only be suitably aligned.
  let mem = unsafe { std::alloc::alloc(layout) };

  let out = (f)(mem.cast::<T::Wrapped>());

  Vec::from_raw_parts(out as *mut T, size, size)
}

/// Prepares a suitably aligned pointer for elements of type `T` to write output
/// to, which will construct a vector from it.
///
/// # Arguments
///
/// * `size` - the number of elements to create storage for
/// * `f` - the function to populate the storage with data.
///
/// # Safety
///
/// This function has the same safety contract as `f`; if `f` is safe for writing
/// only `size` entries in the created pointer, then this function will be safe.
pub unsafe fn read_out_vec<T, F>(size: usize, f: F) -> Vec<T>
where
  F: Fn(*mut T) -> *mut T,
{
  let layout = std::alloc::Layout::array::<T>(size).expect("overflow cannot happen");

  // SAFETY: The layout specification is generated from the `array`
  // constructor function; this can only be suitably aligned.
  let mem = unsafe { std::alloc::alloc(layout) };

  let out = (f)(mem.cast::<T>());

  Vec::from_raw_parts(out, size, size)
}

/// Transparent is a trait that indicates that the implementor is a transparent
/// wrapper around some type, and should be treated interoperably with wrapped
/// type.
///
/// This allows for internal types to be treated as references of the wrapper
/// type, and vice-versa.
///
/// # Note
///
/// This trait only allows for conversions between _references_ of objects;
/// this means that the formed references will become ephemeral references of a
/// different type -- but the lifetime of the object will still be tied to the
/// lifetime of the source.
///
/// # Acknowledgements
///
/// The approach to use this trait is heavily inspired by the [`bytemuck`] crate,
/// which is licensed under ZLib OR MIT OR Apache-2. The idea has been adapted
/// for this library's specific needs.
///
/// # Safety
///
/// The safety contract for this type requires two things:
///
/// * The wrapper type implementing this must be `#[repr(transparent)]`
/// * The implementation must not override the function definitions in the trait.
///
/// [`bytemuck`]: https://crates.io/crates/bytemuck
pub unsafe trait Transparent {
  /// The type that is wrapped by the Transparent impl.
  type Wrapped: ?Sized;

  /// Converts a reference to a type into a reference of the wrapper type.
  ///
  /// # Safety
  ///
  /// The safety contract for this type requires two things:
  ///
  /// * The wrapper type implementing this must be `#[repr(transparent)]`
  /// * This function must not be implemented in `impl`s.
  #[inline]
  fn from_ref(wrapped: &Self::Wrapped) -> &Self {
    debug_assert!(
      std::mem::size_of::<*const Self::Wrapped>() == std::mem::size_of::<*const Self>()
    );
    unsafe {
      let ptr = wrapped as *const Self::Wrapped;
      let wrapper: *const Self = std::mem::transmute_copy(&ptr);
      &*wrapper
    }
  }

  /// Converts a mut reference to a wrapped type into a mut reference to the
  /// wrapper.
  ///
  /// # Safety
  ///
  /// The safety contract for this type requires two things:
  ///
  /// * The wrapper type implementing this must be `#[repr(transparent)]`
  /// * This function must not be implemented in `impl`s.
  #[inline]
  fn from_ref_mut(wrapped: &mut Self::Wrapped) -> &mut Self {
    debug_assert!(std::mem::size_of::<*mut Self::Wrapped>() == std::mem::size_of::<*mut Self>());
    unsafe {
      let ptr = wrapped as *mut Self::Wrapped;
      let wrapper: *mut Self = std::mem::transmute_copy(&ptr);
      &mut *wrapper
    }
  }

  /// Converts this wrapper into an instance of the underlying wrapped type.
  ///
  /// # Safety
  ///
  /// The safety contract for this type requires two things:
  ///
  /// * The wrapper type implementing this must be `#[repr(transparent)]`
  /// * This function must not be implemented in `impl`s.
  #[inline]
  fn as_ref(&self) -> &Self::Wrapped {
    debug_assert!(
      std::mem::size_of::<*const Self::Wrapped>() == std::mem::size_of::<*const Self>()
    );
    unsafe {
      let wrapped: *const Self::Wrapped = std::mem::transmute_copy(&self);
      &*wrapped
    }
  }

  /// Converts this wrapper into an instance of the underlying wrapped type.
  ///
  /// # Safety
  ///
  /// The safety contract for this type requires two things:
  ///
  /// * The wrapper type implementing this must be `#[repr(transparent)]`
  /// * This function must not be implemented in `impl`s.
  #[inline]
  fn as_ref_mut(&mut self) -> &mut Self::Wrapped {
    debug_assert!(std::mem::size_of::<*mut Self::Wrapped>() == std::mem::size_of::<*mut Self>());
    unsafe {
      let wrapped: *mut Self::Wrapped = std::mem::transmute_copy(&self);
      &mut *wrapped
    }
  }
}

/// Slices which are [`Transparent`] are convertible to their wrapped types.
unsafe impl<T> Transparent for [T]
where
  T: Transparent,
  T::Wrapped: Sized,
{
  type Wrapped = [T::Wrapped];
}

/// Arrays which are [`Transparent`] are convertible to their wrapped types.
unsafe impl<const N: usize, T> Transparent for [T; N]
where
  T: Transparent,
  T::Wrapped: Sized,
{
  type Wrapped = [T::Wrapped; N];
}

unsafe impl<T> Transparent for *const T
where
  T: Transparent,
{
  type Wrapped = *const T::Wrapped;
}

unsafe impl<T> Transparent for *mut T
where
  T: Transparent,
{
  type Wrapped = *mut T::Wrapped;
}

unsafe impl<T> Transparent for Vec<T>
where
  T: Transparent,
  T::Wrapped: Sized,
{
  type Wrapped = Vec<T::Wrapped>;
}

unsafe impl<T> Transparent for Box<T>
where
  T: Transparent,
{
  type Wrapped = Box<T::Wrapped>;
}

unsafe impl<T> Transparent for std::sync::Arc<T>
where
  T: Transparent,
{
  type Wrapped = std::sync::Arc<T::Wrapped>;
}

unsafe impl<T> Transparent for std::rc::Rc<T>
where
  T: Transparent,
{
  type Wrapped = std::rc::Rc<T::Wrapped>;
}

#[doc(inline)]
pub use uuid::Uuid;

unsafe impl Transparent for Uuid {
  type Wrapped = uuid::Bytes;
}

/// The location of source code at a point in the codebase.
///
/// This should generally be constructed by using the [`current_location`] macro.
///
/// # Example
///
/// Basic Use:
///
/// ```rust
/// #[doc(hidden)]
/// pub fn _inner_log(message: &str, location: foundation::SourceLocation) {
///   println!("{}: {}", location, message)
/// }
///
/// macro_rules! log {
///   ($($arg:tt)*) => {
/// #   use $crate::*;
///     _inner_log($($arg)*, foundation::current_location!())
///   };
/// }
///
/// // ...
///
/// # fn test_log() {
/// // Implicitly captures source-location at the same time
/// log!("logging something with source location...");
/// # }
/// ```
#[derive(Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct SourceLocation {
  /// The path to the file
  pub file: &'static str,
  /// The line within the file
  pub line: u32,
  /// The column
  pub column: u32,
}

impl std::fmt::Display for SourceLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.file, self.line, self.column)
  }
}

impl std::fmt::Debug for SourceLocation {
  #[inline(always)]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Self as std::fmt::Display>::fmt(self, f)
  }
}

/// A trait that provides a mechanism for taking ownership of the internal
/// underlying API. This is used in C APIs to allow direct access to the wrapped
/// type, at which point it is the caller's responsibility to handle the
/// lifetime of the object.
pub trait Take<T: ?Sized> {
  /// Adopts ownership of the internal object. The caller is now responsible
  /// for managing the underlying type's lifetime.
  ///
  /// # Safety
  ///
  /// This function is unsafe, as it becomes the caller's responsibility to
  /// manage the lifetime of the returned object.
  unsafe fn take(self) -> T;
}

/// Creates a [`SourceLocation`] object that represents the exact point in the
/// source file where this macro is invoked.
///
/// This can be used to mechanically capture the location of events in source
/// code for the purposes of debugging.
#[macro_export]
macro_rules! current_location {
  () => {
    $crate::SourceLocation {
      file: file!(),
      line: line!(),
      column: column!(),
    }
  };
}

/// A simple macro that ignores all of its contents and does nothing.
///
/// # Use
///
/// This macro exists to enable expansion of repeat or optional rule groups
/// that may not explicitly make use of that argument. For example, sometimes
/// the conditional inclusion of a token might want to generate something not
/// using that meta-variable -- but expansion requires at least one meta-variable
/// from the group to be specified to not be ambiguous. With this macro, you can
/// still "use" the meta-variable without actually actively using it.
///
/// # Example
///
/// ```
/// # use foundation::phantom_fragment;
/// macro_rules! generate{
///   ($A:ident, $($B:ident)?) => {
///     $(
///       phantom_fragment!($B);
///       impl $A {
///         // When $B is present, define this function without using $B.
///         fn do_something(&self) { /* ... */ }
///       }
///     )?
///   }
/// }
/// ```
#[macro_export]
macro_rules! phantom_fragment {
  ($($_:tt)*) => {};
  ($_:block) => {};
  ($_:expr) => {};
  ($_:ident) => {};
  ($_:item) => {};
  ($_:lifetime) => {};
  ($_:literal) => {};
  ($_:meta) => {};
  ($_:pat) => {};
  ($_:pat_param) => {};
  ($_:path) => {};
  ($_:stmt) => {};
  ($_:ty) => {};
  ($_:vis) => {};
}
