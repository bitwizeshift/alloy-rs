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
    Self(value)
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
