//! Jensen-Shannon divergence.

use crate::error::InfoError;
use std::collections::HashMap;
use std::hash::Hash;

/// Jensen-Shannon divergence $JSD(P \| Q)$ in bits.
///
/// $JSD(P \| Q) = \frac{1}{2} D_{KL}(P \| M) + \frac{1}{2} D_{KL}(Q \| M)$
/// where $M = \frac{P + Q}{2}$.
///
/// Always well-defined and bounded in $[0, 1]$ bits. Symmetric: $JSD(P\|Q) = JSD(Q\|P)$.
///
/// Returns `InfoError::EmptyInput` if either slice is empty.
pub fn js_divergence<T>(p: &[T], q: &[T]) -> Result<f64, InfoError>
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

    // KL(P || M): iterate over support of P
    let mut kl_p_m = 0.0f64;
    for (x, &pc) in &p_counts {
        let p_x = pc as f64 / p_total;
        let q_x = q_counts.get(x).map_or(0.0, |&qc| qc as f64 / q_total);
        let m_x = 0.5 * (p_x + q_x); // always > 0 since p_x > 0
        kl_p_m += p_x * (p_x / m_x).log2();
    }

    // KL(Q || M): iterate over support of Q
    let mut kl_q_m = 0.0f64;
    for (x, &qc) in &q_counts {
        let q_x = qc as f64 / q_total;
        let p_x = p_counts.get(x).map_or(0.0, |&pc| pc as f64 / p_total);
        let m_x = 0.5 * (p_x + q_x); // always > 0 since q_x > 0
        kl_q_m += q_x * (q_x / m_x).log2();
    }

    Ok(0.5 * (kl_p_m + kl_q_m))
}

/// Panic-on-error convenience variant.
pub fn js_divergence_unchecked<T>(p: &[T], q: &[T]) -> f64
where
    T: Eq + Hash,
{
    js_divergence(p, q).expect("js_divergence failed")
}
