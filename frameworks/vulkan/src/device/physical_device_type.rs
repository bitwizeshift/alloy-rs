use vulkan_sys as c;

/// A wrapper around [`VkPhysicalDeviceType`]
///
/// [`VkPhysicalDeviceType`]: c::VkPhysicalDeviceType
#[derive(Copy, Clone)]
pub enum PhysicalDeviceType {
  /// The device is a standard CPU
  CPU,
  /// The device is a discrete GPU
  DiscreteGPU,
  /// The device is an integrated GPU
  IntegratedGPU,
  /// The device is a virtual or emulated GPU
  VirtualGPU,
  /// The device is some other type of device.
  Other,
}

impl From<c::VkPhysicalDeviceType> for PhysicalDeviceType {
  fn from(value: c::VkPhysicalDeviceType) -> Self {
    Self::from_c(value)
  }
}

impl From<PhysicalDeviceType> for c::VkPhysicalDeviceType {
  fn from(value: PhysicalDeviceType) -> Self {
    value.to_c()
  }
}

impl std::fmt::Display for PhysicalDeviceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::CPU => "CPU",
      Self::DiscreteGPU => "Discrete GPU",
      Self::IntegratedGPU => "Integrated GPU",
      Self::VirtualGPU => "Virtual GPU",
      Self::Other => "Other",
    }
    .fmt(f)
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for PhysicalDeviceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::CPU => "VK_PHYSICAL_DEVICE_TYPE_CPU",
      Self::DiscreteGPU => "VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU",
      Self::IntegratedGPU => "VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU",
      Self::VirtualGPU => "VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU",
      Self::Other => "VK_PHYSICAL_DEVICE_TYPE_OTHER",
    }
    .fmt(f)
  }
}

impl PhysicalDeviceType {
  /// Constructs this [`PhysicalDeviceType`] from an instance of [`VkPhysicalDeviceType`].
  ///
  /// # Arguments
  ///
  /// * `t` - the underlying type to use
  pub fn from_c(t: c::VkPhysicalDeviceType) -> Self {
    match t {
      c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_CPU => Self::CPU,
      c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => Self::DiscreteGPU,
      c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => Self::IntegratedGPU,
      c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => Self::VirtualGPU,
      _ => Self::Other,
    }
  }

  /// Converts this [`PhysicalDeviceType`] into a C Vulkan type
  pub fn to_c(&self) -> c::VkPhysicalDeviceType {
    match self {
      Self::CPU => c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_CPU,
      Self::DiscreteGPU => c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
      Self::IntegratedGPU => c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU,
      Self::VirtualGPU => c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU,
      Self::Other => c::VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_OTHER,
    }
  }
}
