//! Kullback-Leibler divergence.

use crate::error::InfoError;
use std::collections::HashMap;
use std::hash::Hash;

/// KL divergence $D_{KL}(P \| Q)$ in bits.
///
/// Measures how much the distribution induced by `p` differs from the one
/// induced by `q`. Note that KL divergence is **not symmetric**.
///
/// Returns an error if:
/// - `p` or `q` are empty
/// - `p` contains a value absent from `q` (divergence would be infinite)
pub fn kl_divergence<T>(p: &[T], q: &[T]) -> Result<f64, InfoError>
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

    let mut kl = 0.0f64;
    for (x, &pc) in &p_counts {
        let p_x = pc as f64 / p_total;
        match q_counts.get(x) {
            Some(&qc) => {
                let q_x = qc as f64 / q_total;
                kl += p_x * (p_x / q_x).log2();
            }
            None => return Err(InfoError::UndefinedDivergence),
        }
    }

    Ok(kl)
}

/// Panic-on-error convenience variant.
pub fn kl_divergence_unchecked<T>(p: &[T], q: &[T]) -> f64
where
    T: Eq + Hash,
{
    kl_divergence(p, q).expect("kl_divergence failed")
}
