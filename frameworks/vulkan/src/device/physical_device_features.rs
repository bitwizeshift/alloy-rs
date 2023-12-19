use vulkan_sys as c;

/// A wrapper around [`VkPhysicalDeviceFeatures`] objects.
///
/// [`VkPhysicalDeviceFeatures`]: c::VkPhysicalDeviceFeatures
#[repr(transparent)]
pub struct PhysicalDeviceFeatures(c::VkPhysicalDeviceFeatures);

foundation::define_transparent! {
  PhysicalDeviceFeatures.0 => c::VkPhysicalDeviceFeatures
}

impl From<c::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
  fn from(value: c::VkPhysicalDeviceFeatures) -> Self {
    Self(value)
  }
}

impl From<PhysicalDeviceFeatures> for c::VkPhysicalDeviceFeatures {
  fn from(value: PhysicalDeviceFeatures) -> Self {
    *value.as_ref()
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
    f.debug_struct("PhysicalDeviceFeatures")
      .field("robust_buffer_access", &self.robust_buffer_access())
      .field("full_draw_index_uint32", &self.full_draw_index_uint32())
      .field("image_cube_array", &self.image_cube_array())
      .field("independent_blend", &self.independent_blend())
      .field("geometry_shader", &self.geometry_shader())
      .field("tessellation_shader", &self.tessellation_shader())
      .field("sample_rate_shading", &self.sample_rate_shading())
      .field("dual_src_blend", &self.dual_src_blend())
      .field("logic_op", &self.logic_op())
      .field("multi_draw_indirect", &self.multi_draw_indirect())
      .field(
        "draw_indirect_first_instance",
        &self.draw_indirect_first_instance(),
      )
      .field("depth_clamp", &self.depth_clamp())
      .field("depth_bias_clamp", &self.depth_bias_clamp())
      .field("fill_mode_non_solid", &self.fill_mode_non_solid())
      .field("depth_bounds", &self.depth_bounds())
      .field("wide_lines", &self.wide_lines())
      .field("large_points", &self.large_points())
      .field("alpha_to_one", &self.alpha_to_one())
      .field("multi_viewport", &self.multi_viewport())
      .field("sampler_anisotropy", &self.sampler_anisotropy())
      .field("texture_compression_etc2", &self.texture_compression_etc2())
      .field(
        "texture_compression_astc_ldr",
        &self.texture_compression_astc_ldr(),
      )
      .field("texture_compression_bc", &self.texture_compression_bc())
      .field("occlusion_query_precise", &self.occlusion_query_precise())
      .field(
        "pipeline_statistics_query",
        &self.pipeline_statistics_query(),
      )
      .field(
        "vertex_pipeline_stores_and_atomics",
        &self.vertex_pipeline_stores_and_atomics(),
      )
      .field(
        "fragment_stores_and_atomics",
        &self.fragment_stores_and_atomics(),
      )
      .field(
        "shader_tessellation_and_geometry_point_size",
        &self.shader_tessellation_and_geometry_point_size(),
      )
      .field(
        "shader_image_gather_extended",
        &self.shader_image_gather_extended(),
      )
      .field(
        "shader_storage_image_extended_formats",
        &self.shader_storage_image_extended_formats(),
      )
      .field(
        "shader_storage_image_multisample",
        &self.shader_storage_image_multisample(),
      )
      .field(
        "shader_storage_image_read_without_format",
        &self.shader_storage_image_read_without_format(),
      )
      .field(
        "shader_storage_image_write_without_format",
        &self.shader_storage_image_write_without_format(),
      )
      .field(
        "shader_uniform_buffer_array_dynamic_indexing",
        &self.shader_uniform_buffer_array_dynamic_indexing(),
      )
      .field(
        "shader_sampled_image_array_dynamic_indexing",
        &self.shader_sampled_image_array_dynamic_indexing(),
      )
      .field(
        "shader_storage_buffer_array_dynamic_indexing",
        &self.shader_storage_buffer_array_dynamic_indexing(),
      )
      .field(
        "shader_storage_image_array_dynamic_indexing",
        &self.shader_storage_image_array_dynamic_indexing(),
      )
      .field("shader_clip_distance", &self.shader_clip_distance())
      .field("shader_cull_distance", &self.shader_cull_distance())
      .field("shader_float64", &self.shader_float64())
      .field("shader_int64", &self.shader_int64())
      .field("shader_int16", &self.shader_int16())
      .field(
        "shader_resource_residency",
        &self.shader_resource_residency(),
      )
      .field("shader_resource_min_lod", &self.shader_resource_min_lod())
      .field("sparse_binding", &self.sparse_binding())
      .field("sparse_residency_buffer", &self.sparse_residency_buffer())
      .field("sparse_residency_image2d", &self.sparse_residency_image2d())
      .field("sparse_residency_image3d", &self.sparse_residency_image3d())
      .field(
        "sparse_residency2_samples",
        &self.sparse_residency2_samples(),
      )
      .field(
        "sparse_residency4_samples",
        &self.sparse_residency4_samples(),
      )
      .field(
        "sparse_residency8_samples",
        &self.sparse_residency8_samples(),
      )
      .field(
        "sparse_residency16_samples",
        &self.sparse_residency16_samples(),
      )
      .field("sparse_residency_aliased", &self.sparse_residency_aliased())
      .field(
        "variable_multisample_rate",
        &self.variable_multisample_rate(),
      )
      .field("inherited_queries", &self.inherited_queries())
      .finish()
  }
}
