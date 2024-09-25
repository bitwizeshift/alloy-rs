//! This module provide information about the current GL context.
//!
//! The functions provided in here map largely 1:1 to OpenGL calls to [`glGet`]
//! in varying capacities.
//!
//! [`glGet`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGet.xhtml
use crate::c::GLenum;
use crate::error::Result;

/// Gets the supported version of OpenGL as defined by the `GL_MAJOR_VERSION`
/// and `GL_MINOR_VERSION` values returned from the current context.
///
/// The first returned value is the major version and the second is the minor.
pub fn supported_version() -> (u32, u32) {
  let mut major = 0;
  let mut minor = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAJOR_VERSION as GLenum, &mut major);
    crate::c::glGetIntegerv(crate::c::GL_MINOR_VERSION as GLenum, &mut minor);
  }
  (major as u32, minor as u32)
}

/// Gets the supported version of OpenGL as defined by the `GL_MAJOR_VERSION`
/// and `GL_MINOR_VERSION` values returned from the current context.
///
/// This function always tests whether an error was returned from [`glGetError`]
/// after.
///
/// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
#[inline(always)]
pub fn supported_version_checked() -> Result<(u32, u32)> {
  crate::error::check(supported_version)
}

/// Gets the maximum number of layers allowed in an array texture, and must be
/// at least 256.
pub fn max_array_texture_layers() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_ARRAY_TEXTURE_LAYERS as GLenum, &mut size);
  }
  size as usize
}

/// Gets the maximum number of layers allowed in an array texture, and must be
/// at least 256.
///
/// This function always tests whether an error was returned from [`glGetError`]
/// after.
///
/// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
#[inline(always)]
pub fn max_array_texture_layers_checked() -> Result<usize> {
  crate::error::check(max_array_texture_layers)
}

/// Gets a rough estimate of the largest 3D texture that the GL can handle. The
/// value must be at least 64.
pub fn max_3d_texture_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_3D_TEXTURE_SIZE as GLenum, &mut size);
  }
  size as usize
}

/// Gets a rough estimate of the largest 3D texture that the GL can handle. The
/// value must be at least 64.
///
/// This function always tests whether an error was returned from [`glGetError`]
/// after.
///
/// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
#[inline(always)]
pub fn max_3d_texture_size_checked() -> Result<usize> {
  crate::error::check(max_3d_texture_size)
}

/// Gets the maximum number of texture units that can be used in a fragment
/// shader.
pub fn max_texture_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_TEXTURE_SIZE as GLenum, &mut size);
  }
  size as usize
}

/// Gets the maximum number of texture units that can be used in a fragment
/// shader.
///
/// This function always tests whether an error was returned from [`glGetError`]
/// after.
///
/// [`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
#[inline(always)]
pub fn max_texture_size_checked() -> Result<usize> {
  crate::error::check(max_texture_size)
}

/// Gets the maximum number of texture units that can be used in a fragment
/// shader.
pub fn max_clip_distances() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_CLIP_DISTANCES as GLenum, &mut size);
  }
  size as usize
}

/// Gets the maximum number of samples in a color multisample texture.
pub fn max_color_texture_samples() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_COLOR_TEXTURE_SAMPLES as GLenum, &mut size);
  }
  size as usize
}

/// Gets the maximum number of atomic counters available to all active shaders.
pub fn max_combined_atomic_counters() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_COMBINED_ATOMIC_COUNTERS as GLenum,
      &mut size,
    );
  }
  size as usize
}

/// Gets the number of words for fragment shader uniform variables in all
/// uniform blocks (including default). The value must be at least 1.
pub fn max_combined_fragment_uniform_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

/// Gets the number of words for geometry shader uniform variables in all
/// uniform blocks (including default). The value must be at least 1.
pub fn max_combined_geometry_uniform_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

/// Gets the maximum supported texture image units that can be used to access
/// texture maps from the vertex shader and the fragment processor combined. If
/// both the vertex shader and the fragment processing stage access the same
/// texture image unit, then that counts as using two texture image units
/// against this limit. The value must be at least 48.
pub fn max_combined_texture_image_units() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS as GLenum,
      &mut size,
    );
  }
  size as usize
}

/// Gets the maximum number of uniform blocks per program. The value must be at
/// least 70.
pub fn max_combined_uniform_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_COMBINED_UNIFORM_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

/// Gets the number of words for vertex shader uniform variables in all uniform
/// blocks (including default). The value must be at least 1.
pub fn max_combined_vertex_uniform_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

/// Gets the maximum number of color attachments that can be used in a fragment
/// shader.
pub fn max_color_attachments() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_COLOR_ATTACHMENTS as GLenum, &mut size);
  }
  size as usize
}

