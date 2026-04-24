# conditional_entropy

Computes the **conditional entropy** of $X$ given $Y$ from paired samples.

## What does it measure?

Conditional entropy answers: **how much uncertainty remains in $X$ once we already know $Y$?**

It quantifies the irreducible uncertainty — the part of $X$ that $Y$ cannot explain. Think of it as the entropy of $X$ "after subtracting" the information that $Y$ provides.

Two extremes:
- If $Y$ **completely determines** $X$ (e.g. $X$ is a function of $Y$), there is no remaining uncertainty: $H(X \mid Y) = 0$.
- If $X$ and $Y$ are **independent**, knowing $Y$ tells you nothing about $X$, so the uncertainty is unchanged: $H(X \mid Y) = H(X)$.

## Formula

$$H(X \mid Y) = H(X, Y) - H(Y)$$

This is the chain rule of entropy: the joint uncertainty minus the uncertainty already accounted for by $Y$.

## Signature

```rust
pub fn conditional_entropy<X, Y>(x: &[X], y: &[Y]) -> Result<f64, InfoError>
where
    X: Eq + Hash,
    Y: Eq + Hash
```

```rust
pub fn conditional_entropy_unchecked<X, Y>(x: &[X], y: &[Y]) -> f64
where
    X: Eq + Hash,
    Y: Eq + Hash
```

### Parameters

| Parameter | Description |
|---|---|
| `x` | Observed samples of the variable whose residual uncertainty is measured |
| `y` | Observed samples of the conditioning variable |

The $i$-th element of `x` and the $i$-th element of `y` are treated as a joint observation.

### Returns

$H(X \mid Y)$ in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | Either slice is empty |
| `InfoError::LengthMismatch` | The slices have different lengths |

## Examples

```rust
use entropium::{entropy, conditional_entropy};

// Knowing X perfectly eliminates its own uncertainty: H(X|X) = 0
let x = vec![0, 1, 0, 1, 1, 0];
assert!(conditional_entropy(&x, &x).unwrap() < 1e-12);

// Independent variables: H(X|Y) = H(X)
let x = vec![0, 0, 1, 1];
let y = vec![0, 1, 0, 1];
let h_x_given_y = conditional_entropy(&x, &y).unwrap();
let h_x         = entropy(&x).unwrap();
assert!((h_x_given_y - h_x).abs() < 1e-12);

// Partially correlated: H(X|Y) is between 0 and H(X)
let temperature = vec![0, 0, 0, 1, 1, 1, 2, 2];
let season      = vec![0, 0, 0, 1, 1, 1, 2, 3]; // season predicts temp, but not perfectly
let h = conditional_entropy(&temperature, &season).unwrap();
println!("Residual uncertainty in temperature given season: {h:.4} bits");
```

## Practical uses

- **Feature selection**: $H(Y \mid X)$ measures how much uncertainty remains in the target $Y$ after observing feature $X$. Features that drive it close to zero are highly predictive.
- **Information gain**: decision trees split on the feature $X$ that maximises $H(Y) - H(Y \mid X)$ (the reduction in label uncertainty).
- **Lossless compression of correlated streams**: if two correlated streams $X$ and $Y$ must be encoded separately and the decoder receives $Y$ first, the stream $X$ can be compressed to $H(X \mid Y)$ bits per symbol instead of $H(X)$.

## Properties

| Property | Statement |
|---|---|
| Non-negativity | $H(X \mid Y) \geq 0$ |
| Zero | $H(X \mid Y) = 0$ iff $X$ is a function of $Y$ |
| Independence | $H(X \mid Y) = H(X)$ iff $X \perp Y$ |
| Asymmetry | $H(X \mid Y) \neq H(Y \mid X)$ in general |
| Chain rule | $H(X \mid Y) = H(X,Y) - H(Y)$ |
| Relation to MI | $H(X \mid Y) = H(X) - I(X;Y)$ |
