use vulkan_sys as c;

/// A wrapped representation of the [`VkPhysicalDeviceSparseProperties`].
///
/// [`VkPhysicalDeviceSparseProperties`]: c::VkPhysicalDeviceSparseProperties
#[repr(transparent)]
pub struct PhysicalDeviceSparseProperties(c::VkPhysicalDeviceSparseProperties);

unsafe impl foundation::Transparent for PhysicalDeviceSparseProperties {
  type Wrapped = c::VkPhysicalDeviceSparseProperties;
}

impl AsRef<c::VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
  fn as_ref(&self) -> &c::VkPhysicalDeviceSparseProperties {
    &self.0
  }
}

impl AsMut<c::VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
  fn as_mut(&mut self) -> &mut c::VkPhysicalDeviceSparseProperties {
    &mut self.0
  }
}

impl foundation::Take<c::VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
  unsafe fn take(self) -> c::VkPhysicalDeviceSparseProperties {
    self.0
  }
}

impl std::ops::Deref for PhysicalDeviceSparseProperties {
  type Target = c::VkPhysicalDeviceSparseProperties;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for PhysicalDeviceSparseProperties {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl PhysicalDeviceSparseProperties {
  /// Returns the sparse property residencyStandard2DBlockShape.
  #[inline]
  pub fn residency_standard_2d_block_shape(&self) -> bool {
    self.0.residencyStandard2DBlockShape != 0
  }

  /// Returns the sparse property residencyStandard2DMultisampleBlockShape.
  #[inline]
  pub fn residency_standard_2d_multisample_block_shape(&self) -> bool {
    self.0.residencyStandard2DMultisampleBlockShape != 0
  }

  /// Returns the sparse property residencyStandard3DBlockShape.
  #[inline]
  pub fn residency_standard_3d_block_shape(&self) -> bool {
    self.0.residencyStandard3DBlockShape != 0
  }

  /// Returns the sparse property residencyAlignedMipSize.
  #[inline]
  pub fn residency_aligned_mip_size(&self) -> bool {
    self.0.residencyAlignedMipSize != 0
  }

  /// Returns the sparse property residencyNonResidentStrict.
  #[inline]
  pub fn residency_non_resident_strict(&self) -> bool {
    self.0.residencyNonResidentStrict != 0
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for PhysicalDeviceSparseProperties {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("PhysicalDeviceSparseProperties")
      .field(
        "residency_standard_2d_block_shape",
        &self.residency_standard_2d_block_shape(),
      )
      .field(
        "residency_standard_2d_multisample_block_shape",
        &self.residency_standard_2d_multisample_block_shape(),
      )
      .field(
        "residency_standard_3d_block_shape",
        &self.residency_standard_3d_block_shape(),
      )
      .field(
        "residency_aligned_mip_size",
        &self.residency_aligned_mip_size(),
      )
      .field(
        "residency_non_resident_strict",
        &self.residency_non_resident_strict(),
      )
      .finish()
  }
}
