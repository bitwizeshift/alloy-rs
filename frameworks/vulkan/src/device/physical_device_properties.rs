use crate::device::{PhysicalDeviceLimits, PhysicalDeviceSparseProperties, PhysicalDeviceType};
use crate::Version;
use std::ffi::CStr;
use vulkan_sys as c;

/// A wrapper around [`VkPhysicalDeviceProperties`] objects.
///
/// [`VkPhysicalDeviceProperties`]: c::VkPhysicalDeviceProperties
#[repr(transparent)]
pub struct PhysicalDeviceProperties(c::VkPhysicalDeviceProperties);

unsafe impl foundation::Transparent for PhysicalDeviceProperties {
  type Wrapped = c::VkPhysicalDeviceProperties;
}

impl AsRef<c::VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
  fn as_ref(&self) -> &c::VkPhysicalDeviceProperties {
    &self.0
  }
}

impl AsMut<c::VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
  fn as_mut(&mut self) -> &mut c::VkPhysicalDeviceProperties {
    &mut self.0
  }
}

impl foundation::Take<c::VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
  unsafe fn take(self) -> c::VkPhysicalDeviceProperties {
    self.0
  }
}

impl std::ops::Deref for PhysicalDeviceProperties {
  type Target = c::VkPhysicalDeviceProperties;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for PhysicalDeviceProperties {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl PhysicalDeviceProperties {
  /// Constructs this [`PhysicalDeviceProperties`] from the underlying [`VkPhysicalDeviceProperties`]
  ///
  /// # Arguments
  ///
  /// * `device` - the physical device
  ///
  /// [`VkPhysicalDeviceProperties`]: c::VkPhysicalDeviceProperties
  #[inline]
  pub fn from_c(properties: c::VkPhysicalDeviceProperties) -> Self {
    Self(properties)
  }

  /// Gets the UUID of the pipeline cache.
  #[inline]
  pub fn pipeline_cache_uuid(&self) -> foundation::Uuid {
    foundation::Uuid::from_bytes(self.0.pipelineCacheUUID)
  }

  /// Gets the name of this device as a [`str`].
  #[inline]
  pub fn name(&self) -> &str {
    // SAFETY: Vulkan strings are defined in terms of UTF-8 strings.
    unsafe { self.name_cstr().to_str().unwrap_unchecked() }
  }

  /// Gets the name of this device as a [`CStr`].
  #[inline]
  pub fn name_cstr(&self) -> &CStr {
    // SAFETY: Vulkan API is required to provide null-terminated strings.
    unsafe { foundation::cstr_from_char_slice(&self.0.deviceName) }
  }

  /// Gets the API version of this device.
  #[inline]
  pub fn api_version(&self) -> Version {
    Version::from_c(self.0.apiVersion)
  }

  /// Gets the driver version of this device.
  #[inline]
  pub fn driver_version(&self) -> Version {
    Version::from_c(self.0.driverVersion)
  }

  /// Gets the vendor ID of this device.
  #[inline]
  pub fn vendor_id(&self) -> u32 {
    self.0.vendorID
  }

  /// Gets the device ID of this device.
  #[inline]
  pub fn device_id(&self) -> u32 {
    self.0.deviceID
  }

  /// Gets the underlying device type.
  #[inline]
  pub fn device_type(&self) -> PhysicalDeviceType {
    PhysicalDeviceType::from_c(self.0.deviceType)
  }

  /// Gets the underlying device limits.
  #[inline]
  pub fn limits(&self) -> &PhysicalDeviceLimits {
    <PhysicalDeviceLimits as foundation::Transparent>::from_ref(&self.0.limits)
  }

  /// Gets the underlying sparse properties.
  #[inline]
  pub fn sparse_properties(&self) -> &PhysicalDeviceSparseProperties {
    <PhysicalDeviceSparseProperties as foundation::Transparent>::from_ref(&self.0.sparseProperties)
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for PhysicalDeviceProperties {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "  Name: {}", self.name())?;
    writeln!(f, "  Pipeline Cache UUID: {}", self.pipeline_cache_uuid())?;
    writeln!(f, "  Type: {}", self.device_type())?;
    writeln!(f, "  Device ID: 0x{:x}", self.device_id())?;
    writeln!(f, "  Vendor ID: 0x{:x}", self.vendor_id())?;
    writeln!(f, "  API Version: {}", self.api_version())?;
    writeln!(f, "  Driver Version: {}", self.driver_version())?;
    writeln!(f, "Limits:")?;
    writeln!(f, "{:?}", self.limits())?;
    writeln!(f, "Sparse Properties:")?;
    writeln!(f, "{:?}", self.sparse_properties())
  }
}
