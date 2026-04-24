# mutual_information

Mutual information between two discrete random variables, in bits.

## Definition

$$I(X;Y) = \sum_{x,y} p(x,y) \log_2 \frac{p(x,y)}{p(x)\,p(y)}$$

$I(X;Y)$ measures how much information $X$ and $Y$ share. It is zero when they are independent, and equals $H(X) = H(Y)$ when they are identical.

## Signatures

```rust
pub fn mutual_information<T>(x: &[T], y: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash

pub fn mutual_information_unchecked<T>(x: &[T], y: &[T]) -> f64
where
    T: Eq + Hash
```

Note that `x` and `y` must be of the **same type** `T`.

## Parameters

| Parameter | Type | Description |
|---|---|---|
| `x` | `&[T]` | Observed samples of the first variable |
| `y` | `&[T]` | Observed samples of the second variable |

## Returns

The mutual information $I(X;Y)$ in bits, or an error if:

- either slice is empty → `InfoError::EmptyInput`
- slices have different lengths → `InfoError::LengthMismatch`

## Examples

```rust
use entropium::mutual_information;

// Independent variables → I(X;Y) = 0
let x = vec![0, 0, 1, 1];
let y = vec![0, 1, 0, 1];
assert!(mutual_information(&x, &y).unwrap() < 1e-12);

// Identical variables → I(X;X) = H(X)
let x = vec![0, 0, 0, 1, 1, 1, 1, 1];
let mi = mutual_information(&x, &x).unwrap();
let h  = entropium::entropy(&x).unwrap();
assert!((mi - h).abs() < 1e-12);

// Symmetry: I(X;Y) = I(Y;X)
let x = vec![0, 1, 0, 1, 0, 1];
let y = vec![1, 1, 0, 0, 1, 0];
let mi_xy = mutual_information(&x, &y).unwrap();
let mi_yx = mutual_information(&y, &x).unwrap();
assert!((mi_xy - mi_yx).abs() < 1e-12);
```

## Properties

| Property | Formula |
|---|---|
| Non-negativity | $I(X;Y) \geq 0$ |
| Symmetry | $I(X;Y) = I(Y;X)$ |
| Independence | $I(X;Y) = 0$ iff $X \perp Y$ |
| Identical | $I(X;X) = H(X)$ |
| Relation to entropy | $I(X;Y) = H(X) + H(Y) - H(X,Y)$ |
| Relation to conditional | $I(X;Y) = H(X) - H(X \mid Y)$ |
