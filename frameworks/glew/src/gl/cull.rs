use crate::gl::error::Result;

use super::Capability;

/// An enumeration of the face to cull
#[derive(Copy, Clone, Debug)]
pub enum CullFace {
  /// Cull the front face
  Front = crate::c::GL_FRONT as isize,
  /// Cull the back face
  Back = crate::c::GL_BACK as isize,
  /// Cull both the front and back face
  FrontAndBack = crate::c::GL_FRONT_AND_BACK as isize,
}

impl std::fmt::Display for CullFace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Front => "Front",
      Self::Back => "Back",
      Self::FrontAndBack => "FrontAndBack",
    }
    .fmt(f)
  }
}

impl CullFace {
  /// Returns the face from the given enumeration
  ///
  /// # Arguments
  ///
  /// * `e` - The enumeration to get the face from
  pub const fn from_enum(e: crate::gl::Enum) -> Option<Self> {
    match e {
      crate::c::GL_FRONT => Some(Self::Front),
      crate::c::GL_BACK => Some(Self::Back),
      crate::c::GL_FRONT_AND_BACK => Some(Self::FrontAndBack),
      _ => None,
    }
  }

  /// Returns the face from the given enumeration without checking
  ///
  /// # Arguments
  ///
  /// * `e` - The enumeration to get the face from
  ///
  /// # Safety
  ///
  /// The enumeration must be one of the valid faces
  pub const unsafe fn from_enum_unchecked(e: crate::gl::Enum) -> Self {
    match e {
      crate::c::GL_FRONT => Self::Front,
      crate::c::GL_BACK => Self::Back,
      crate::c::GL_FRONT_AND_BACK => Self::FrontAndBack,
      _ => unreachable!(),
    }
  }

  /// Returns the value of the face
  #[inline]
  pub const fn value(self) -> crate::c::GLenum {
    self as crate::c::GLenum
  }
}

/// Enables polygon culling.
#[inline(always)]
pub fn enable_cull_face() {
  Capability::CullFace.enable();
}

/// Disables polygon culling.
#[inline(always)]
pub fn disable_cull_face() {
  Capability::CullFace.disable();
}

/// Toggles polygon culling.
#[inline(always)]
pub fn toggle_cull_face() {
  Capability::CullFace.toggle();
}

/// Specifies which face to cull (front, back, or both)
///
/// # Arguments
///
/// * `face` - The face to cull
#[inline]
pub fn set_cull_face(face: CullFace) {
  unsafe { crate::c::glCullFace(face as u32) }
}

/// Specifies which face to cull (front, back, or both), and checks for errors.
///
/// This is a convenience-wrapper around [`set_cull_face`] and
/// [`crate::gl::error::check`].
///
/// # Arguments
///
/// * `face` - The face to cull
#[inline]
pub fn set_cull_face_checked(face: CullFace) -> Result<()> {
  crate::gl::error::check(|| set_cull_face(face))
}

/// Queries whether whether polygon culling is enabled.
#[inline(always)]
pub fn is_cull_face_enabled() -> bool {
  Capability::CullFace.is_enabled()
}

/// Returns the current face to cull.
///
/// If face culling is disabled, returns [`None`]
pub fn cull_face() -> Option<CullFace> {
  if !is_cull_face_enabled() {
    None
  } else {
    Some(unsafe { cull_face_unchecked() })
  }
}

/// Returns the current face to cull without checking if cull face is enabled.
///
/// # Safety
///
/// The cull face must be enabled.
#[inline]
pub unsafe fn cull_face_unchecked() -> CullFace {
  let mut value = 0;
  unsafe {
    crate::c::glGetIntegerv(crate::c::GL_CULL_FACE_MODE, &mut value);
  }
  unsafe { CullFace::from_enum_unchecked(value as crate::gl::Enum) }
}
