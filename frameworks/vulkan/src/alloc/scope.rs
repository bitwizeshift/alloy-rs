use vulkan_sys as c;

/// Represents an allocation Scope that may be requested, so that data can appear
/// on separate heaps.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
  /// The allocation is for Command objects
  Command,
  /// The allocation is for Objects
  Object,
  /// The allocation is for Catch objects
  Cache,
  /// The allocation is for Device objects
  Device,
  /// The allocation is for Instance objects
  Instance,
}

impl Scope {
  /// Converts a C [`VkSystemAllocationScope`] into an allocation [`Scope`].
  ///
  /// This function will return [`None`] if the scope is not a valid scope value.
  ///
  /// # Arguments
  ///
  /// * `scope` - the allocation scope
  ///
  /// [`VkSystemAllocationScope`]: c::VkSystemAllocationScope
  pub fn from_c(scope: c::VkSystemAllocationScope) -> Option<Scope> {
    match scope {
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_COMMAND => Some(Scope::Command),
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_OBJECT => Some(Scope::Object),
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_CACHE => Some(Scope::Cache),
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_DEVICE => Some(Scope::Device),
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE => Some(Scope::Instance),
      _ => None,
    }
  }

  /// Converts a C [`VkSystemAllocationScope`] into an allocation [`Scope`].
  ///
  /// # Safety
  ///
  /// The user is responsible of making sure that `scope` is a valid value.
  ///
  /// # Arguments
  ///
  /// * `scope` - the allocation scope
  ///
  /// [`VkSystemAllocationScope`]: c::VkSystemAllocationScope
  pub unsafe fn from_c_unchecked(scope: c::VkSystemAllocationScope) -> Scope {
    match scope {
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_COMMAND => Scope::Command,
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_OBJECT => Scope::Object,
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_CACHE => Scope::Cache,
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_DEVICE => Scope::Device,
      c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE => Scope::Instance,
      _ => unreachable!(),
    }
  }

  /// Converts this [`Scope`] into a [`VkSystemAllocationScope`].
  ///
  /// [`VkSystemAllocationScope`]: c::VkSystemAllocationScope
  pub fn to_c(self) -> c::VkSystemAllocationScope {
    match self {
      Scope::Command => c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_COMMAND,
      Scope::Object => c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_OBJECT,
      Scope::Cache => c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_CACHE,
      Scope::Device => c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_DEVICE,
      Scope::Instance => c::VkSystemAllocationScope_VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE,
    }
  }
}
