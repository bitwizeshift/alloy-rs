//! This module provides interpolation logic for the core systems.

use crate::ops::Lerp;
use std::ops::{Add, Mul};

/// Linearly interpolates between two values.
///
/// # Arguments
///
/// * `start` - The starting state.
/// * `end` - The ending state.
/// * `alpha` - The interpolation factor.
pub fn linear<T, U>(start: &T, end: &U, alpha: f32) -> T::Output
where
  T: Lerp<U>,
{
  start.lerp(end, alpha)
}

/// Quadratically interpolates between three values.
///
/// # Arguments
///
/// * `start` - The starting state.
/// * `intermediate` - The intermediate value.
/// * `end` - The end state.
/// * `alpha` - The interpolation factor.
pub fn quadratic<T>(start: &T, intermediate: &T, end: &T, alpha: f32) -> T
where
  T: Lerp<Output = T> + Mul<f32, Output = T> + Add<T, Output = T>,
{
  let alpha_sq = alpha * alpha;
  let alpha_inv = 1.0 - alpha;
  let alpha_inv_sq = alpha_inv * alpha_inv;

  start.lerp(intermediate, 2.0 * alpha * alpha_inv) * alpha_inv_sq
    + intermediate.lerp(end, 2.0 * alpha * alpha_inv) * alpha_sq
}

/// Interpolates between two values using a circular interpolation.
///
/// # Arguments
///
/// * `start` - The starting state.
/// * `end` - The ending state.
/// * `alpha` - The interpolation factor.
pub fn circular<T, U>(start: &T, end: &U, alpha: f32) -> T::Output
where
  T: Lerp<U>,
{
  start.lerp(end, 1.0 - (1.0 - (alpha * alpha)).sqrt())
}

/// Interpolates between four values using a bilinear interpolation.
///
/// # Arguments
///
/// * `v00` - The value at the top-left corner.
/// * `v10` - The value at the top-right corner.
/// * `v01` - The value at the bottom-left corner.
/// * `v11` - The value at the bottom-right corner.
/// * `tx` - The interpolation factor along the x-axis.
/// * `ty` - The interpolation factor along the y-axis.
pub fn bilinear<T0, T1, T2, T3, R>(v00: &T0, v10: &T1, v01: &T2, v11: &T3, tx: f32, ty: f32) -> R
where
  T0: Lerp<T1>,
  T2: Lerp<T3>,
  T0::Output: Lerp<T2::Output, Output = R>,
{
  linear(&linear(v00, v10, tx), &linear(v01, v11, tx), ty)
}

/// Interpolates between two values using a sine interpolation.
///
/// # Arguments
///
/// * `start` - The starting state.
/// * `end` - The ending state.
/// * `alpha` - The interpolation factor.
pub fn sine<T, U>(start: &T, end: &U, alpha: f32) -> T::Output
where
  T: Lerp<U>,
{
  let alpha = (alpha * std::f32::consts::PI).sin() * 0.5 + 0.5;
  start.lerp(end, alpha)
}

/// Interpolates between two values using a cosine interpolation.
///
/// # Arguments
///
/// * `start` - The starting state.
/// * `end` - The ending state.
/// * `alpha` - The interpolation factor.
pub fn cosine<T, U>(start: &T, end: &U, alpha: f32) -> T::Output
where
  T: Lerp<U>,
{
  let alpha = (alpha * std::f32::consts::PI).cos() * 0.5 + 0.5;
  start.lerp(end, alpha)
}

/// Interpolates between two values using a smoothstep interpolation.
///
/// # Arguments
///
/// * `start` - The starting state.
/// * `end` - The ending state.
/// * `alpha` - The interpolation factor.
pub fn smoothstep<T, U>(start: &T, end: &U, alpha: f32) -> T::Output
where
  T: Lerp<U>,
{
  let alpha = alpha * alpha * (3.0 - 2.0 * alpha);
  start.lerp(end, alpha)
}
