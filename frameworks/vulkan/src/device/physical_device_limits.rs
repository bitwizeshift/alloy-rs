use crate::device::SampleCountFlags;
use crate::DeviceSize;
use vulkan_sys as c;

///
#[repr(transparent)]
pub struct PhysicalDeviceLimits(c::VkPhysicalDeviceLimits);

unsafe impl foundation::Transparent for PhysicalDeviceLimits {
  type Wrapped = c::VkPhysicalDeviceLimits;
}

impl AsRef<c::VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
  fn as_ref(&self) -> &c::VkPhysicalDeviceLimits {
    &self.0
  }
}

impl AsMut<c::VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
  fn as_mut(&mut self) -> &mut c::VkPhysicalDeviceLimits {
    &mut self.0
  }
}

impl foundation::Take<c::VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
  unsafe fn take(self) -> c::VkPhysicalDeviceLimits {
    self.0
  }
}

impl std::ops::Deref for PhysicalDeviceLimits {
  type Target = c::VkPhysicalDeviceLimits;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for PhysicalDeviceLimits {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl PhysicalDeviceLimits {
  /// Constructs this [`PhysicalDeviceFeatures`] from the underlying
  /// [`VkPhysicalDeviceLimits`]
  ///
  /// # Arguments
  ///
  /// * `device` - the physical device
  ///
  /// [`VkPhysicalDeviceLimits`]: c::VkPhysicalDeviceLimits
  #[inline]
  pub fn from_c(limits: c::VkPhysicalDeviceLimits) -> Self {
    Self(limits)
  }

  /// Returns the device's maxImageDimension1D
  #[inline]
  pub fn max_image_dimension_1d(&self) -> u32 {
    self.0.maxImageDimension1D
  }

  /// Returns the device's maxImageDimension2D
  #[inline]
  pub fn max_image_dimension_2d(&self) -> u32 {
    self.0.maxImageDimension2D
  }

  /// Returns the device's maxImageDimension3D
  #[inline]
  pub fn max_image_dimension_3d(&self) -> u32 {
    self.0.maxImageDimension3D
  }
  /// Returns the device's maxImageDimensionCube
  #[inline]
  pub fn max_image_dimension_cube(&self) -> u32 {
    self.0.maxImageDimensionCube
  }

  /// Returns the device's maxImageArrayLayers
  #[inline]
  pub fn max_image_array_layers(&self) -> u32 {
    self.0.maxImageArrayLayers
  }

  /// Returns the device's maxTexelBufferElements
  #[inline]
  pub fn max_texel_buffer_elements(&self) -> u32 {
    self.0.maxTexelBufferElements
  }

  /// Returns the device's maxUniformBufferRange
  #[inline]
  pub fn max_uniform_buffer_range(&self) -> u32 {
    self.0.maxUniformBufferRange
  }

  /// Returns the device's maxStorageBufferRange
  #[inline]
  pub fn max_storage_buffer_range(&self) -> u32 {
    self.0.maxStorageBufferRange
  }

  /// Returns the device's maxPushConstantsSize
  #[inline]
  pub fn max_push_constants_size(&self) -> u32 {
    self.0.maxPushConstantsSize
  }

  /// Returns the device's maxMemoryAllocationCount
  #[inline]
  pub fn max_memory_allocation_count(&self) -> u32 {
    self.0.maxMemoryAllocationCount
  }

  /// Returns the device's maxSamplerAllocationCount
  #[inline]
  pub fn max_sampler_allocation_count(&self) -> u32 {
    self.0.maxSamplerAllocationCount
  }

  /// Returns the device's bufferImageGranularity
  #[inline]
  pub fn buffer_image_granularity(&self) -> DeviceSize {
    self.0.bufferImageGranularity
  }

  /// Returns the device's sparseAddressSpaceSize
  #[inline]
  pub fn sparse_address_space_size(&self) -> DeviceSize {
    self.0.sparseAddressSpaceSize
  }

  /// Returns the device's maxBoundDescriptorSets
  #[inline]
  pub fn max_bound_descriptor_sets(&self) -> u32 {
    self.0.maxBoundDescriptorSets
  }

  /// Returns the device's maxPerStageDescriptorSamplers
  #[inline]
  pub fn max_per_stage_descriptor_samplers(&self) -> u32 {
    self.0.maxPerStageDescriptorSamplers
  }

  /// Returns the device's maxPerStageDescriptorUniformBuffers
  #[inline]
  pub fn max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
    self.0.maxPerStageDescriptorUniformBuffers
  }

  /// Returns the device's maxPerStageDescriptorStorageBuffers
  #[inline]
  pub fn max_per_stage_descriptor_storage_buffers(&self) -> u32 {
    self.0.maxPerStageDescriptorStorageBuffers
  }

  /// Returns the device's maxPerStageDescriptorSampledImages
  #[inline]
  pub fn max_per_stage_descriptor_sampled_images(&self) -> u32 {
    self.0.maxPerStageDescriptorSampledImages
  }

  /// Returns the device's maxPerStageDescriptorStorageImages
  #[inline]
  pub fn max_per_stage_descriptor_storage_images(&self) -> u32 {
    self.0.maxPerStageDescriptorStorageImages
  }

  /// Returns the device's maxPerStageDescriptorInputAttachments
  #[inline]
  pub fn max_per_stage_descriptor_input_attachments(&self) -> u32 {
    self.0.maxPerStageDescriptorInputAttachments
  }

  /// Returns the device's maxPerStageResources
  #[inline]
  pub fn max_per_stage_resources(&self) -> u32 {
    self.0.maxPerStageResources
  }

  /// Returns the device's maxDescriptorSetSamplers
  #[inline]
  pub fn max_descriptor_set_samplers(&self) -> u32 {
    self.0.maxDescriptorSetSamplers
  }

  /// Returns the device's maxDescriptorSetUniformBuffers
  #[inline]
  pub fn max_descriptor_set_uniform_buffers(&self) -> u32 {
    self.0.maxDescriptorSetUniformBuffers
  }

  /// Returns the device's maxDescriptorSetUniformBuffersDynamic
  #[inline]
  pub fn max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
    self.0.maxDescriptorSetUniformBuffersDynamic
  }

