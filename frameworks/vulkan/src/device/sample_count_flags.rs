use vulkan_sys as c;

///
pub struct SampleCountFlags(c::VkSampleCountFlags);

foundation::define_transparent! {
  SampleCountFlags.0 => c::VkSampleCountFlags
}

impl From<c::VkSampleCountFlags> for SampleCountFlags {
  fn from(value: c::VkSampleCountFlags) -> Self {
    Self(value)
  }
}

impl SampleCountFlags {
  /// Constructs this [`SampleCountFlags`] from the underlying [`VkSampleCountFlags`]
  ///
  /// # Arguments
  ///
  /// * `value` - the underlying value to construct from
  ///
  /// [`VkSampleCountFlags`]: c::VkSampleCountFlags
  #[inline]
  pub fn from_c(value: c::VkSampleCountFlags) -> Self {
    Self(value)
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_1_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT != 0
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_2_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_2_BIT != 0
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_4_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_4_BIT != 0
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_8_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_8_BIT != 0
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_16_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_16_BIT != 0
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_32_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_32_BIT != 0
  }

  /// Checks whether the different sample count bits are set.
  #[inline]
  pub fn sample_count_64_bit(&self) -> bool {
    self.0 & c::VkSampleCountFlagBits_VK_SAMPLE_COUNT_64_BIT != 0
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for SampleCountFlags {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut set = f.debug_set();
    if self.sample_count_1_bit() {
      set.entry(&"1");
    }
    if self.sample_count_2_bit() {
      set.entry(&"2");
    }
    if self.sample_count_4_bit() {
      set.entry(&"4");
    }
    if self.sample_count_8_bit() {
      set.entry(&"8");
    }
    if self.sample_count_16_bit() {
      set.entry(&"16");
    }
    if self.sample_count_32_bit() {
      set.entry(&"32");
    }
    if self.sample_count_64_bit() {
      set.entry(&"64");
    }

    set.finish()
  }
}
