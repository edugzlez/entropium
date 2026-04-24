# API Overview

`entropium` exposes four information-theoretic quantities and one error type.

## Functions

| Function | Description |
|---|---|
| [`entropy`](./entropy) | Shannon entropy $H(X)$ of a single variable |
| [`joint_entropy`](./joint-entropy) | Joint entropy $H(X,Y)$ of two variables |
| [`conditional_entropy`](./conditional-entropy) | Conditional entropy $H(X \mid Y)$ |
| [`mutual_information`](./mutual-information) | Mutual information $I(X;Y)$ |

Each function has a checked variant returning `Result<f64, InfoError>` and an `_unchecked` variant that panics.

## Error type

```rust
pub enum InfoError {
    EmptyInput,
    LengthMismatch { left: usize, right: usize },
}
```

| Variant | When |
|---|---|
| `EmptyInput` | Any input slice is empty |
| `LengthMismatch` | The two input slices have different lengths |

## Units

All values are in **bits** — the logarithm is base 2.

## Bounds

All quantities returned by `entropium` are non-negative.
