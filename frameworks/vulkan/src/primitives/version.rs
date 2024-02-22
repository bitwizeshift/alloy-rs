/// Representation of a semantic version, as defined by Vulkan.
///
/// Versions in Vulkan use 7 bits for the major version, 10 for the minor,
/// 12 for the patch, and then an additional 3 for the variant.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version(u32);

impl Default for Version {
  fn default() -> Self {
    // SAFETY: each value is within the valid range.
    unsafe { Self::new_unchecked(0, 0, 1, 0) }
  }
}

impl AsRef<u32> for Version {
  fn as_ref(&self) -> &u32 {
    &self.0
  }
}

impl AsMut<u32> for Version {
  fn as_mut(&mut self) -> &mut u32 {
    &mut self.0
  }
}

impl Version {
  const VARIANT_BITS: u32 = 3;
  const MAJOR_BITS: u32 = 7;
  const MINOR_BITS: u32 = 10;
  const PATCH_BITS: u32 = 12;

  const VARIANT_OFFSET: u32 = Self::PATCH_BITS + Self::MINOR_BITS + Self::MAJOR_BITS;
  const MAJOR_OFFSET: u32 = Self::PATCH_BITS + Self::MINOR_BITS;
  const MINOR_OFFSET: u32 = Self::PATCH_BITS;
  const PATCH_OFFSET: u32 = 0;

  const VARIANT_MASK: u32 = (1 << Self::VARIANT_BITS) - 1;
  const MAJOR_MASK: u32 = (1 << Self::MAJOR_BITS) - 1;
  const MINOR_MASK: u32 = (1 << Self::MINOR_BITS) - 1;
  const PATCH_MASK: u32 = (1 << Self::PATCH_BITS) - 1;

  const VARIANT_MAX: u32 = (1 << Self::VARIANT_BITS) - 1;
  const MAJOR_MAX: u32 = (1 << Self::MAJOR_BITS) - 1;
  const MINOR_MAX: u32 = (1 << Self::MINOR_BITS) - 1;
  const PATCH_MAX: u32 = (1 << Self::PATCH_BITS) - 1;

  /// Constructs a new [`Version`] object, given the 4 components of the version.
  ///
  /// This function will return [`None`] if any of the component values exceed
  /// the maximum values for their components
  ///
  /// # Arguments
  ///
  /// * `variant` - the variant version component (between 0-7)
  /// * `major`   - the major version component (between 0-127)
  /// * `minor`   - the minor version component (between 0-1023)
  /// * `patch`   - the patch version component (between 0-4095)
  pub const fn new(variant: u32, major: u32, minor: u32, patch: u32) -> Option<Self> {
    if variant > Self::VARIANT_MAX
      || major > Self::MAJOR_MAX
      || minor > Self::MINOR_MAX
      || patch > Self::PATCH_MAX
    {
      None
    } else {
      // SAFETY: this *is* checked immediately above.
      Some(unsafe { Self::new_unchecked(variant, major, minor, patch) })
    }
  }

  /// Constructs a version without checking that each component is within the
  /// valid range of values.
  ///
  /// This assumes the value has been checked before being called.
  ///
  /// # Arguments
  ///
  /// * `variant` - the variant version component (between 0-7)
  /// * `major`   - the major version component (between 0-127)
  /// * `minor`   - the minor version component (between 0-1023)
  /// * `patch`   - the patch version component (between 0-4095)
  ///
  /// # Safety
  ///
  /// This function requires that the inputs for each of the version parts are
  /// within the above indicated ranges.
  pub const unsafe fn new_unchecked(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
    Self(
      (variant << Self::VARIANT_OFFSET)
        | (major << Self::MAJOR_OFFSET)
        | (minor << Self::MINOR_OFFSET)
        | (patch << Self::PATCH_OFFSET),
    )
  }

  /// Constructs this [`Version`] directly from the C value.
  ///
  /// # Arguments
  ///
  /// * `value` - the value
  #[inline]
  pub const fn from_c(value: u32) -> Self {
    Self(value)
  }

  /// Gets the variant component of the version.
  pub const fn variant(&self) -> u32 {
    (self.0 >> Self::VARIANT_OFFSET) & Self::VARIANT_MASK
  }

  /// Gets the major component of the version.
  pub const fn major(&self) -> u32 {
    (self.0 >> Self::MAJOR_OFFSET) & Self::MAJOR_MASK
  }

  /// Gets the minor component of the version
  pub const fn minor(&self) -> u32 {
    (self.0 >> Self::MINOR_OFFSET) & Self::MINOR_MASK
  }

  /// Gets the patch component of the verrsion
  pub const fn patch(&self) -> u32 {
    (self.0 >> Self::PATCH_OFFSET) & Self::PATCH_MASK
  }

  /// Gets the version as a raw u32 value.
  pub const fn u32(&self) -> u32 {
    self.0
  }
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}.{}.{} ({})",
      self.major(),
      self.minor(),
      self.patch(),
      self.0,
    )
  }
}

impl std::fmt::Debug for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Version{{{}}}", self)
  }
}
