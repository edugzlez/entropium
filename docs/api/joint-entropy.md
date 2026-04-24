# joint_entropy

Joint entropy of two discrete random variables, in bits.

## Definition

$$H(X, Y) = -\sum_{x,y} p(x,y) \log_2 p(x,y)$$

$H(X,Y)$ measures the total uncertainty of the pair $(X, Y)$. It equals $H(X) + H(Y)$ when $X$ and $Y$ are independent, and equals $H(X) = H(Y)$ when they are identical.

## Signatures

```rust
pub fn joint_entropy<X, Y>(x: &[X], y: &[Y]) -> Result<f64, InfoError>
where
    X: Eq + Hash,
    Y: Eq + Hash

pub fn joint_entropy_unchecked<X, Y>(x: &[X], y: &[Y]) -> f64
where
    X: Eq + Hash,
    Y: Eq + Hash
```

## Parameters

| Parameter | Type | Description |
|---|---|---|
| `x` | `&[X]` | Observed samples of the first variable |
| `y` | `&[Y]` | Observed samples of the second variable |

`X` and `Y` can be different types.

## Returns

The joint entropy in bits, or an error if:

- either slice is empty → `InfoError::EmptyInput`
- slices have different lengths → `InfoError::LengthMismatch`

## Examples

```rust
use entropium::joint_entropy;

// Two independent fair bits → H(X,Y) = 2
let x = vec![0, 0, 1, 1];
let y = vec![0, 1, 0, 1];
assert_eq!(joint_entropy(&x, &y).unwrap(), 2.0);

// Identical variables → H(X,X) = H(X)
let x = vec![0, 0, 1, 1, 1, 0];
let h_xx = joint_entropy(&x, &x).unwrap();
let h_x  = entropium::entropy(&x).unwrap();
assert!((h_xx - h_x).abs() < 1e-12);

// Mixed types
let ids   = vec![1u32, 2, 1, 2];
let labels = vec!["a", "b", "a", "b"];
let h = joint_entropy(&ids, &labels).unwrap();
```

## Properties

| Property | Formula |
|---|---|
| Symmetry | $H(X,Y) = H(Y,X)$ |
| Independence | $H(X,Y) = H(X) + H(Y)$ iff $X \perp Y$ |
| Identical | $H(X,X) = H(X)$ |
| Chain rule | $H(X,Y) = H(X) + H(Y \mid X)$ |
