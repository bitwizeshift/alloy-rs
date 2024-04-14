/// An enumeration of capabilities that can be tested against.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum Capability {
  Blend = crate::c::GL_BLEND as isize,
  // TODO: this might be better as ClipDistance(n) where n is the index and adds
  // on to GL_CLIP_DISTANCE0. This will require changing this enum to no longer
  // be an integral representation, and would require an additional level of
  // mapping somewhere in a function.
  ClipDistance0 = crate::c::GL_CLIP_DISTANCE0 as isize,
  ClipDistance1 = crate::c::GL_CLIP_DISTANCE1 as isize,
  ClipDistance2 = crate::c::GL_CLIP_DISTANCE2 as isize,
  ClipDistance3 = crate::c::GL_CLIP_DISTANCE3 as isize,
  ClipDistance4 = crate::c::GL_CLIP_DISTANCE4 as isize,
  ClipDistance5 = crate::c::GL_CLIP_DISTANCE5 as isize,
  ColorLogicOp = crate::c::GL_COLOR_LOGIC_OP as isize,
  CullFace = crate::c::GL_CULL_FACE as isize,
  DepthTest = crate::c::GL_DEPTH_TEST as isize,
  Dither = crate::c::GL_DITHER as isize,
  FramebufferSRGB = crate::c::GL_FRAMEBUFFER_SRGB as isize,
  LineSmooth = crate::c::GL_LINE_SMOOTH as isize,
  Multisample = crate::c::GL_MULTISAMPLE as isize,
  PolygonOffsetFill = crate::c::GL_POLYGON_OFFSET_FILL as isize,
  PolygonOffsetLine = crate::c::GL_POLYGON_OFFSET_LINE as isize,
  PolygonOffsetPoint = crate::c::GL_POLYGON_OFFSET_POINT as isize,
  PolygonSmooth = crate::c::GL_POLYGON_SMOOTH as isize,
  PrimitiveRestart = crate::c::GL_PRIMITIVE_RESTART as isize,
  PrimitiveRestartFixedIndex = crate::c::GL_PRIMITIVE_RESTART_FIXED_INDEX as isize,
  RasterizerDiscard = crate::c::GL_RASTERIZER_DISCARD as isize,
  SampleAlphaToCoverage = crate::c::GL_SAMPLE_ALPHA_TO_COVERAGE as isize,
  SampleAlphaToOne = crate::c::GL_SAMPLE_ALPHA_TO_ONE as isize,
  SampleCoverage = crate::c::GL_SAMPLE_COVERAGE as isize,
  SampleShading = crate::c::GL_SAMPLE_SHADING as isize,
  SampleMask = crate::c::GL_SAMPLE_MASK as isize,
  ScissorTest = crate::c::GL_SCISSOR_TEST as isize,
  StencilTest = crate::c::GL_STENCIL_TEST as isize,
  TextureCubeMapSeamless = crate::c::GL_TEXTURE_CUBE_MAP_SEAMLESS as isize,
}

impl Capability {
  /// Returns the value of the capability
  #[inline]
  pub const fn value(self) -> crate::c::GLenum {
    self as crate::c::GLenum
  }

  /// Queries if the capability is enabled.
  #[inline]
  pub fn is_enabled(self) -> bool {
    unsafe { crate::c::glIsEnabled(self.value()) != 0 }
  }

  /// Enables the capability.
  #[inline]
  pub fn enable(self) {
    unsafe { crate::c::glEnable(self.value()) }
  }

  /// Disables the capability.
  #[inline]
  pub fn disable(self) {
    unsafe { crate::c::glDisable(self.value()) }
  }

  /// Toggles the capability.
  #[inline]
  pub fn toggle(self) {
    if self.is_enabled() {
      self.disable();
    } else {
      self.enable();
    }
  }
}
