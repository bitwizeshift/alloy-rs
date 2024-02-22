//! This module provides wrappers for both logical and physical Vulkan devices
//! (e.g. [`VkPhysicalDevice`])
//!
//! [`VkPhysicalDevice`]: crate::c::VkPhysicalDevice

use crate::c;
use crate::device::{PhysicalDeviceFeatures, PhysicalDeviceProperties};
use crate::extension::ExtensionProperties;
use crate::instance::Instance;
use crate::queue::QueueFamilyProperties;
use crate::surface::Surface;

/// A wrapper around [`VkPhysicalDevice`] objects.
///
/// [`VkPhysicalDevice`]: c::VkPhysicalDevice
pub struct PhysicalDevice<'a> {
  device: c::VkPhysicalDevice,
  instance: &'a Instance,
}

impl<'a> PhysicalDevice<'a> {
  /// Constructs this [`PhysicalDevice`] from the underlying [`VkPhysicalDevice`]
  ///
  /// # Panics
  ///
  /// This function will [`panic`] if the `device` is null.
  ///
  /// # Arguments
  ///
  /// * `device` - the physical device
  ///
  /// [`VkPhysicalDevice`]: c::VkPhysicalDevice
  pub fn from_c(instance: &'a Instance, device: c::VkPhysicalDevice) -> Self {
    debug_assert!(!device.is_null());
    Self { device, instance }
  }

  /// Returns the underlying [`PhysicalDeviceProperties`] of this device.
  pub fn properties(&self) -> PhysicalDeviceProperties {
    let mut out = unsafe { std::mem::zeroed::<c::VkPhysicalDeviceProperties>() };
    unsafe {
      c::vkGetPhysicalDeviceProperties(self.device, &mut out as *mut c::VkPhysicalDeviceProperties)
    }

    PhysicalDeviceProperties::from_c(out)
  }

  /// Returns the underlying [`PhysicalDeviceFeatures`] of this device.
  pub fn features(&self) -> PhysicalDeviceFeatures {
    let mut out = unsafe { std::mem::zeroed::<c::VkPhysicalDeviceFeatures>() };
    unsafe {
      c::vkGetPhysicalDeviceFeatures(self.device, &mut out as *mut c::VkPhysicalDeviceFeatures)
    }

    PhysicalDeviceFeatures::from_c(out)
  }

  /// Returns the list of queue family properties for this device.
  pub fn queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
    let mut count: u32 = 0;
    unsafe {
      c::vkGetPhysicalDeviceQueueFamilyProperties(
        self.device,
        &mut count as *mut u32,
        std::ptr::null_mut(),
      );

      foundation::read_transparent_out_vec(count as usize, |p| {
        let mut count = count;
        c::vkGetPhysicalDeviceQueueFamilyProperties(self.device, &mut count as *mut u32, p);
        p
      })
    }
  }

  /// Checks for surface support with the given queue family index.
  pub fn surface_support(&self, queue_family_index: usize, surface: &Surface<'_>) -> bool {
    let mut out = crate::Bool32::new(false);

    unsafe {
      c::vkGetPhysicalDeviceSurfaceSupportKHR(
        self.device,
        queue_family_index as u32,
        *surface.as_ref(),
        out.as_mut() as *mut c::VkBool32,
      )
    };
    out == true
  }

  /// Queries available device extensions.
  pub fn device_extensions(&self) -> Vec<ExtensionProperties> {
    let mut count: u32 = 0;
    unsafe {
      c::vkEnumerateDeviceExtensionProperties(
        self.device,
        std::ptr::null(),
        &mut count as *mut u32,
        std::ptr::null_mut(),
      );
      foundation::read_transparent_out_vec(count as usize, |v| {
        let mut count = count;
        c::vkEnumerateDeviceExtensionProperties(
          self.device,
          std::ptr::null(),
          &mut count as *mut u32,
          v,
        );
        v
      })
    }
  }

  /// Gets the underlying Vulkan instance from the associated device.
  pub fn instance(&self) -> &Instance {
    self.instance
  }

  /// Extracts the internal [`VkPhysicalDevice`] into a [`RawPhysicalDevice`]
  /// which does not carry automatic resource management.
  ///
  /// # Safety
  ///
  /// This operation requires that the caller now take proper ownership of the
  /// [`RawPhysicalDevice`] and manage dropping this operation later.
  pub unsafe fn take(self) -> RawPhysicalDevice {
    RawPhysicalDevice(self.device)
  }
}

impl<'a> AsRef<c::VkPhysicalDevice> for PhysicalDevice<'a> {
  /// Gets a reference to the underlying [`VkPhysicalDevice`].
  ///
  /// [`VkPhysicalDevice`]: c::VkPhysicalDevice
  fn as_ref(&self) -> &c::VkPhysicalDevice {
    &self.device
  }
}

impl<'a> AsMut<c::VkPhysicalDevice> for PhysicalDevice<'a> {
  /// Gets a mutable reference to the underlying [`VkPhysicalDevice`].
  ///
  /// [`VkPhysicalDevice`]: c::VkPhysicalDevice
  fn as_mut(&mut self) -> &mut c::VkPhysicalDevice {
    &mut self.device
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for PhysicalDevice<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("debug")
      .field("properties", &self.properties())
      .field("features", &self.features())
      .field("queue_family_properties", &self.queue_family_properties())
      .finish()
  }
}

///
pub struct RawPhysicalDevice(c::VkPhysicalDevice);

impl RawPhysicalDevice {
  /// Takes ownership of the underlying [`VkPhysicalDevice`]
  ///
  /// # Safety
  ///
  /// This operation requires that the caller manually handle dropping the
  /// device.
  ///
  /// [`VkPhysicalDevice`]: c::VkPhysicalDevice
  #[inline]
  pub unsafe fn take(self) -> c::VkPhysicalDevice {
    self.0
  }
}

impl AsRef<c::VkPhysicalDevice> for RawPhysicalDevice {
  /// Gets a reference to the underlying [`VkPhysicalDevice`].
  ///
  /// [`VkPhysicalDevice`]: c::VkPhysicalDevice
  fn as_ref(&self) -> &c::VkPhysicalDevice {
    &self.0
  }
}

impl AsMut<c::VkPhysicalDevice> for RawPhysicalDevice {
  /// Gets a mutable reference to the underlying [`VkPhysicalDevice`].
  ///
  /// [`VkPhysicalDevice`]: c::VkPhysicalDevice
  fn as_mut(&mut self) -> &mut c::VkPhysicalDevice {
    &mut self.0
  }
}
