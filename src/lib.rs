//! Information-theory primitives: entropy, joint entropy, conditional entropy,
//! and mutual information. All values are in bits (base-2 logarithm).
//!
//! Every function comes in two flavours:
//! - A checked variant that returns `Result<f64, InfoError>`.
//! - An `_unchecked` variant that panics on error, intended for quick
//!   prototyping or when the caller has already validated its inputs.

mod conditional_entropy;
mod cross_entropy;
mod entropy;
mod error;
mod joint_entropy;
mod js_divergence;
mod kl_divergence;
mod mutual_information;

pub use conditional_entropy::{conditional_entropy, conditional_entropy_unchecked};
pub use cross_entropy::{cross_entropy, cross_entropy_unchecked};
pub use entropy::{entropy, entropy_unchecked};
pub use error::InfoError;
pub use joint_entropy::{joint_entropy, joint_entropy_unchecked};
pub use js_divergence::{js_divergence, js_divergence_unchecked};
pub use kl_divergence::{kl_divergence, kl_divergence_unchecked};
pub use mutual_information::{mutual_information, mutual_information_unchecked};

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64, eps: f64) {
        assert!(
            (a - b).abs() < eps,
            "left={a}, right={b}, diff={}",
            (a - b).abs()
        );
    }

    #[test]
    fn entropy_of_constant_is_zero() {
        let data = vec![1, 1, 1, 1, 1];
        let h = entropy(&data).unwrap();
        assert_close(h, 0.0, 1e-12);
    }

    #[test]
    fn entropy_of_fair_bit_is_one() {
        let data = vec![0, 1, 0, 1];
        let h = entropy(&data).unwrap();
        assert_close(h, 1.0, 1e-12);
    }

    #[test]
    fn entropy_of_three_symbols_two_one_one() {
        let data = vec![(1, 2), (1, 2), (1, 3)];
        let h = entropy(&data).unwrap();

        let expected = -((2.0 / 3.0) * (2.0f64 / 3.0).log2() + (1.0 / 3.0) * (1.0f64 / 3.0).log2());
        assert_close(h, expected, 1e-12);
    }

    #[test]
    fn entropy_empty_fails() {
        let data: Vec<u32> = vec![];
        let err = entropy(&data).unwrap_err();
        assert_eq!(err, InfoError::EmptyInput);
    }

    #[test]
    fn mutual_information_of_independent_variables_is_zero() {
        let x = vec![0, 0, 1, 1];
        let y = vec![0, 1, 0, 1];

        let mi = mutual_information(&x, &y).unwrap();
        assert_close(mi, 0.0, 1e-12);
    }

    #[test]
    fn mutual_information_of_identical_bit_is_one() {
        let x = vec![0, 0, 1, 1];
        let y = vec![0, 0, 1, 1];

        let mi = mutual_information(&x, &y).unwrap();
        assert_close(mi, 1.0, 1e-12);
    }

    #[test]
    fn mutual_information_symmetric() {
        let x = vec![0, 1, 0, 1, 0, 1];
        let y = vec![1, 1, 0, 0, 1, 0];

        let mi_xy = mutual_information(&x, &y).unwrap();
        let mi_yx = mutual_information(&y, &x).unwrap();

        assert_close(mi_xy, mi_yx, 1e-12);
    }

    #[test]
    fn mutual_information_length_mismatch_fails() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2];

        let err = mutual_information(&x, &y).unwrap_err();
        assert_eq!(err, InfoError::LengthMismatch { left: 3, right: 2 });
    }

    #[test]
    fn mutual_information_empty_fails() {
        let x: Vec<u32> = vec![];
        let y: Vec<u32> = vec![];

        let err = mutual_information(&x, &y).unwrap_err();
        assert_eq!(err, InfoError::EmptyInput);
    }

    #[test]
    fn mutual_information_matches_entropy_for_identical_variables() {
        let x = vec![0, 0, 0, 1, 1, 1, 1, 1];
        let h = entropy(&x).unwrap();
        let mi = mutual_information(&x, &x).unwrap();

        assert_close(mi, h, 1e-12);
    }

    #[test]
    fn joint_entropy_of_independent_fair_bits_is_two() {
        let x = vec![0, 0, 1, 1];
        let y = vec![0, 1, 0, 1];
        let h_xy = joint_entropy(&x, &y).unwrap();
        assert_close(h_xy, 2.0, 1e-12);
    }

    #[test]
    fn joint_entropy_of_identical_variables_matches_entropy() {
        let x = vec![0, 0, 1, 1, 1, 0];
        let h_xy = joint_entropy(&x, &x).unwrap();
        let h_x = entropy(&x).unwrap();
        assert_close(h_xy, h_x, 1e-12);
    }

    #[test]
    fn joint_entropy_length_mismatch_fails() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2];
        let err = joint_entropy(&x, &y).unwrap_err();
        assert_eq!(err, InfoError::LengthMismatch { left: 3, right: 2 });
    }

    #[test]
    fn conditional_entropy_of_identical_variables_is_zero() {
        let x = vec![0, 1, 0, 1, 1, 0];
        let h_x_given_x = conditional_entropy(&x, &x).unwrap();
        assert_close(h_x_given_x, 0.0, 1e-12);
    }

    #[test]
    fn conditional_entropy_of_independent_fair_bits_is_one() {
        let x = vec![0, 0, 1, 1];
        let y = vec![0, 1, 0, 1];
        let h_x_given_y = conditional_entropy(&x, &y).unwrap();
        assert_close(h_x_given_y, 1.0, 1e-12);
    }

    #[test]
    fn conditional_entropy_length_mismatch_fails() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2];
        let err = conditional_entropy(&x, &y).unwrap_err();
        assert_eq!(err, InfoError::LengthMismatch { left: 3, right: 2 });
    }

    // --- kl_divergence ---

    #[test]
    fn kl_divergence_identical_distributions_is_zero() {
        let p = vec![0, 0, 1, 1, 1];
        assert_close(kl_divergence(&p, &p).unwrap(), 0.0, 1e-12);
    }

    #[test]
    fn kl_divergence_is_not_symmetric() {
        // P: (0→1/2, 1→1/3, 2→1/6)  Q: (0→1/6, 1→1/6, 2→2/3)
        let p = vec![0, 0, 0, 1, 1, 2];
        let q = vec![0, 1, 2, 2, 2, 2];
        let kl_pq = kl_divergence(&p, &q).unwrap();
        let kl_qp = kl_divergence(&q, &p).unwrap();
        assert!((kl_pq - kl_qp).abs() > 1e-10);
    }

    #[test]
    fn kl_divergence_disjoint_support_fails() {
        let p = vec![0, 1];
        let q = vec![2, 3];
        assert_eq!(kl_divergence(&p, &q).unwrap_err(), InfoError::UndefinedDivergence);
    }

    #[test]
    fn kl_divergence_is_nonnegative() {
        let p = vec![0, 0, 1, 1, 1, 2];
        let q = vec![0, 1, 1, 2, 2, 2];
        assert!(kl_divergence(&p, &q).unwrap() >= 0.0);
    }

    // --- js_divergence ---

    #[test]
    fn js_divergence_identical_distributions_is_zero() {
        let p = vec![0, 1, 0, 1];
        assert_close(js_divergence(&p, &p).unwrap(), 0.0, 1e-12);
    }

    #[test]
    fn js_divergence_is_symmetric() {
        let p = vec![0, 0, 1, 2];
        let q = vec![1, 1, 2, 0];
        let jsd_pq = js_divergence(&p, &q).unwrap();
        let jsd_qp = js_divergence(&q, &p).unwrap();
        assert_close(jsd_pq, jsd_qp, 1e-12);
    }

    #[test]
    fn js_divergence_bounded_in_zero_one() {
        let p = vec![0, 0, 0, 1];
        let q = vec![1, 1, 1, 0];
        let jsd = js_divergence(&p, &q).unwrap();
        assert!(jsd >= 0.0 && jsd <= 1.0 + 1e-12);
    }

    #[test]
    fn js_divergence_disjoint_support_is_one() {
        // Completely disjoint supports → maximum divergence = 1 bit
        let p = vec![0, 0];
        let q = vec![1, 1];
        assert_close(js_divergence(&p, &q).unwrap(), 1.0, 1e-12);
    }

    // --- cross_entropy ---

    #[test]
    fn cross_entropy_equals_entropy_for_identical_distributions() {
        let p = vec![0, 0, 1, 1, 1];
        let h = entropy(&p).unwrap();
        let ce = cross_entropy(&p, &p).unwrap();
        assert_close(ce, h, 1e-12);
    }

    #[test]
    fn cross_entropy_greater_than_or_equal_to_entropy() {
        let p = vec![0, 0, 1, 1, 1, 2];
        let q = vec![0, 1, 1, 2, 2, 2];
        let h = entropy(&p).unwrap();
        let ce = cross_entropy(&p, &q).unwrap();
        assert!(ce >= h - 1e-12);
    }

    #[test]
    fn cross_entropy_disjoint_support_fails() {
        let p = vec![0, 1];
        let q = vec![2, 3];
        assert_eq!(cross_entropy(&p, &q).unwrap_err(), InfoError::UndefinedDivergence);
    }

    #[test]
    fn conditional_entropy_matches_definition() {
        let x = vec![0, 0, 1, 1, 1, 0, 1, 0];
        let y = vec![1, 1, 0, 0, 1, 0, 0, 1];
        let h_x_given_y = conditional_entropy(&x, &y).unwrap();
        let expected = joint_entropy(&x, &y).unwrap() - entropy(&y).unwrap();
        assert_close(h_x_given_y, expected, 1e-12);
    }

}
