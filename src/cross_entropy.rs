//! Cross-entropy.

use crate::error::InfoError;
use std::collections::HashMap;
use std::hash::Hash;

/// Cross-entropy $H(P, Q)$ in bits.
///
/// $H(P,Q) = H(P) + D_{KL}(P \| Q)$
///
/// Measures the average number of bits needed to encode outcomes of `p` using
/// the distribution of `q`. Always $\geq H(P)$, with equality iff $P = Q$.
///
/// Returns an error if:
/// - `p` or `q` are empty
/// - `p` contains a value absent from `q` (cross-entropy would be infinite)
pub fn cross_entropy<T>(p: &[T], q: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash,
{
    if p.is_empty() || q.is_empty() {
        return Err(InfoError::EmptyInput);
    }

    let mut p_counts: HashMap<&T, usize> = HashMap::new();
    for x in p {
        *p_counts.entry(x).or_insert(0) += 1;
    }

    let mut q_counts: HashMap<&T, usize> = HashMap::new();
    for x in q {
        *q_counts.entry(x).or_insert(0) += 1;
    }

    let p_total = p.len() as f64;
    let q_total = q.len() as f64;

    let mut h = 0.0f64;
    for (x, &pc) in &p_counts {
        let p_x = pc as f64 / p_total;
        match q_counts.get(x) {
            Some(&qc) => {
                let q_x = qc as f64 / q_total;
                h -= p_x * q_x.log2();
            }
            None => return Err(InfoError::UndefinedDivergence),
        }
    }

    Ok(h)
}

/// Panic-on-error convenience variant.
pub fn cross_entropy_unchecked<T>(p: &[T], q: &[T]) -> f64
where
    T: Eq + Hash,
{
    cross_entropy(p, q).expect("cross_entropy failed")
}
