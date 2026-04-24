# mutual_information

Computes the **mutual information** between two discrete random variables from paired samples.

## What does it measure?

Mutual information answers: **how much do $X$ and $Y$ tell us about each other?**

It quantifies the reduction in uncertainty about one variable when the other is observed. Unlike correlation, which only captures linear relationships, mutual information detects *any* statistical dependency — linear or non-linear.

Two extremes:
- If $X$ and $Y$ are **independent**, knowing one reveals nothing about the other: $I(X;Y) = 0$.
- If $X$ and $Y$ are **identical**, knowing one completely determines the other: $I(X;Y) = H(X) = H(Y)$.

In between, $I(X;Y)$ measures exactly how many bits of $X$'s uncertainty are resolved by knowing $Y$.

## Formula

$$I(X;Y) = \sum_{x,y} p(x,y) \log_2 \frac{p(x,y)}{p(x)\,p(y)}$$

Equivalently:

$$I(X;Y) = H(X) + H(Y) - H(X,Y) = H(X) - H(X \mid Y)$$

## Signature

```rust
pub fn mutual_information<T>(x: &[T], y: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash
```

```rust
pub fn mutual_information_unchecked<T>(x: &[T], y: &[T]) -> f64
where
    T: Eq + Hash
```

Note that `x` and `y` must be **the same type** `T`. For mixed types, compute `joint_entropy` and `entropy` separately.

### Parameters

| Parameter | Description |
|---|---|
| `x` | Observed samples of the first variable |
| `y` | Observed samples of the second variable — same length as `x` |

### Returns

$I(X;Y)$ in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | Either slice is empty |
| `InfoError::LengthMismatch` | The slices have different lengths |

## Examples

```rust
use entropium::{entropy, mutual_information};

// Independent variables: I(X;Y) = 0
let x = vec![0, 0, 1, 1];
let y = vec![0, 1, 0, 1];
assert!(mutual_information(&x, &y).unwrap() < 1e-12);

// Identical variables: I(X;X) = H(X)
let x = vec![0, 0, 0, 1, 1, 1, 1, 1];
let mi = mutual_information(&x, &x).unwrap();
let h  = entropy(&x).unwrap();
assert!((mi - h).abs() < 1e-12);

// Symmetry: I(X;Y) = I(Y;X)
let x = vec![0, 1, 0, 1, 0, 1];
let y = vec![1, 1, 0, 0, 1, 0];
assert!((mutual_information(&x, &y).unwrap()
       - mutual_information(&y, &x).unwrap()).abs() < 1e-12);

// Partial dependency
let feature = vec![0, 0, 0, 1, 1, 1, 0, 1];
let label   = vec![0, 0, 1, 1, 1, 1, 0, 0];
let mi = mutual_information(&feature, &label).unwrap();
println!("Feature carries {mi:.4} bits about the label");
```

## Practical uses

- **Feature selection**: rank features by $I(\text{feature}; \text{label})$ to find which ones carry the most information about the target. Unlike Pearson correlation, mutual information captures non-linear dependencies.
- **Clustering evaluation**: compare two cluster assignments with mutual information to measure how similar they are regardless of label permutations.
- **Neuroscience / signal processing**: measure the statistical coupling between two spike trains or time series.
- **Causal analysis**: low mutual information between two variables is a necessary (though not sufficient) condition for independence.

## Properties

| Property | Statement |
|---|---|
| Non-negativity | $I(X;Y) \geq 0$ |
| Symmetry | $I(X;Y) = I(Y;X)$ |
| Independence | $I(X;Y) = 0$ iff $X \perp Y$ |
| Upper bound | $I(X;Y) \leq \min(H(X), H(Y))$ |
| Identical | $I(X;X) = H(X)$ |
| From marginals | $I(X;Y) = H(X) + H(Y) - H(X,Y)$ |
| From conditional | $I(X;Y) = H(X) - H(X \mid Y) = H(Y) - H(Y \mid X)$ |
