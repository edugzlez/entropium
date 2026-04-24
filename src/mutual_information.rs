//! Mutual information between two random variables.

use crate::error::InfoError;
use std::collections::HashMap;
use std::hash::Hash;

/// Mutual information I(X;Y) in bits.
///
/// Returns an error if:
/// - `x` or `y` are empty
/// - input lengths do not match
pub fn mutual_information<T>(x: &[T], y: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash,
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

    let mut x_counts = HashMap::new();
    let mut y_counts = HashMap::new();
    let mut joint_counts = HashMap::new();
    for (a, b) in x.iter().zip(y.iter()) {
        *x_counts.entry(a).or_insert(0usize) += 1;
        *y_counts.entry(b).or_insert(0usize) += 1;
        *joint_counts.entry((a, b)).or_insert(0usize) += 1;
    }
    let total = x.len() as f64;

    let mi = joint_counts
        .iter()
        .map(|(&(a, b), &joint_count)| {
            let p_xy = joint_count as f64 / total;
            let p_x = *x_counts.get(a).expect("x marginal missing") as f64 / total;
            let p_y = *y_counts.get(b).expect("y marginal missing") as f64 / total;
            p_xy * (p_xy / (p_x * p_y)).log2()
        })
        .sum();

    Ok(mi)
}

/// Panic-on-error convenience variant of [`mutual_information`].
pub fn mutual_information_unchecked<T>(x: &[T], y: &[T]) -> f64
where
    T: Eq + Hash,
{
    mutual_information(x, y).expect("mutual_information failed")
}
