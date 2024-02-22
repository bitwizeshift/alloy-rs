use std::str::ParseBoolError;

use crate::c;

/// A 0-cost abstraction wrapper around the underlying [`VkBool32`].
///
/// This type exists primarily to provide better
///
/// [`VkBool32`]: c::VkBool32
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bool32(c::VkBool32);

impl Bool32 {
  /// An easy constant for representing a true [`Bool32`] instance.
  pub const TRUE: Self = Self::new(true);

  /// An easy constant for representing a false [`Bool32`] instance.
  pub const FALSE: Self = Self::new(false);

  /// Constructs this [`Bool32`] with a boolean value.
  #[inline(always)]
  pub const fn new(b: bool) -> Self {
    Self(b as c::VkBool32)
  }
}

impl From<bool> for Bool32 {
  #[inline(always)]
  fn from(value: bool) -> Self {
    Bool32::new(value)
  }
}

impl From<c::VkBool32> for Bool32 {
  #[inline(always)]
  fn from(value: c::VkBool32) -> Self {
    Self((value != 0) as c::VkBool32)
  }
}

impl AsRef<c::VkBool32> for Bool32 {
  fn as_ref(&self) -> &c::VkBool32 {
    &self.0
  }
}

impl AsMut<c::VkBool32> for Bool32 {
  fn as_mut(&mut self) -> &mut c::VkBool32 {
    &mut self.0
  }
}

impl std::ops::Deref for Bool32 {
  type Target = c::VkBool32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for Bool32 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl PartialEq<bool> for Bool32 {
  #[inline]
  fn eq(&self, other: &bool) -> bool {
    self.0 == *other as c::VkBool32
  }
}

impl PartialEq<Bool32> for bool {
  #[inline]
  fn eq(&self, other: &Bool32) -> bool {
    *self as c::VkBool32 == other.0
  }
}

impl PartialEq<c::VkBool32> for Bool32 {
  #[inline]
  fn eq(&self, other: &c::VkBool32) -> bool {
    self.0 == Self::from(*other)
  }
}

impl PartialEq<Bool32> for c::VkBool32 {
  #[inline]
  fn eq(&self, other: &Bool32) -> bool {
    &Bool32::from(*self) == other
  }
}

impl std::fmt::Display for Bool32 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.0 == 0 { "false" } else { "true" }.fmt(f)
  }
}

impl std::fmt::Debug for Bool32 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Bool32{{{}}}", self)
  }
}

impl std::str::FromStr for Bool32 {
  type Err = ParseBoolError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    bool::from_str(s).map(Bool32::new)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use vulkan_sys as c;

  #[test]
  fn test_eq() {
    assert_eq!(Bool32::TRUE, true);
    assert_eq!(Bool32::FALSE, false);
    assert_eq!(Bool32::TRUE, 4 as c::VkBool32);
    assert_eq!(Bool32::FALSE, 0 as c::VkBool32);
    assert_eq!(true, Bool32::TRUE);
    assert_eq!(false, Bool32::FALSE);
    assert_eq!(4 as c::VkBool32, Bool32::TRUE);
    assert_eq!(0 as c::VkBool32, Bool32::FALSE);
  }

  #[test]
  fn test_ne() {
    assert_ne!(Bool32::TRUE, false);
    assert_ne!(Bool32::FALSE, true);
    assert_ne!(Bool32::FALSE, 4 as c::VkBool32);
    assert_ne!(Bool32::TRUE, 0 as c::VkBool32);
    assert_ne!(false, Bool32::TRUE);
    assert_ne!(true, Bool32::FALSE);
    assert_ne!(4 as c::VkBool32, Bool32::FALSE);
    assert_ne!(0 as c::VkBool32, Bool32::TRUE);
  }

  #[test]
  fn test_display() {
    assert_eq!(format!("{}", Bool32::from(true)), "true");
    assert_eq!(format!("{}", Bool32::from(false)), "false");
    assert_eq!(format!("{}", Bool32::from(42)), "true");
  }

  #[test]
  fn test_from_str() {
    use std::str::FromStr;
    assert_eq!(Bool32::from_str("true"), Ok(Bool32::TRUE));
    assert_eq!(Bool32::from_str("false"), Ok(Bool32::FALSE));
    assert!(Bool32::from_str("no").is_err());
  }
}
