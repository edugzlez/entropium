//! Conditional entropy of one random variable given another.

use crate::entropy::entropy;
use crate::error::InfoError;
use crate::joint_entropy::joint_entropy;
use std::hash::Hash;

/// Conditional entropy H(X|Y) in bits.
///
/// Returns an error if:
/// - `x` or `y` are empty
/// - input lengths do not match
pub fn conditional_entropy<X, Y>(x: &[X], y: &[Y]) -> Result<f64, InfoError>
where
    X: Eq + Hash,
    Y: Eq + Hash,
{
    let h_xy = joint_entropy(x, y)?;
    let h_y = entropy(y)?;
    Ok(h_xy - h_y)
}

/// Panic-on-error convenience variant.
pub fn conditional_entropy_unchecked<X, Y>(x: &[X], y: &[Y]) -> f64
where
    X: Eq + Hash,
    Y: Eq + Hash,
{
    conditional_entropy(x, y).expect("conditional_entropy failed")
}
