use vulkan_sys as c;

/// A wrapper around [`VkPhysicalDeviceFeatures`] objects.
///
/// [`VkPhysicalDeviceFeatures`]: c::VkPhysicalDeviceFeatures
#[repr(transparent)]
pub struct PhysicalDeviceFeatures(c::VkPhysicalDeviceFeatures);

unsafe impl foundation::Transparent for PhysicalDeviceFeatures {
  type Wrapped = c::VkPhysicalDeviceFeatures;
}

impl AsRef<c::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
  fn as_ref(&self) -> &c::VkPhysicalDeviceFeatures {
    &self.0
  }
}

impl AsMut<c::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
  fn as_mut(&mut self) -> &mut c::VkPhysicalDeviceFeatures {
    &mut self.0
  }
}

impl foundation::Take<c::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
  unsafe fn take(self) -> c::VkPhysicalDeviceFeatures {
    self.0
  }
}

impl std::ops::Deref for PhysicalDeviceFeatures {
  type Target = c::VkPhysicalDeviceFeatures;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for PhysicalDeviceFeatures {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl PhysicalDeviceFeatures {
  /// Constructs this [`VkPhysicalDeviceFeatures`] from the underlying [`VkPhysicalDeviceFeatures`]
  ///
  /// # Arguments
  ///
  /// * `device` - the physical device
  ///
  /// [`VkPhysicalDeviceFeatures`]: c::VkPhysicalDeviceFeatures
  #[inline]
  pub fn from_c(properties: c::VkPhysicalDeviceFeatures) -> Self {
    Self(properties)
  }

  /// Checks whether robust buffer access is supported on this device.
  #[inline]
  pub fn robust_buffer_access(&self) -> bool {
    self.0.robustBufferAccess != 0
  }

  /// Checks whether full draw index with uint32 is supported on this device.
  #[inline]
  pub fn full_draw_index_uint32(&self) -> bool {
    self.0.fullDrawIndexUint32 != 0
  }

  /// Checks whether image cube array is supported on this device.
  #[inline]
  pub fn image_cube_array(&self) -> bool {
    self.0.imageCubeArray != 0
  }

  /// Checks whether independent blend is supported on this device.
  #[inline]
  pub fn independent_blend(&self) -> bool {
    self.0.independentBlend != 0
  }

  /// Checks whether geometry shader is supported on this device.
  #[inline]
  pub fn geometry_shader(&self) -> bool {
    self.0.geometryShader != 0
  }

  /// Checks whether tessellation shader is supported on this device.
  #[inline]
  pub fn tessellation_shader(&self) -> bool {
    self.0.tessellationShader != 0
  }

  /// Checks whether sample rate shading is supported on this device.
  #[inline]
  pub fn sample_rate_shading(&self) -> bool {
    self.0.sampleRateShading != 0
  }

  /// Checks whether dual src blend is supported on this device.
  #[inline]
  pub fn dual_src_blend(&self) -> bool {
    self.0.dualSrcBlend != 0
  }

  /// Checks whether logic op is supported on this device.
  #[inline]
  pub fn logic_op(&self) -> bool {
    self.0.logicOp != 0
  }

  /// Checks whether multi draw indirect is supported on this device.
  #[inline]
  pub fn multi_draw_indirect(&self) -> bool {
    self.0.multiDrawIndirect != 0
  }

  /// Checks whether draw indirect first instance is supported on this device.
  #[inline]
  pub fn draw_indirect_first_instance(&self) -> bool {
    self.0.drawIndirectFirstInstance != 0
  }

  /// Checks whether depth clamp is supported on this device.
  #[inline]
  pub fn depth_clamp(&self) -> bool {
    self.0.depthClamp != 0
  }

  /// Checks whether depth bias clamp is supported on this device.
  #[inline]
  pub fn depth_bias_clamp(&self) -> bool {
    self.0.depthBiasClamp != 0
  }

  /// Checks whether fill mode non-solid is supported on this device.
  #[inline]
  pub fn fill_mode_non_solid(&self) -> bool {
    self.0.fillModeNonSolid != 0
  }

  /// Checks whether depth bounds is supported on this device.
  #[inline]
  pub fn depth_bounds(&self) -> bool {
    self.0.depthBounds != 0
  }

  /// Checks whether wide lines is supported on this device.
  #[inline]
  pub fn wide_lines(&self) -> bool {
    self.0.wideLines != 0
  }

  /// Checks whether large points is supported on this device.
  #[inline]
  pub fn large_points(&self) -> bool {
    self.0.largePoints != 0
  }

  /// Checks whether alpha to one is supported on this device.
  #[inline]
  pub fn alpha_to_one(&self) -> bool {
    self.0.alphaToOne != 0
  }

  /// Checks whether multi viewport is supported on this device.
  #[inline]
  pub fn multi_viewport(&self) -> bool {
    self.0.multiViewport != 0
  }

  /// Checks whether sampler anisotropy is supported on this device.
  #[inline]
  pub fn sampler_anisotropy(&self) -> bool {
    self.0.samplerAnisotropy != 0
  }

  /// Checks whether texture compression 'etc2' is supported on this device.
  #[inline]
  pub fn texture_compression_etc2(&self) -> bool {
    self.0.textureCompressionETC2 != 0
  }

  /// Checks whether texture compression 'astc ldr' is supported on this device.
  #[inline]
  pub fn texture_compression_astc_ldr(&self) -> bool {
    self.0.textureCompressionASTC_LDR != 0
  }

  /// Checks whether texture compression 'bc' is supported on this device.
  #[inline]
  pub fn texture_compression_bc(&self) -> bool {
    self.0.textureCompressionBC != 0
  }

  /// Checks whether occlusion query precise is supported on this device.
  #[inline]
  pub fn occlusion_query_precise(&self) -> bool {
    self.0.occlusionQueryPrecise != 0
  }

  /// Checks whether pipeline statistics query is supported on this device.
  #[inline]
  pub fn pipeline_statistics_query(&self) -> bool {
    self.0.pipelineStatisticsQuery != 0
  }

  /// Checks whether vertexpipeline stores and atomics is supported on this device.
  #[inline]
  pub fn vertex_pipeline_stores_and_atomics(&self) -> bool {
    self.0.vertexPipelineStoresAndAtomics != 0
  }

  /// Checks whether fragment stores and atomics is supported on this device.
  #[inline]
  pub fn fragment_stores_and_atomics(&self) -> bool {
    self.0.fragmentStoresAndAtomics != 0
  }

  /// Checks whether shader tessellation and geometry point size is supported on this device.
  #[inline]
  pub fn shader_tessellation_and_geometry_point_size(&self) -> bool {
    self.0.shaderTessellationAndGeometryPointSize != 0
  }

  /// Checks whether shader image gather extended is supported on this device.
  #[inline]
  pub fn shader_image_gather_extended(&self) -> bool {
    self.0.shaderImageGatherExtended != 0
  }

  /// Checks whether shader storage image extended formats is supported on this device.
  #[inline]
  pub fn shader_storage_image_extended_formats(&self) -> bool {
    self.0.shaderStorageImageExtendedFormats != 0
  }

  /// Checks whether shader_storage_image_multisample is supported on this device.
  #[inline]
  pub fn shader_storage_image_multisample(&self) -> bool {
    self.0.shaderStorageImageMultisample != 0
  }

  /// Checks whether shader_storage_image_read_without_format is supported on this device.
  #[inline]
  pub fn shader_storage_image_read_without_format(&self) -> bool {
    self.0.shaderStorageImageReadWithoutFormat != 0
  }

  /// Checks whether shader_storage_image_write_without_format is supported on this device.
  #[inline]
  pub fn shader_storage_image_write_without_format(&self) -> bool {
    self.0.shaderStorageImageWriteWithoutFormat != 0
  }

  /// Checks whether shader_uniform_buffer_array_dynamic_indexing is supported on this device.
  #[inline]
  pub fn shader_uniform_buffer_array_dynamic_indexing(&self) -> bool {
    self.0.shaderUniformBufferArrayDynamicIndexing != 0
  }

  /// Checks whether shader_sampled_image_array_dynamic_indexing is supported on this device.
  #[inline]
  pub fn shader_sampled_image_array_dynamic_indexing(&self) -> bool {
    self.0.shaderSampledImageArrayDynamicIndexing != 0
  }

  /// Checks whether shader_storage_buffer_array_dynamic_indexing is supported on this device.
  #[inline]
  pub fn shader_storage_buffer_array_dynamic_indexing(&self) -> bool {
    self.0.shaderStorageBufferArrayDynamicIndexing != 0
  }

  /// Checks whether shader_storage_image_array_dynamic_indexing is supported on this device.
  #[inline]
  pub fn shader_storage_image_array_dynamic_indexing(&self) -> bool {
    self.0.shaderStorageImageArrayDynamicIndexing != 0
  }

  /// Checks whether shader_clip_distance is supported on this device.
  #[inline]
  pub fn shader_clip_distance(&self) -> bool {
    self.0.shaderClipDistance != 0
  }

  /// Checks whether shader_cull_distance is supported on this device.
  #[inline]
  pub fn shader_cull_distance(&self) -> bool {
    self.0.shaderCullDistance != 0
  }

  /// Checks whether shader_float64 is supported on this device.
  #[inline]
  pub fn shader_float64(&self) -> bool {
    self.0.shaderFloat64 != 0
  }

  /// Checks whether shader_int64 is supported on this device.
  #[inline]
  pub fn shader_int64(&self) -> bool {
    self.0.shaderInt64 != 0
  }

  /// Checks whether shader_int16 is supported on this device.
  #[inline]
  pub fn shader_int16(&self) -> bool {
    self.0.shaderInt16 != 0
  }

  /// Checks whether shader_resource_residency is supported on this device.
  #[inline]
  pub fn shader_resource_residency(&self) -> bool {
    self.0.shaderResourceResidency != 0
  }

  /// Checks whether shader_resource_min_lod is supported on this device.
  #[inline]
  pub fn shader_resource_min_lod(&self) -> bool {
    self.0.shaderResourceMinLod != 0
  }

  /// Checks whether sparse_binding is supported on this device.
  #[inline]
  pub fn sparse_binding(&self) -> bool {
    self.0.sparseBinding != 0
  }

  /// Checks whether sparse_residency_buffer is supported on this device.
  #[inline]
  pub fn sparse_residency_buffer(&self) -> bool {
    self.0.sparseResidencyBuffer != 0
  }

  /// Checks whether sparse_residency_image2d is supported on this device.
  #[inline]
  pub fn sparse_residency_image2d(&self) -> bool {
    self.0.sparseResidencyImage2D != 0
  }

  /// Checks whether sparse_residency_image3d is supported on this device.
  #[inline]
  pub fn sparse_residency_image3d(&self) -> bool {
    self.0.sparseResidencyImage3D != 0
  }

  /// Checks whether sparse_residency2_samples is supported on this device.
  #[inline]
  pub fn sparse_residency2_samples(&self) -> bool {
    self.0.sparseResidency2Samples != 0
  }

  /// Checks whether sparse_residency4_samples is supported on this device.
  #[inline]
  pub fn sparse_residency4_samples(&self) -> bool {
    self.0.sparseResidency4Samples != 0
  }

  /// Checks whether sparse_residency8_samples is supported on this device.
  #[inline]
  pub fn sparse_residency8_samples(&self) -> bool {
    self.0.sparseResidency8Samples != 0
  }

  /// Checks whether sparse_residency16_samples is supported on this device.
  #[inline]
  pub fn sparse_residency16_samples(&self) -> bool {
    self.0.sparseResidency16Samples != 0
  }

  /// Checks whether sparse_residency_aliased is supported on this device.
  #[inline]
  pub fn sparse_residency_aliased(&self) -> bool {
    self.0.sparseResidencyAliased != 0
  }

  /// Checks whether variable_multisample_rate is supported on this device.
  #[inline]
  pub fn variable_multisample_rate(&self) -> bool {
    self.0.variableMultisampleRate != 0
  }

  /// Checks whether inherited_queries is supported on this device.
  #[inline]
  pub fn inherited_queries(&self) -> bool {
    self.0.inheritedQueries != 0
  }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for PhysicalDeviceFeatures {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "  robustBufferAccess: {}", self.robust_buffer_access())?;
    writeln!(
      f,
      "  fullDrawIndexUint32: {}",
      self.full_draw_index_uint32()
    )?;
    writeln!(f, "  imageCubeArray: {}", self.image_cube_array())?;
    writeln!(f, "  independentBlend: {}", self.independent_blend())?;
    writeln!(f, "  geometryShader: {}", self.geometry_shader())?;
    writeln!(f, "  tessellationShader: {}", self.tessellation_shader())?;
    writeln!(f, "  sampleRateShading: {}", self.sample_rate_shading())?;
    writeln!(f, "  dualSrcBlend: {}", self.dual_src_blend())?;
    writeln!(f, "  logicOp: {}", self.logic_op())?;
    writeln!(f, "  multiDrawIndirect: {}", self.multi_draw_indirect())?;
    writeln!(
      f,
      "  drawIndirectFirstInstance: {}",
      self.draw_indirect_first_instance()
    )?;
    writeln!(f, "  depthClamp: {}", self.depth_clamp())?;
    writeln!(f, "  depthBiasClamp: {}", self.depth_bias_clamp())?;
    writeln!(f, "  fillModeNonSolid: {}", self.fill_mode_non_solid())?;
    writeln!(f, "  depthBounds: {}", self.depth_bounds())?;
    writeln!(f, "  wideLines: {}", self.wide_lines())?;
    writeln!(f, "  largePoints: {}", self.large_points())?;
    writeln!(f, "  alphaToOne: {}", self.alpha_to_one())?;
    writeln!(f, "  multiViewport: {}", self.multi_viewport())?;
    writeln!(f, "  samplerAnisotropy: {}", self.sampler_anisotropy())?;
    writeln!(
      f,
      "  textureCompressionETC2: {}",
      self.texture_compression_etc2()
    )?;
    writeln!(
      f,
      "  textureCompressionASTC_LDR: {}",
      self.texture_compression_astc_ldr()
    )?;
    writeln!(
      f,
      "  textureCompressionBC: {}",
      self.texture_compression_bc()
    )?;
    writeln!(
      f,
      "  occlusionQueryPrecise: {}",
      self.occlusion_query_precise()
    )?;
    writeln!(
      f,
      "  pipelineStatisticsQuery: {}",
      self.pipeline_statistics_query()
    )?;
    writeln!(
      f,
      "  vertexPipelineStoresAndAtomics: {}",
      self.vertex_pipeline_stores_and_atomics()
    )?;
    writeln!(
      f,
      "  fragmentStoresAndAtomics: {}",
      self.fragment_stores_and_atomics()
    )?;
    writeln!(
      f,
      "  shaderTessellationAndGeometryPointSize: {}",
      self.shader_tessellation_and_geometry_point_size()
    )?;
    writeln!(
      f,
      "  shaderImageGatherExtended: {}",
      self.shader_image_gather_extended()
    )?;
    writeln!(
      f,
      "  shaderStorageImageExtendedFormats: {}",
      self.shader_storage_image_extended_formats()
    )?;
    writeln!(
      f,
      "  shaderStorageImageMultisample: {}",
      self.shader_storage_image_multisample()
    )?;
    writeln!(
      f,
      "  shaderStorageImageReadWithoutFormat: {}",
      self.shader_storage_image_read_without_format()
    )?;
    writeln!(
      f,
      "  shaderStorageImageWriteWithoutFormat: {}",
      self.shader_storage_image_write_without_format()
    )?;
    writeln!(
      f,
      "  shaderUniformBufferArrayDynamicIndexing: {}",
      self.shader_uniform_buffer_array_dynamic_indexing()
    )?;
    writeln!(
      f,
      "  shaderSampledImageArrayDynamicIndexing: {}",
      self.shader_sampled_image_array_dynamic_indexing()
    )?;
    writeln!(
      f,
      "  shaderStorageBufferArrayDynamicIndexing: {}",
      self.shader_storage_buffer_array_dynamic_indexing()
    )?;
    writeln!(
      f,
      "  shaderStorageImageArrayDynamicIndexing: {}",
      self.shader_storage_image_array_dynamic_indexing()
    )?;
    writeln!(f, "  shaderClipDistance: {}", self.shader_clip_distance())?;
    writeln!(f, "  shaderCullDistance: {}", self.shader_cull_distance())?;
    writeln!(f, "  shaderFloat64: {}", self.shader_float64())?;
    writeln!(f, "  shaderInt64: {}", self.shader_int64())?;
    writeln!(f, "  shaderInt16: {}", self.shader_int16())?;
    writeln!(
      f,
      "  shaderResourceResidency: {}",
      self.shader_resource_residency()
    )?;
    writeln!(
      f,
      "  shaderResourceMinLod: {}",
      self.shader_resource_min_lod()
    )?;
    writeln!(f, "  sparseBinding: {}", self.sparse_binding())?;
    writeln!(
      f,
      "  sparseResidencyBuffer: {}",
      self.sparse_residency_buffer()
    )?;
    writeln!(
      f,
      "  sparseResidencyImage2D: {}",
      self.sparse_residency_image2d()
    )?;
    writeln!(
      f,
      "  sparseResidencyImage3D: {}",
      self.sparse_residency_image3d()
    )?;
    writeln!(
      f,
      "  sparseResidency2Samples: {}",
      self.sparse_residency2_samples()
    )?;
    writeln!(
      f,
      "  sparseResidency4Samples: {}",
      self.sparse_residency4_samples()
    )?;
    writeln!(
      f,
      "  sparseResidency8Samples: {}",
      self.sparse_residency8_samples()
    )?;
    writeln!(
      f,
      "  sparseResidency16Samples: {}",
      self.sparse_residency16_samples()
    )?;
    writeln!(
      f,
      "  sparseResidencyAliased: {}",
      self.sparse_residency_aliased()
    )?;
    writeln!(
      f,
      "  variableMultisampleRate: {}",
      self.variable_multisample_rate()
    )?;
    writeln!(f, "  inheritedQueries: {}", self.inherited_queries())?;
    Ok(())
  }
}
