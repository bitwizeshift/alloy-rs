//! This module provides definitions for validation layers and their properties.

use crate::{c, Version};
use core::ffi::CStr;

/// The name of the  standard Khronos validation layer.
pub const KHRONOS_VALIDATION: &CStr = foundation::cstr!("VK_LAYER_KHRONOS_validation");

/// The rust representation of the vulkan [`VkLayerProperties`]
///
/// [`VkLayerProperties`]: c::VkLayerProperties
#[repr(transparent)]
pub struct LayerProperties(c::VkLayerProperties);

unsafe impl foundation::Transparent for LayerProperties {
  type Wrapped = c::VkLayerProperties;
}

impl AsRef<c::VkLayerProperties> for LayerProperties {
  fn as_ref(&self) -> &c::VkLayerProperties {
    &self.0
  }
}

impl AsMut<c::VkLayerProperties> for LayerProperties {
  fn as_mut(&mut self) -> &mut c::VkLayerProperties {
    &mut self.0
  }
}

impl foundation::Take<c::VkLayerProperties> for LayerProperties {
  unsafe fn take(self) -> c::VkLayerProperties {
    self.0
  }
}

impl std::ops::Deref for LayerProperties {
  type Target = c::VkLayerProperties;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for LayerProperties {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl LayerProperties {
  /// Constructs this [`LayerProperties`] from the underlying Vulkan
  /// [`VkLayerProperties`]
  ///
  /// [`VkLayerProperties`]: c::VkLayerProperties
  pub fn from_c(properties: c::VkLayerProperties) -> Self {
    Self(properties)
  }

  /// Gets the name of this layer.
  pub fn layer_name(&self) -> &str {
    // SAFETY: Vulkan strings are defined in terms of UTF-8 strings.
    unsafe { self.layer_name_cstr().to_str().unwrap_unchecked() }
  }

  /// Gets the name of this layer as a CStr
  pub fn layer_name_cstr(&self) -> &CStr {
    // SAFETY: Vulkan API is required to provide null-terminated strings.
    unsafe { foundation::cstr_from_char_slice(&self.0.layerName) }
  }

  /// Gets the name of this layer.
  pub fn description(&self) -> &str {
    // SAFETY: Vulkan strings are defined in terms of UTF-8 strings.
    unsafe { self.description_cstr().to_str().unwrap_unchecked() }
  }

  /// Gets the name of this layer as a CStr
  pub fn description_cstr(&self) -> &CStr {
    // SAFETY: Vulkan API is required to provide null-terminated strings.
    unsafe { foundation::cstr_from_char_slice(&self.0.description) }
  }

  /// Gets the specification version of this layer.
  ///
  /// This returns an integer, incremented with backward compatible changes.
  pub fn spec_version(&self) -> Version {
    Version::from_c(self.0.specVersion)
  }

  /// Gets the implementation version of this layer.
  ///
  /// This returns an integer, incremented with backward compatible changes.
  pub fn implementation_version(&self) -> Version {
    Version::from_c(self.0.implementationVersion)
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for LayerProperties {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("LayerProperties")
      .field("layer_name", &self.layer_name())
      .field("description", &self.description())
      .field("spec_version", &self.spec_version())
      .field("implementation_version", &self.implementation_version())
      .finish()
  }
}
