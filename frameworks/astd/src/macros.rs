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
/// # use astd::phantom_fragment;
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

/// Defines the input types as being transparent wrappers around another type,
/// which enables conversions between reference types.
#[macro_export]
macro_rules! transparent {
  ($($Outter:ident.$Ex:tt => $Inner:ty$(,)?)+) => {
    $(
      unsafe impl $crate::convert::Transparent for $Outter {
        type Wrapped = $Inner;
      }

      impl AsRef<$Inner> for $Outter {
        fn as_ref(&self) -> &$Inner {
          &self.$Ex
        }
      }

      impl AsMut<$Inner> for $Outter {
        fn as_mut(&mut self) -> &mut $Inner {
          &mut self.$Ex
        }
      }

      impl ::astd::convert::Take<$Inner> for $Outter {
        unsafe fn take(self) -> $Inner {
          self.$Ex
        }
      }

      impl std::ops::Deref for $Outter {
        type Target = $Inner;
        fn deref(&self) -> &Self::Target {
          &self.$Ex
        }
      }

      impl std::ops::DerefMut for $Outter {
        fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.$Ex
        }
      }
    )+
  };
}
