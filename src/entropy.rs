//! Shannon entropy.

use crate::error::InfoError;
use std::collections::HashMap;
use std::hash::Hash;

/// Shannon entropy H(X) in bits.
///
/// Returns an error if `data` is empty.
pub fn entropy<T>(data: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash,
{
    if data.is_empty() {
        return Err(InfoError::EmptyInput);
    }

    let mut counts = HashMap::new();
    for item in data {
        *counts.entry(item).or_insert(0usize) += 1;
    }
    let total = data.len() as f64;

    let h = counts
        .values()
        .map(|&count| {
            let p = count as f64 / total;
            -p * p.log2()
        })
        .sum();

    Ok(h)
}

/// Panic-on-error convenience variant of [`entropy`].
pub fn entropy_unchecked<T>(data: &[T]) -> f64
where
    T: Eq + Hash,
{
    entropy(data).expect("entropy failed")
}
