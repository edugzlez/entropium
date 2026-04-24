//! Joint entropy of two random variables.

use crate::error::InfoError;
use std::collections::HashMap;
use std::hash::Hash;

/// Joint entropy H(X,Y) in bits.
///
/// Returns an error if:
/// - `x` or `y` are empty
/// - input lengths do not match
pub fn joint_entropy<X, Y>(x: &[X], y: &[Y]) -> Result<f64, InfoError>
where
    X: Eq + Hash,
    Y: Eq + Hash,
{
    if x.is_empty() || y.is_empty() {
        return Err(InfoError::EmptyInput);
    }

    if x.len() != y.len() {
        return Err(InfoError::LengthMismatch {
            left: x.len(),
            right: y.len(),
        });
    }

    let mut joint_counts = HashMap::new();
    for (a, b) in x.iter().zip(y.iter()) {
        *joint_counts.entry((a, b)).or_insert(0usize) += 1;
    }

    let total = x.len() as f64;
    let h_xy = joint_counts
        .values()
        .map(|&joint_count| {
            let p = joint_count as f64 / total;
            -p * p.log2()
        })
        .sum();

    Ok(h_xy)
}

/// Panic-on-error convenience variant.
pub fn joint_entropy_unchecked<X, Y>(x: &[X], y: &[Y]) -> f64
where
    X: Eq + Hash,
    Y: Eq + Hash,
{
    joint_entropy(x, y).expect("joint_entropy failed")
}
