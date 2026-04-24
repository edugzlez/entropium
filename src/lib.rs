//! Information-theory primitives: entropy, joint entropy, conditional entropy,
//! and mutual information. All values are in bits (base-2 logarithm).
//!
//! Every function comes in two flavours:
//! - A checked variant that returns `Result<f64, InfoError>`.
//! - An `_unchecked` variant that panics on error, intended for quick
//!   prototyping or when the caller has already validated its inputs.

mod conditional_entropy;
mod entropy;
mod error;
mod joint_entropy;
mod mutual_information;

pub use conditional_entropy::{conditional_entropy, conditional_entropy_unchecked};
pub use entropy::{entropy, entropy_unchecked};
pub use error::InfoError;
pub use joint_entropy::{joint_entropy, joint_entropy_unchecked};
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

    #[test]
    fn conditional_entropy_matches_definition() {
        let x = vec![0, 0, 1, 1, 1, 0, 1, 0];
        let y = vec![1, 1, 0, 0, 1, 0, 0, 1];
        let h_x_given_y = conditional_entropy(&x, &y).unwrap();
        let expected = joint_entropy(&x, &y).unwrap() - entropy(&y).unwrap();
        assert_close(h_x_given_y, expected, 1e-12);
    }

}