  /// Returns the device's maxDescriptorSetStorageBuffers
  #[inline]
  pub fn max_descriptor_set_storage_buffers(&self) -> u32 {
    self.0.maxDescriptorSetStorageBuffers
  }

  /// Returns the device's maxDescriptorSetStorageBuffersDynamic
  #[inline]
  pub fn max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
    self.0.maxDescriptorSetStorageBuffersDynamic
  }

  /// Returns the device's maxDescriptorSetSampledImages
  #[inline]
  pub fn max_descriptor_set_sampled_images(&self) -> u32 {
    self.0.maxDescriptorSetSampledImages
  }

  /// Returns the device's maxDescriptorSetStorageImages
  #[inline]
  pub fn max_descriptor_set_storage_images(&self) -> u32 {
    self.0.maxDescriptorSetStorageImages
  }

  /// Returns the device's maxDescriptorSetInputAttachments
  #[inline]
  pub fn max_descriptor_set_input_attachments(&self) -> u32 {
    self.0.maxDescriptorSetInputAttachments
  }

  /// Returns the device's maxVertexInputAttributes
  #[inline]
  pub fn max_vertex_input_attributes(&self) -> u32 {
    self.0.maxVertexInputAttributes
  }

  /// Returns the device's maxVertexInputBindings
  #[inline]
  pub fn max_vertex_input_bindings(&self) -> u32 {
    self.0.maxVertexInputBindings
  }

  /// Returns the device's maxVertexInputAttributeOffset
  #[inline]
  pub fn max_vertex_input_attribute_offset(&self) -> u32 {
    self.0.maxVertexInputAttributeOffset
  }

  /// Returns the device's maxVertexInputBindingStride
  #[inline]
  pub fn max_vertex_input_binding_stride(&self) -> u32 {
    self.0.maxVertexInputBindingStride
  }

  /// Returns the device's maxVertexOutputComponents
  #[inline]
  pub fn max_vertex_output_components(&self) -> u32 {
    self.0.maxVertexOutputComponents
  }

  /// Returns the device's maxTessellationGenerationLevel
  #[inline]
  pub fn max_tessellation_generation_level(&self) -> u32 {
    self.0.maxTessellationGenerationLevel
  }

  /// Returns the device's maxTessellationPatchSize
  #[inline]
  pub fn max_tessellation_patch_size(&self) -> u32 {
    self.0.maxTessellationPatchSize
  }

  /// Returns the device's maxTessellationControlPerVertexInputComponents
  #[inline]
  pub fn max_tessellation_control_per_vertex_input_components(&self) -> u32 {
    self.0.maxTessellationControlPerVertexInputComponents
  }

  /// Returns the device's maxTessellationControlPerVertexOutputComponents
  #[inline]
  pub fn max_tessellation_control_per_vertex_output_components(&self) -> u32 {
    self.0.maxTessellationControlPerVertexOutputComponents
  }

  /// Returns the device's maxTessellationControlPerPatchOutputComponents
  #[inline]
  pub fn max_tessellation_control_per_patch_output_components(&self) -> u32 {
    self.0.maxTessellationControlPerPatchOutputComponents
  }

  /// Returns the device's maxTessellationControlTotalOutputComponents
  #[inline]
  pub fn max_tessellation_control_total_output_components(&self) -> u32 {
    self.0.maxTessellationControlTotalOutputComponents
  }

  /// Returns the device's maxTessellationEvaluationInputComponents
  #[inline]
  pub fn max_tessellation_evaluation_input_components(&self) -> u32 {
    self.0.maxTessellationEvaluationInputComponents
  }

  /// Returns the device's maxTessellationEvaluationOutputComponents
  #[inline]
  pub fn max_tessellation_evaluation_output_components(&self) -> u32 {
    self.0.maxTessellationEvaluationOutputComponents
  }

  /// Returns the device's maxGeometryShaderInvocations
  #[inline]
  pub fn max_geometry_shader_invocations(&self) -> u32 {
    self.0.maxGeometryShaderInvocations
  }

  /// Returns the device's maxGeometryInputComponents
  #[inline]
  pub fn max_geometry_input_components(&self) -> u32 {
    self.0.maxGeometryInputComponents
  }

  /// Returns the device's maxGeometryOutputComponents
  #[inline]
  pub fn max_geometry_output_components(&self) -> u32 {
    self.0.maxGeometryOutputComponents
  }

  /// Returns the device's maxGeometryOutputVertices
  #[inline]
  pub fn max_geometry_output_vertices(&self) -> u32 {
    self.0.maxGeometryOutputVertices
  }

  /// Returns the device's maxGeometryTotalOutputComponents
  #[inline]
  pub fn max_geometry_total_output_components(&self) -> u32 {
    self.0.maxGeometryTotalOutputComponents
  }

  /// Returns the device's maxFragmentInputComponents
  #[inline]
  pub fn max_fragment_input_components(&self) -> u32 {
    self.0.maxFragmentInputComponents
  }

  /// Returns the device's maxFragmentOutputAttachments
  #[inline]
  pub fn max_fragment_output_attachments(&self) -> u32 {
    self.0.maxFragmentOutputAttachments
  }

  /// Returns the device's maxFragmentDualSrcAttachments
  #[inline]
  pub fn max_fragment_dual_src_attachments(&self) -> u32 {
    self.0.maxFragmentDualSrcAttachments
  }

  /// Returns the device's maxFragmentCombinedOutputResources
  #[inline]
  pub fn max_fragment_combined_output_resources(&self) -> u32 {
    self.0.maxFragmentCombinedOutputResources
  }

  /// Returns the device's maxComputeSharedMemorySize
  #[inline]
  pub fn max_compute_shared_memory_size(&self) -> u32 {
    self.0.maxComputeSharedMemorySize
  }

  /// Returns the device's maxComputeWorkGroupCount
  #[inline]
  pub fn max_compute_work_group_count(&self) -> [u32; 3usize] {
    self.0.maxComputeWorkGroupCount
  }

  /// Returns the device's maxComputeWorkGroupInvocations
  #[inline]
  pub fn max_compute_work_group_invocations(&self) -> u32 {
    self.0.maxComputeWorkGroupInvocations
  }

  /// Returns the device's maxComputeWorkGroupSize
  #[inline]
  pub fn max_compute_work_group_size(&self) -> [u32; 3usize] {
    self.0.maxComputeWorkGroupSize
  }

  /// Returns the device's subPixelPrecisionBits
  #[inline]
  pub fn sub_pixel_precision_bits(&self) -> u32 {
    self.0.subPixelPrecisionBits
  }

  /// Returns the device's subTexelPrecisionBits
  #[inline]
  pub fn sub_texel_precision_bits(&self) -> u32 {
    self.0.subTexelPrecisionBits
  }

  /// Returns the device's mipmapPrecisionBits
  #[inline]
  pub fn mipmap_precision_bits(&self) -> u32 {
    self.0.mipmapPrecisionBits
  }

  /// Returns the device's maxDrawIndexedIndexValue
  #[inline]
  pub fn max_draw_indexed_index_value(&self) -> u32 {
    self.0.maxDrawIndexedIndexValue
  }

  /// Returns the device's maxDrawIndirectCount
  #[inline]
  pub fn max_draw_indirect_count(&self) -> u32 {
    self.0.maxDrawIndirectCount
  }

  /// Returns the device's maxSamplerLodBias
  #[inline]
  pub fn max_sampler_lod_bias(&self) -> f32 {
    self.0.maxSamplerLodBias
  }

  /// Returns the device's maxSamplerAnisotropy
  #[inline]
  pub fn max_sampler_anisotropy(&self) -> f32 {
    self.0.maxSamplerAnisotropy
  }

  /// Returns the device's maxViewports
  #[inline]
  pub fn max_viewports(&self) -> u32 {
    self.0.maxViewports
  }

  /// Returns the device's maxViewportDimensions
  #[inline]
  pub fn max_viewport_dimensions(&self) -> [u32; 2usize] {
    self.0.maxViewportDimensions
  }

  /// Returns the device's viewportBoundsRange
  #[inline]
  pub fn viewport_bounds_range(&self) -> [f32; 2usize] {
    self.0.viewportBoundsRange
  }

  /// Returns the device's viewportSubPixelBits
  #[inline]
  pub fn viewport_sub_pixel_bits(&self) -> u32 {
    self.0.viewportSubPixelBits
  }

  /// Returns the device's minMemoryMapAlignment
  #[inline]
  pub fn min_memory_map_alignment(&self) -> usize {
    self.0.minMemoryMapAlignment
  }

  /// Returns the device's minTexelBufferOffsetAlignment
  #[inline]
  pub fn min_texel_buffer_offset_alignment(&self) -> DeviceSize {
    self.0.minTexelBufferOffsetAlignment
  }

  /// Returns the device's minUniformBufferOffsetAlignment
  #[inline]
  pub fn min_uniform_buffer_offset_alignment(&self) -> DeviceSize {
    self.0.minUniformBufferOffsetAlignment
  }

  /// Returns the device's minStorageBufferOffsetAlignment
  #[inline]
  pub fn min_storage_buffer_offset_alignment(&self) -> DeviceSize {
    self.0.minStorageBufferOffsetAlignment
  }

  /// Returns the device's minTexelOffset
  #[inline]
  pub fn min_texel_offset(&self) -> i32 {
    self.0.minTexelOffset
  }

  /// Returns the device's maxTexelOffset
  #[inline]
  pub fn max_texel_offset(&self) -> u32 {
    self.0.maxTexelOffset
  }

  /// Returns the device's minTexelGatherOffset
  #[inline]
  pub fn min_texel_gather_offset(&self) -> i32 {
    self.0.minTexelGatherOffset
  }

  /// Returns the device's maxTexelGatherOffset
  #[inline]
  pub fn max_texel_gather_offset(&self) -> u32 {
    self.0.maxTexelGatherOffset
  }

  /// Returns the device's minInterpolationOffset
  #[inline]
  pub fn min_interpolation_offset(&self) -> f32 {
    self.0.minInterpolationOffset
  }

  /// Returns the device's maxInterpolationOffset
  #[inline]
  pub fn max_interpolation_offset(&self) -> f32 {
    self.0.maxInterpolationOffset
  }

  /// Returns the device's subPixelInterpolationOffsetBits
  #[inline]
  pub fn sub_pixel_interpolation_offset_bits(&self) -> u32 {
    self.0.subPixelInterpolationOffsetBits
  }

  /// Returns the device's maxFramebufferWidth
  #[inline]
  pub fn max_framebuffer_width(&self) -> u32 {
    self.0.maxFramebufferWidth
  }

  /// Returns the device's maxFramebufferHeight
  #[inline]
  pub fn max_framebuffer_height(&self) -> u32 {
    self.0.maxFramebufferHeight
  }

  /// Returns the device's maxFramebufferLayers
  #[inline]
  pub fn max_framebuffer_layers(&self) -> u32 {
    self.0.maxFramebufferLayers
  }

  /// Returns the device's framebufferColorSampleCounts
  #[inline]
  pub fn framebuffer_color_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.framebufferColorSampleCounts)
  }

  /// Returns the device's framebufferDepthSampleCounts
  #[inline]
  pub fn framebuffer_depth_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.framebufferDepthSampleCounts)
  }

  /// Returns the device's framebufferStencilSampleCounts
  #[inline]
  pub fn framebuffer_stencil_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.framebufferStencilSampleCounts)
  }

  /// Returns the device's framebufferNoAttachmentsSampleCounts
  #[inline]
  pub fn framebuffer_no_attachments_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.framebufferNoAttachmentsSampleCounts)
  }

  /// Returns the device's maxColorAttachments
  #[inline]
  pub fn max_color_attachments(&self) -> u32 {
    self.0.maxColorAttachments
  }

  /// Returns the device's sampledImageColorSampleCounts
  #[inline]
  pub fn sampled_image_color_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.sampledImageColorSampleCounts)
  }

  /// Returns the device's sampledImageIntegerSampleCounts
  #[inline]
  pub fn sampled_image_integer_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.sampledImageIntegerSampleCounts)
  }

  /// Returns the device's sampledImageDepthSampleCounts
  #[inline]
  pub fn sampled_image_depth_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.sampledImageDepthSampleCounts)
  }

  /// Returns the device's sampledImageStencilSampleCounts
  #[inline]
  pub fn sampled_image_stencil_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.sampledImageStencilSampleCounts)
  }

  /// Returns the device's storageImageSampleCounts
  #[inline]
  pub fn storage_image_sample_counts(&self) -> SampleCountFlags {
    SampleCountFlags::from_c(self.0.storageImageSampleCounts)
  }

  /// Returns the device's maxSampleMaskWords
  #[inline]
  pub fn max_sample_mask_words(&self) -> u32 {
    self.0.maxSampleMaskWords
  }

  /// Returns the device's timestampComputeAndGraphics
  #[inline]
  pub fn timestamp_compute_and_graphics(&self) -> bool {
    self.0.timestampComputeAndGraphics != 0
  }

  /// Returns the device's timestampPeriod
  #[inline]
  pub fn timestamp_period(&self) -> f32 {
    self.0.timestampPeriod
  }

  /// Returns the device's maxClipDistances
  #[inline]
  pub fn max_clip_distances(&self) -> u32 {
    self.0.maxClipDistances
  }

  /// Returns the device's maxCullDistances
  #[inline]
  pub fn max_cull_distances(&self) -> u32 {
    self.0.maxCullDistances
  }

  /// Returns the device's maxCombinedClipAndCullDistances
  #[inline]
  pub fn max_combined_clip_and_cull_distances(&self) -> u32 {
    self.0.maxCombinedClipAndCullDistances
  }

  /// Returns the device's discreteQueuePriorities
  #[inline]
  pub fn discrete_queue_priorities(&self) -> u32 {
    self.0.discreteQueuePriorities
  }

  /// Returns the device's pointSizeRange
  #[inline]
  pub fn point_size_range(&self) -> [f32; 2usize] {
    self.0.pointSizeRange
  }

  /// Returns the device's lineWidthRange
  #[inline]
  pub fn line_width_range(&self) -> [f32; 2usize] {
    self.0.lineWidthRange
  }

  /// Returns the device's pointSizeGranularity
  #[inline]
  pub fn point_size_granularity(&self) -> f32 {
    self.0.pointSizeGranularity
  }

  /// Returns the device's lineWidthGranularity
  #[inline]
  pub fn line_width_granularity(&self) -> f32 {
    self.0.lineWidthGranularity
  }

  /// Returns the device's strictLines
  #[inline]
  pub fn strict_lines(&self) -> bool {
    self.0.strictLines != 0
  }

  /// Returns the device's standardSampleLocations
  #[inline]
  pub fn standard_sample_locations(&self) -> bool {
    self.0.standardSampleLocations != 0
  }

  /// Returns the device's optimalBufferCopyOffsetAlignment
  #[inline]
  pub fn optimal_buffer_copy_offset_alignment(&self) -> DeviceSize {
    self.0.optimalBufferCopyOffsetAlignment
  }

  /// Returns the device's optimalBufferCopyRowPitchAlignment
  #[inline]
  pub fn optimal_buffer_copy_row_pitch_alignment(&self) -> DeviceSize {
    self.0.optimalBufferCopyRowPitchAlignment
  }

  /// Returns the device's nonCoherentAtomSize
  #[inline]
  pub fn non_coherent_atom_size(&self) -> DeviceSize {
    self.0.nonCoherentAtomSize
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for PhysicalDeviceLimits {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(
      f,
      "  maxImageDimension1D: {}",
      self.max_image_dimension_1d()
    )?;
    writeln!(
      f,
      "  maxImageDimension2D: {}",
      self.max_image_dimension_2d()
    )?;
    writeln!(
      f,
      "  maxImageDimension3D: {}",
      self.max_image_dimension_3d()
    )?;
    writeln!(
      f,
      "  maxImageDimensionCube: {}",
      self.max_image_dimension_cube()
    )?;
    writeln!(
      f,
      "  maxImageArrayLayers: {}",
      self.max_image_array_layers()
    )?;
    writeln!(
      f,
      "  maxTexelBufferElements: {}",
      self.max_texel_buffer_elements()
    )?;
    writeln!(
      f,
      "  maxUniformBufferRange: {}",
      self.max_uniform_buffer_range()
    )?;
    writeln!(
      f,
      "  maxStorageBufferRange: {}",
      self.max_storage_buffer_range()
    )?;
    writeln!(
      f,
      "  maxPushConstantsSize: {}",
      self.max_push_constants_size()
    )?;
    writeln!(
      f,
      "  maxMemoryAllocationCount: {}",
      self.max_memory_allocation_count(),
    )?;
    writeln!(
      f,
      "  maxSamplerAllocationCount: {}",
      self.max_sampler_allocation_count(),
    )?;
    writeln!(
      f,
      "  bufferImageGranularity: {}",
      self.buffer_image_granularity()
    )?;
    writeln!(
      f,
      "  sparseAddressSpaceSize: {}",
      self.sparse_address_space_size()
    )?;
    writeln!(
      f,
      "  maxBoundDescriptorSets: {}",
      self.max_bound_descriptor_sets()
    )?;
    writeln!(
      f,
      "  maxPerStageDescriptorSamplers: {}",
      self.max_per_stage_descriptor_samplers(),
    )?;
    writeln!(
      f,
      "  maxPerStageDescriptorUniformBuffers: {}",
      self.max_per_stage_descriptor_uniform_buffers(),
    )?;
    writeln!(
      f,
      "  maxPerStageDescriptorStorageBuffers: {}",
      self.max_per_stage_descriptor_storage_buffers(),
    )?;
    writeln!(
      f,
      "  maxPerStageDescriptorSampledImages: {}",
      self.max_per_stage_descriptor_sampled_images(),
    )?;
    writeln!(
      f,
      "  maxPerStageDescriptorStorageImages: {}",
      self.max_per_stage_descriptor_storage_images(),
    )?;
    writeln!(
      f,
      "  maxPerStageDescriptorInputAttachments: {}",
      self.max_per_stage_descriptor_input_attachments(),
    )?;
    writeln!(
      f,
      "  maxPerStageResources: {}",
      self.max_per_stage_resources()
    )?;
    writeln!(
      f,
      "  maxDescriptorSetSamplers: {}",
      self.max_descriptor_set_samplers(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetUniformBuffers: {}",
      self.max_descriptor_set_uniform_buffers(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetUniformBuffersDynamic: {}",
      self.max_descriptor_set_uniform_buffers_dynamic(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetStorageBuffers: {}",
      self.max_descriptor_set_storage_buffers(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetStorageBuffersDynamic: {}",
      self.max_descriptor_set_storage_buffers_dynamic(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetSampledImages: {}",
      self.max_descriptor_set_sampled_images(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetStorageImages: {}",
      self.max_descriptor_set_storage_images(),
    )?;
    writeln!(
      f,
      "  maxDescriptorSetInputAttachments: {}",
      self.max_descriptor_set_input_attachments(),
    )?;
    writeln!(
      f,
      "  maxVertexInputAttributes: {}",
      self.max_vertex_input_attributes(),
    )?;
    writeln!(
      f,
      "  maxVertexInputBindings: {}",
      self.max_vertex_input_bindings()
    )?;
    writeln!(
      f,
      "  maxVertexInputAttributeOffset: {}",
      self.max_vertex_input_attribute_offset(),
    )?;
    writeln!(
      f,
      "  maxVertexInputBindingStride: {}",
      self.max_vertex_input_binding_stride(),
    )?;
    writeln!(
      f,
      "  maxVertexOutputComponents: {}",
      self.max_vertex_output_components(),
    )?;
    writeln!(
      f,
      "  maxTessellationGenerationLevel: {}",
      self.max_tessellation_generation_level(),
    )?;
    writeln!(
      f,
      "  maxTessellationPatchSize: {}",
      self.max_tessellation_patch_size(),
    )?;
    writeln!(
      f,
      "  maxTessellationControlPerVertexInputComponents: {}",
      self.max_tessellation_control_per_vertex_input_components(),
    )?;
    writeln!(
      f,
      "  maxTessellationControlPerVertexOutputComponents: {}",
      self.max_tessellation_control_per_vertex_output_components(),
    )?;
    writeln!(
      f,
      "  maxTessellationControlPerPatchOutputComponents: {}",
      self.max_tessellation_control_per_patch_output_components(),
    )?;
    writeln!(
      f,
      "  maxTessellationControlTotalOutputComponents: {}",
      self.max_tessellation_control_total_output_components(),
    )?;
    writeln!(
      f,
      "  maxTessellationEvaluationInputComponents: {}",
      self.max_tessellation_evaluation_input_components(),
    )?;
    writeln!(
      f,
      "  maxTessellationEvaluationOutputComponents: {}",
      self.max_tessellation_evaluation_output_components(),
    )?;
    writeln!(
      f,
      "  maxGeometryShaderInvocations: {}",
      self.max_geometry_shader_invocations(),
    )?;
    writeln!(
      f,
      "  maxGeometryInputComponents: {}",
      self.max_geometry_input_components(),
    )?;
    writeln!(
      f,
      "  maxGeometryOutputComponents: {}",
      self.max_geometry_output_components(),
    )?;
    writeln!(
      f,
      "  maxGeometryOutputVertices: {}",
      self.max_geometry_output_vertices(),
    )?;
    writeln!(
      f,
      "  maxGeometryTotalOutputComponents: {}",
      self.max_geometry_total_output_components(),
    )?;
    writeln!(
      f,
      "  maxFragmentInputComponents: {}",
      self.max_fragment_input_components(),
    )?;
    writeln!(
      f,
      "  maxFragmentOutputAttachments: {}",
      self.max_fragment_output_attachments(),
    )?;
    writeln!(
      f,
      "  maxFragmentDualSrcAttachments: {}",
      self.max_fragment_dual_src_attachments(),
    )?;
    writeln!(
      f,
      "  maxFragmentCombinedOutputResources: {}",
      self.max_fragment_combined_output_resources(),
    )?;
    writeln!(
      f,
      "  maxComputeSharedMemorySize: {}",
      self.max_compute_shared_memory_size(),
    )?;
    writeln!(
      f,
      "  maxComputeWorkGroupCount: {:?}",
      self.max_compute_work_group_count(),
    )?;
    writeln!(
      f,
      "  maxComputeWorkGroupInvocations: {}",
      self.max_compute_work_group_invocations(),
    )?;
    writeln!(
      f,
      "  maxComputeWorkGroupSize: {:?}",
      self.max_compute_work_group_size(),
    )?;
    writeln!(
      f,
      "  subPixelPrecisionBits: {}",
      self.sub_pixel_precision_bits()
    )?;
    writeln!(
      f,
      "  subTexelPrecisionBits: {}",
      self.sub_texel_precision_bits()
    )?;
    writeln!(f, "  mipmapPrecisionBits: {}", self.mipmap_precision_bits())?;
    writeln!(
      f,
      "  maxDrawIndexedIndexValue: {}",
      self.max_draw_indexed_index_value(),
    )?;
    writeln!(
      f,
      "  maxDrawIndirectCount: {}",
      self.max_draw_indirect_count()
    )?;
    writeln!(f, "  maxSamplerLodBias: {}", self.max_sampler_lod_bias())?;
    writeln!(
      f,
      "  maxSamplerAnisotropy: {}",
      self.max_sampler_anisotropy()
    )?;
    writeln!(f, "  maxViewports: {}", self.max_viewports())?;
    writeln!(
      f,
      "  maxViewportDimensions: {:?}",
      self.max_viewport_dimensions(),
    )?;
    writeln!(
      f,
      "  viewportBoundsRange: {:?}",
      self.viewport_bounds_range()
    )?;
    writeln!(
      f,
      "  viewportSubPixelBits: {}",
      self.viewport_sub_pixel_bits()
    )?;
    writeln!(
      f,
      "  minMemoryMapAlignment: {}",
      self.min_memory_map_alignment()
    )?;
    writeln!(
      f,
      "  minTexelBufferOffsetAlignment: {}",
      self.min_texel_buffer_offset_alignment(),
    )?;
    writeln!(
      f,
      "  minUniformBufferOffsetAlignment: {}",
      self.min_uniform_buffer_offset_alignment(),
    )?;
    writeln!(
      f,
      "  minStorageBufferOffsetAlignment: {}",
      self.min_storage_buffer_offset_alignment(),
    )?;
    writeln!(f, "  minTexelOffset: {}", self.min_texel_offset())?;
    writeln!(f, "  maxTexelOffset: {}", self.max_texel_offset())?;
    writeln!(
      f,
      "  minTexelGatherOffset: {}",
      self.min_texel_gather_offset()
    )?;
    writeln!(
      f,
      "  maxTexelGatherOffset: {}",
      self.max_texel_gather_offset()
    )?;
    writeln!(
      f,
      "  minInterpolationOffset: {}",
      self.min_interpolation_offset()
    )?;
    writeln!(
      f,
      "  maxInterpolationOffset: {}",
      self.max_interpolation_offset()
    )?;
    writeln!(
      f,
      "  subPixelInterpolationOffsetBits: {}",
      self.sub_pixel_interpolation_offset_bits(),
    )?;
    writeln!(f, "  maxFramebufferWidth: {}", self.max_framebuffer_width())?;
    writeln!(
      f,
      "  maxFramebufferHeight: {}",
      self.max_framebuffer_height()
    )?;
    writeln!(
      f,
      "  maxFramebufferLayers: {}",
      self.max_framebuffer_layers()
    )?;
    writeln!(
      f,
      "  framebufferColorSampleCounts: {:?}",
      self.framebuffer_color_sample_counts(),
    )?;
    writeln!(
      f,
      "  framebufferDepthSampleCounts: {:?}",
      self.framebuffer_depth_sample_counts(),
    )?;
    writeln!(
      f,
      "  framebufferStencilSampleCounts: {:?}",
      self.framebuffer_stencil_sample_counts(),
    )?;
    writeln!(
      f,
      "  framebufferNoAttachmentsSampleCounts: {:?}",
      self.framebuffer_no_attachments_sample_counts(),
    )?;
    writeln!(f, "  maxColorAttachments: {}", self.max_color_attachments())?;
    writeln!(
      f,
      "  sampledImageColorSampleCounts: {:?}",
      self.sampled_image_color_sample_counts(),
    )?;
    writeln!(
      f,
      "  sampledImageIntegerSampleCounts: {:?}",
      self.sampled_image_integer_sample_counts(),
    )?;
    writeln!(
      f,
      "  sampledImageDepthSampleCounts: {:?}",
      self.sampled_image_depth_sample_counts(),
    )?;
    writeln!(
      f,
      "  sampledImageStencilSampleCounts: {:?}",
      self.sampled_image_stencil_sample_counts(),
    )?;
    writeln!(
      f,
      "  storageImageSampleCounts: {:?}",
      self.storage_image_sample_counts(),
    )?;
    writeln!(f, "  maxSampleMaskWords: {}", self.max_sample_mask_words())?;
    writeln!(
      f,
      "  timestampComputeAndGraphics: {}",
      self.timestamp_compute_and_graphics(),
    )?;
    writeln!(f, "  timestampPeriod: {}", self.timestamp_period())?;
    writeln!(f, "  maxClipDistances: {}", self.max_clip_distances())?;
    writeln!(f, "  maxCullDistances: {}", self.max_cull_distances())?;
    writeln!(
      f,
      "  maxCombinedClipAndCullDistances: {}",
      self.max_combined_clip_and_cull_distances(),
    )?;
    writeln!(
      f,
      "  discreteQueuePriorities: {}",
      self.discrete_queue_priorities(),
    )?;
    writeln!(
      f,
      "  pointSizeRange: {:?}",
      self.point_size_range().as_slice()
    )?;
    writeln!(f, "  lineWidthRange: {:?}", self.line_width_range())?;
    writeln!(
      f,
      "  pointSizeGranularity: {}",
      self.point_size_granularity()
    )?;
    writeln!(
      f,
      "  lineWidthGranularity: {}",
      self.line_width_granularity()
    )?;
    writeln!(f, "  strictLines: {}", self.strict_lines())?;
    writeln!(
      f,
      "  standardSampleLocations: {}",
      self.standard_sample_locations(),
    )?;
    writeln!(
      f,
      "  optimalBufferCopyOffsetAlignment: {}",
      self.optimal_buffer_copy_offset_alignment(),
    )?;
    writeln!(
      f,
      "  optimalBufferCopyRowPitchAlignment: {}",
      self.optimal_buffer_copy_row_pitch_alignment(),
    )?;
    writeln!(
      f,
      "  nonCoherentAtomSize: {}",
      self.non_coherent_atom_size()
    )?;
    Ok(())
  }
}
