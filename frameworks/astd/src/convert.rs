//! The [`convert`] module provides operations and trait utilities for
//! conversions.
//!
//! [`convert`]: crate::convert

use std::convert::Infallible;

/// Simple and safe ownership acquisition of sub-elements of a type.
///
/// Any caller of this trait that acquires the underlying element must abide by
/// the underlying requirements of that type for resource destruction.
pub trait Take<T>
where
  T: Sized,
{
  /// Takes ownership of the underlying `T` element that is a sub-resource of
  /// this type.
  ///
  /// # Safety
  ///
  /// The caller of this function must ensure that they take responsibility of
  /// the underlying `T` element if ownership is safely acquired. This may
  /// require manually dropping or destroying the underlying resource in a safe
  /// way.
  unsafe fn take(self) -> T;
}

/// Simple and safe ownership acquisition of sub-elements of a type.
///
/// This generalizes the [`Take`] trait to enable failure conditions for the
/// case of any acquisition failures.
///
/// Any caller of this trait that successfully acquires the underlying element
/// must abide by the underlying requirements of that type for resource
/// destruction.
pub trait TryTake<T>
where
  T: Sized,
{
  /// The possible error type that might occur when trying to take ownership
  /// of the underlying `T`.
  type Error;

  /// Attempts to take ownership of the underlying `T` element from this object.
  ///
  /// # Safety
  ///
  /// The caller of this function must ensure that they take responsibility of
  /// the underlying `T` element if ownership is safely acquired. This may
  /// require manually dropping or destroying the underlying resource in a safe
  /// way.
  unsafe fn try_take(self) -> Result<T, Self::Error>;
}

impl<T, U> TryTake<T> for U
where
  U: Take<T>,
{
  type Error = Infallible;

  unsafe fn try_take(self) -> Result<T, Self::Error> {
    Ok(self.take())
  }
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
