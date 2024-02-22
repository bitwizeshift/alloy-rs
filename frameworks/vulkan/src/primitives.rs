use vulkan_sys as c;

mod result;

pub use result::Error;

pub use result::Result;

mod version;

pub use version::Version;

/// A special constant representing the 1.0 vulkan version.
pub const VERSION_1_0: Version = unsafe { Version::new_unchecked(0, 1, 0, 0) };

/// A special constant representing the 1.1 vulkan version.
pub const VERSION_1_1: Version = unsafe { Version::new_unchecked(0, 1, 1, 0) };

/// A special constant representing the 1.2 vulkan version.
pub const VERSION_1_2: Version = unsafe { Version::new_unchecked(0, 1, 2, 0) };

/// A special constant representing the 1.3 vulkan version.
pub const VERSION_1_3: Version = unsafe { Version::new_unchecked(0, 1, 3, 0) };

mod bool32;

pub use bool32::*;

#[doc(inline)]
pub use c::VkDeviceSize as DeviceSize;

#[doc(inline)]
pub use c::VkExtent2D as Extent2D;

#[doc(inline)]
pub use c::VkExtent3D as Extent3D;