/// Gets the max cube map texture size.
pub fn max_cube_map_texture_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_CUBE_MAP_TEXTURE_SIZE as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_depth_texture_samples() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_DEPTH_TEXTURE_SAMPLES as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_draw_buffers() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_DRAW_BUFFERS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_dual_source_draw_buffers() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_DUAL_SOURCE_DRAW_BUFFERS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_elements_indices() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_ELEMENTS_INDICES as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_elements_vertices() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_ELEMENTS_VERTICES as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_fragment_shader_storage_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_fragment_input_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_FRAGMENT_INPUT_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_fragment_uniform_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_fragment_uniform_vectors() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_FRAGMENT_UNIFORM_VECTORS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_fragment_uniform_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_FRAGMENT_UNIFORM_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_framebuffer_width() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_FRAMEBUFFER_WIDTH as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_framebuffer_height() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_FRAMEBUFFER_HEIGHT as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_framebuffer_layers() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_FRAMEBUFFER_LAYERS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_framebuffer_samples() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_FRAMEBUFFER_SAMPLES as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_geometry_atomic_counters() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_ATOMIC_COUNTERS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_shader_storage_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_input_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_INPUT_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_output_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_OUTPUT_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_output_vertices() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_OUTPUT_VERTICES as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_texture_image_units() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_uniform_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_UNIFORM_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_geometry_uniform_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_GEOMETRY_UNIFORM_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_integer_samples() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_INTEGER_SAMPLES as GLenum, &mut size);
  }
  size as usize
}

///
pub fn min_map_buffer_alignment() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MIN_MAP_BUFFER_ALIGNMENT as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_label_length() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_LABEL_LENGTH as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_program_texel_offset() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_PROGRAM_TEXEL_OFFSET as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_rectangle_texture_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_RECTANGLE_TEXTURE_SIZE as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_renderbuffer_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_RENDERBUFFER_SIZE as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_sample_mask_words() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_SAMPLE_MASK_WORDS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_server_wait_timeout() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_SERVER_WAIT_TIMEOUT as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_shader_storage_buffer_bindings() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_tess_control_atomic_counters() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_tess_evaluation_atomic_counters() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_tess_control_shader_storage_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_tess_evaluation_shader_storage_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_texture_buffer_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_TEXTURE_BUFFER_SIZE as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_texture_image_units() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_TEXTURE_IMAGE_UNITS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_texture_lod_bias() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_TEXTURE_LOD_BIAS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_uniform_buffer_bindings() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_UNIFORM_BUFFER_BINDINGS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_uniform_block_size() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_UNIFORM_BLOCK_SIZE as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_uniform_locations() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_UNIFORM_LOCATIONS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_varying_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VARYING_COMPONENTS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_varying_vectors() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VARYING_VECTORS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_varying_floats() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VARYING_FLOATS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_vertex_attribs() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VERTEX_ATTRIBS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_vertex_shader_storage_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_vertex_texture_image_units() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_vertex_uniform_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_VERTEX_UNIFORM_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_vertex_uniform_vectors() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VERTEX_UNIFORM_VECTORS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_vertex_output_components() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(
      crate::c::GL_MAX_VERTEX_OUTPUT_COMPONENTS as GLenum,
      &mut size,
    );
  }
  size as usize
}

///
pub fn max_vertex_uniform_blocks() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VERTEX_UNIFORM_BLOCKS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_viewport_dimensions() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VIEWPORT_DIMS as GLenum, &mut size);
  }
  size as usize
}

///
pub fn max_viewports() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_MAX_VIEWPORTS as GLenum, &mut size);
  }
  size as usize
}

/// Gets the number of extensions supported by the current GL context.
///
/// This function is a convenience wrapper around `glGetIntegerv` with the
/// `GL_NUM_EXTENSIONS` parameter.
pub fn num_extensions() -> usize {
  let mut size = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_NUM_EXTENSIONS as GLenum, &mut size);
  }
  size as usize
}

/// Returns the name of the extension at the given index.
///
/// # Parameters
///
/// * `i` - the index of the extension to get
pub fn extension(i: usize) -> Option<&'static str> {
  let ext = unsafe { crate::c::glGetStringi(crate::c::GL_EXTENSIONS as _, i as _) };
  if ext.is_null() {
    None
  } else {
    Some(unsafe { std::ffi::CStr::from_ptr(ext as _).to_str().unwrap() })
  }
}

/// Returns an iterator over all the extensions supported by the current GL
/// context.
pub fn extensions() -> ExtensionIter {
  let count = num_extensions();

  ExtensionIter::new(count)
}

/// Gets a string for the shading language version.
pub fn shading_language_version() -> &'static str {
  let version = unsafe { crate::c::glGetString(crate::c::GL_SHADING_LANGUAGE_VERSION as _) };
  unsafe { std::ffi::CStr::from_ptr(version as _).to_str().unwrap() }
}

/// Gets a string for the vendor name.
pub fn vendor() -> &'static str {
  let vendor = unsafe { crate::c::glGetString(crate::c::GL_VENDOR as _) };
  unsafe { std::ffi::CStr::from_ptr(vendor as _).to_str().unwrap() }
}

/// Gets a string for the renderer name.
pub fn renderer() -> &'static str {
  let renderer = unsafe { crate::c::glGetString(crate::c::GL_RENDERER as _) };
  unsafe { std::ffi::CStr::from_ptr(renderer as _).to_str().unwrap() }
}

/// Gets a string for the version.
pub fn version() -> &'static str {
  let version = unsafe { crate::c::glGetString(crate::c::GL_VERSION as _) };
  unsafe { std::ffi::CStr::from_ptr(version as _).to_str().unwrap() }
}

/// An iterator over the extensions supported by the current GL context.
pub struct ExtensionIter {
  index: usize,
  count: usize,
}

impl ExtensionIter {
  #[inline(always)]
  const fn new(count: usize) -> Self {
    Self { index: 0, count }
  }
}

impl Iterator for ExtensionIter {
  type Item = &'static str;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.count {
      let ext = extension(self.index);
      self.index += 1;
      ext
    } else {
      None
    }
  }
}
