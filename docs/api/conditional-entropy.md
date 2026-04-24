# conditional_entropy

Conditional entropy of $X$ given $Y$, in bits.

## Definition

$$H(X \mid Y) = H(X, Y) - H(Y)$$

$H(X \mid Y)$ measures how much uncertainty remains in $X$ once $Y$ is known. It is zero when $X$ is fully determined by $Y$, and equals $H(X)$ when $X$ and $Y$ are independent.

## Signatures

```rust
pub fn conditional_entropy<X, Y>(x: &[X], y: &[Y]) -> Result<f64, InfoError>
where
    X: Eq + Hash,
    Y: Eq + Hash

pub fn conditional_entropy_unchecked<X, Y>(x: &[X], y: &[Y]) -> f64
where
    X: Eq + Hash,
    Y: Eq + Hash
```

## Parameters

| Parameter | Type | Description |
|---|---|---|
| `x` | `&[X]` | Observed samples of the variable whose uncertainty is measured |
| `y` | `&[Y]` | Observed samples of the conditioning variable |

## Returns

The conditional entropy $H(X \mid Y)$ in bits, or an error if:

- either slice is empty → `InfoError::EmptyInput`
- slices have different lengths → `InfoError::LengthMismatch`

## Examples

```rust
use entropium::conditional_entropy;

// H(X|X) = 0 — no uncertainty in X if you already know X
let x = vec![0, 1, 0, 1, 1, 0];
assert!(conditional_entropy(&x, &x).unwrap() < 1e-12);

// H(X|Y) = H(X) when X and Y are independent
let x = vec![0, 0, 1, 1];
let y = vec![0, 1, 0, 1];
let h_x_given_y = conditional_entropy(&x, &y).unwrap();
let h_x         = entropium::entropy(&x).unwrap();
assert!((h_x_given_y - h_x).abs() < 1e-12);
```

## Properties

| Property | Formula |
|---|---|
| Non-negativity | $H(X \mid Y) \geq 0$ |
| Independence | $H(X \mid Y) = H(X)$ iff $X \perp Y$ |
| Full knowledge | $H(X \mid Y) = 0$ iff $X$ is a function of $Y$ |
| Chain rule | $H(X \mid Y) = H(X, Y) - H(Y)$ |
