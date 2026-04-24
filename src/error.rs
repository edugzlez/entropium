//! Error types for the entropium crate.

/// Errors that can occur when computing information-theoretic quantities.
#[derive(Debug, Clone, PartialEq)]
pub enum InfoError {
    /// The input slice(s) were empty; at least one observation is required.
    EmptyInput,
    /// The two input slices had different lengths (`left` vs `right`).
    LengthMismatch { left: usize, right: usize },
    /// `p` assigns positive probability to a value that `q` assigns zero probability,
    /// making the divergence infinite.
    UndefinedDivergence,
}
