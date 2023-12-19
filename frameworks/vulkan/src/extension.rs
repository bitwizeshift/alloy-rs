//! This module provides definitions for Extensions and properties.

use crate::c;
use core::ffi::CStr;

// Note: This is not an exhaustive list (yet).

/// The extension for the KHR Swapchain
pub const KHR_SWAPCHAIN: &CStr = foundation::cstr!("VK_KHR_swapchain");

/// The extension for KHR surfaces.
pub const KHR_SURFACE: &CStr = foundation::cstr!("VK_KHR_surface");

/// The extension for enumerating portabilities.
pub const PORTABILITY_ENUMERATION: &CStr = foundation::cstr!("VK_KHR_portability_enumeration");

/// The extension for portability subsets.
pub const PORTABILITY_SUBSET: &CStr = foundation::cstr!("VK_KHR_portability_subset");

/// The Apple extension for Metal-API surfaces.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const EXT_METAL_SURFACE: &CStr = foundation::cstr!("VK_EXT_metal_surface");

/// Representation of the [`VkExtensionProperties`] object.
///
/// [`VkExtensionProperties`]: c::VkExtensionProperties
#[repr(transparent)]
pub struct ExtensionProperties(c::VkExtensionProperties);

unsafe impl foundation::Transparent for ExtensionProperties {
  type Wrapped = c::VkExtensionProperties;
}

impl AsRef<c::VkExtensionProperties> for ExtensionProperties {
  #[inline]
  fn as_ref(&self) -> &c::VkExtensionProperties {
    &self.0
  }
}

impl AsMut<c::VkExtensionProperties> for ExtensionProperties {
  #[inline]
  fn as_mut(&mut self) -> &mut c::VkExtensionProperties {
    &mut self.0
  }
}

impl foundation::Take<c::VkExtensionProperties> for ExtensionProperties {
  #[inline]
  unsafe fn take(self) -> c::VkExtensionProperties {
    self.0
  }
}

impl std::ops::Deref for ExtensionProperties {
  type Target = c::VkExtensionProperties;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for ExtensionProperties {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl ExtensionProperties {
  /// Constructs this [`ExtensionProperties`] from the underlying Vulkan
  /// [`VkExtensionProperties`]
  ///
  /// [`VkExtensionProperties`]: c::VkExtensionProperties
  pub fn from_c(properties: c::VkExtensionProperties) -> Self {
    Self(properties)
  }

  /// Gets the name of this extension.
  pub fn extension_name(&self) -> &str {
    // SAFETY: Vulkan strings are defined in terms of UTF-8 strings.
    unsafe { self.extension_name_cstr().to_str().unwrap_unchecked() }
  }

  /// Gets the name of this extension as a CStr
  pub fn extension_name_cstr(&self) -> &CStr {
    // SAFETY: Vulkan API is required to provide null-terminated strings.
    unsafe { foundation::cstr_from_char_slice(&self.0.extensionName) }
  }

  /// Gets the version of this extension.
  ///
  /// This returns an integer, incremented with backward compatible changes.
  pub fn spec_version(&self) -> u32 {
    self.0.specVersion
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for ExtensionProperties {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ExtensionProperties")
      .field("extension_name", &self.extension_name())
      .field("spec_version", &self.spec_version())
      .finish()
  }
}
