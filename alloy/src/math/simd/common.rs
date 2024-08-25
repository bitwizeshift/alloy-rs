use crate::core::AlignedArray;

/// A 128-bit SIMD vector with 4 [f32] components.
pub type F32x4Array = AlignedArray<f32, 4, 16>;
