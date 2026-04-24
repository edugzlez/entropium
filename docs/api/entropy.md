# entropy

Shannon entropy of a discrete random variable, in bits.

## Definition

$$H(X) = -\sum_{x} p(x) \log_2 p(x)$$

$H(X)$ measures the average uncertainty of a variable. It is zero when the outcome is certain, and maximum when all outcomes are equally likely.

## Signatures

```rust
pub fn entropy<T>(data: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash

pub fn entropy_unchecked<T>(data: &[T]) -> f64
where
    T: Eq + Hash
```

## Parameters

| Parameter | Type | Description |
|---|---|---|
| `data` | `&[T]` | Observed samples of the variable |

## Returns

The Shannon entropy in bits, or `InfoError::EmptyInput` if `data` is empty.

## Examples

```rust
use entropium::entropy;

// Constant signal — zero entropy
let constant = vec![1, 1, 1, 1];
assert_eq!(entropy(&constant).unwrap(), 0.0);

// Fair coin — 1 bit
let coin = vec![0, 1, 0, 1];
assert_eq!(entropy(&coin).unwrap(), 1.0);

// Three symbols with unequal probabilities
let x = vec!["a", "a", "b"];
let h = entropy(&x).unwrap();
// H = -(2/3)*log2(2/3) - (1/3)*log2(1/3) ≈ 0.918
```

## Properties

| Property | Formula |
|---|---|
| Non-negativity | $H(X) \geq 0$ |
| Maximum | $H(X) \leq \log_2 \lvert\mathcal{X}\rvert$ |
| Certainty | $H(X) = 0$ iff $X$ is deterministic |
