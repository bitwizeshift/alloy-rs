use crate::c;

/// An error returned from the Vulkan API
pub struct Error(c::VkResult);

impl Error {
  /// Constructs a [`Result`] object that corresponds to whether the value
  /// being checked is a [`VK_SUCCESS`] or an error.
  ///
  /// # Arguments
  ///
  /// * `result` - the result status from a Vulkan API call.
  ///
  /// [`VK_SUCCESS`]: c::VkResult_VK_SUCCESS
  pub const fn check(result: c::VkResult) -> Result<()> {
    if !Self::is_error_status(result) {
      Ok(())
    } else {
      Err(Error(result))
    }
  }

  /// Queries whether the input result is an error status.
  pub const fn is_error_status(result: c::VkResult) -> bool {
    !matches!(
      result,
      c::VkResult_VK_SUCCESS
        | c::VkResult_VK_EVENT_RESET
        | c::VkResult_VK_EVENT_SET
        | c::VkResult_VK_INCOMPLETE
        | c::VkResult_VK_NOT_READY
        | c::VkResult_VK_SUBOPTIMAL_KHR
        | c::VkResult_VK_TIMEOUT
    )
  }

  /// Constructs this [`Error`] from a [`VkResult`].
  ///
  /// [`VkResult`]: c::VkResult
  #[inline]
  pub const fn from_c(value: c::VkResult) -> Self {
    Self(value)
  }

  /// Converts this error into a string.
  pub const fn as_str(&self) -> &str {
    match self.0 {
      c::VkResult_VK_ERROR_DEVICE_LOST => "VK_ERROR_DEVICE_LOST",
      c::VkResult_VK_ERROR_EXTENSION_NOT_PRESENT => "VK_ERROR_EXTENSION_NOT_PRESENT",
      c::VkResult_VK_ERROR_FEATURE_NOT_PRESENT => "VK_ERROR_FEATURE_NOT_PRESENT",
      c::VkResult_VK_ERROR_FORMAT_NOT_SUPPORTED => "VK_ERROR_FORMAT_NOT_SUPPORTED",
      c::VkResult_VK_ERROR_FRAGMENTATION_EXT => "VK_ERROR_FRAGMENTATION_EXT",
      c::VkResult_VK_ERROR_FRAGMENTED_POOL => "VK_ERROR_FRAGMENTED_POOL",
      c::VkResult_VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
        "VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT"
      }
      c::VkResult_VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "VK_ERROR_INCOMPATIBLE_DISPLAY_KHR",
      c::VkResult_VK_ERROR_INCOMPATIBLE_DRIVER => "VK_ERROR_INCOMPATIBLE_DRIVER",
      c::VkResult_VK_ERROR_INITIALIZATION_FAILED => "VK_ERROR_INITIALIZATION_FAILED",
      c::VkResult_VK_ERROR_INVALID_DEVICE_ADDRESS_EXT => "VK_ERROR_INVALID_DEVICE_ADDRESS_EXT",
      c::VkResult_VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
        "VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT"
      }
      c::VkResult_VK_ERROR_INVALID_EXTERNAL_HANDLE => "VK_ERROR_INVALID_EXTERNAL_HANDLE",
      c::VkResult_VK_ERROR_INVALID_SHADER_NV => "VK_ERROR_INVALID_SHADER_NV",
      c::VkResult_VK_ERROR_LAYER_NOT_PRESENT => "VK_ERROR_LAYER_NOT_PRESENT",
      c::VkResult_VK_ERROR_MEMORY_MAP_FAILED => "VK_ERROR_MEMORY_MAP_FAILED",
      c::VkResult_VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "VK_ERROR_NATIVE_WINDOW_IN_USE_KHR",
      c::VkResult_VK_ERROR_NOT_PERMITTED_EXT => "VK_ERROR_NOT_PERMITTED_EXT",
      c::VkResult_VK_ERROR_OUT_OF_DATE_KHR => "VK_ERROR_OUT_OF_DATE_KHR",
      c::VkResult_VK_ERROR_OUT_OF_DEVICE_MEMORY => "VK_ERROR_OUT_OF_DEVICE_MEMORY",
      c::VkResult_VK_ERROR_OUT_OF_HOST_MEMORY => "VK_ERROR_OUT_OF_HOST_MEMORY",
      c::VkResult_VK_ERROR_OUT_OF_POOL_MEMORY => "VK_ERROR_OUT_OF_POOL_MEMORY",
      c::VkResult_VK_ERROR_SURFACE_LOST_KHR => "VK_ERROR_SURFACE_LOST_KHR",
      c::VkResult_VK_ERROR_TOO_MANY_OBJECTS => "VK_ERROR_TOO_MANY_OBJECTS",
      c::VkResult_VK_ERROR_VALIDATION_FAILED_EXT => "VK_ERROR_VALIDATION_FAILED_EXT",
      c::VkResult_VK_EVENT_RESET => "VK_EVENT_RESET",
      c::VkResult_VK_EVENT_SET => "VK_EVENT_SET",
      c::VkResult_VK_INCOMPLETE => "VK_INCOMPLETE",
      c::VkResult_VK_NOT_READY => "VK_NOT_READY",
      c::VkResult_VK_SUBOPTIMAL_KHR => "VK_SUBOPTIMAL_KHR",
      c::VkResult_VK_SUCCESS => "VK_SUCCESS",
      c::VkResult_VK_TIMEOUT => "VK_TIMEOUT",
      _ => "Unhandled VkResult",
    }
  }

  /// Returns an instance of this [`Error`] as a [`VkResult`]
  ///
  /// [`VkResult`]: c::VkResult
  #[inline]
  pub const fn result(&self) -> c::VkResult {
    self.0
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.as_str().fmt(f)
  }
}
impl std::fmt::Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}({})", self.as_str(), self.0)
  }
}
impl std::error::Error for Error {}

/// The result of a Vulkan operation that may fail.
pub type Result<T> = std::result::Result<T, Error>;
