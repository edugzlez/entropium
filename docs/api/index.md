# API Reference

`entropium` provides two families of functions: **single-distribution** measures (entropy variants) and **two-distribution** measures (divergences). All values are in **bits**.

## Single-distribution measures

These functions take observed samples and characterise their uncertainty.

| Function | What it measures |
|---|---|
| [`entropy`](./entropy) | Uncertainty of one variable — $H(X)$ |
| [`joint_entropy`](./joint-entropy) | Total uncertainty of two variables together — $H(X,Y)$ |
| [`conditional_entropy`](./conditional-entropy) | Uncertainty of $X$ once $Y$ is known — $H(X \mid Y)$ |
| [`mutual_information`](./mutual-information) | Information shared between two variables — $I(X;Y)$ |

## Two-distribution measures

These functions compare two empirical distributions $P$ and $Q$.

| Function | What it measures |
|---|---|
| [`kl_divergence`](./kl-divergence) | How much $P$ diverges from $Q$ (asymmetric) — $D_{KL}(P \| Q)$ |
| [`js_divergence`](./js-divergence) | Symmetric, bounded "distance" between $P$ and $Q$ — $JSD(P \| Q)$ |
| [`cross_entropy`](./cross-entropy) | Cost of encoding $P$ with a code optimised for $Q$ — $H(P, Q)$ |

## Error type

```rust
pub enum InfoError {
    /// Any input slice is empty.
    EmptyInput,
    /// Two input slices have different lengths.
    LengthMismatch { left: usize, right: usize },
    /// P assigns positive probability to a value absent from Q,
    /// making the divergence infinite.
    UndefinedDivergence,
}
```

## Relationships between functions

These identities hold exactly (up to floating-point precision) and are verified by the test suite:

```
H(X,Y) = H(X) + H(Y|X)          chain rule
H(X|Y) = H(X,Y) − H(Y)          conditional from joint
I(X;Y) = H(X) + H(Y) − H(X,Y)   mutual from marginals
I(X;Y) = H(X) − H(X|Y)          mutual from conditional
H(P,Q) = H(P) + D_KL(P‖Q)       cross-entropy decomposition
```
