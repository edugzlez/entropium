# joint_entropy

Computes the **joint entropy** of two discrete random variables from paired samples.

## What does it measure?

Joint entropy answers: **how much total uncertainty is there in the pair $(X, Y)$ together?**

It measures the average information content of observing both variables simultaneously, treating them as a single compound variable. The result tells you how many bits you need to describe a joint observation $(x, y)$.

Two intuitive extremes:
- If $X$ and $Y$ are **independent**, observing one tells you nothing about the other. Their joint uncertainty is the sum of their individual uncertainties: $H(X,Y) = H(X) + H(Y)$.
- If $X$ and $Y$ are **identical**, observing the pair gives you no more information than observing either one alone: $H(X,X) = H(X)$.

## Formula

$$H(X,Y) = -\sum_{x,y} p(x,y) \log_2 p(x,y)$$

The joint probabilities $p(x,y)$ are estimated from paired samples by counting co-occurrences.

## Signature

```rust
pub fn joint_entropy<X, Y>(x: &[X], y: &[Y]) -> Result<f64, InfoError>
where
    X: Eq + Hash,
    Y: Eq + Hash
```

```rust
pub fn joint_entropy_unchecked<X, Y>(x: &[X], y: &[Y]) -> f64
where
    X: Eq + Hash,
    Y: Eq + Hash
```

Note that `X` and `Y` can be **different types**.

### Parameters

| Parameter | Description |
|---|---|
| `x` | Observed samples of the first variable |
| `y` | Observed samples of the second variable — must be the same length as `x` |

The $i$-th element of `x` and the $i$-th element of `y` are treated as a joint observation $(x_i, y_i)$.

### Returns

Joint entropy in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | Either slice is empty |
| `InfoError::LengthMismatch` | The slices have different lengths |

## Examples

```rust
use entropium::{entropy, joint_entropy};

// Two independent fair bits → H(X,Y) = H(X) + H(Y) = 2 bits
let x = vec![0, 0, 1, 1];
let y = vec![0, 1, 0, 1];
assert_eq!(joint_entropy(&x, &y).unwrap(), 2.0);

// Identical variables → H(X,X) = H(X)
let x = vec![0, 0, 0, 1, 1, 1];
let h_xx = joint_entropy(&x, &x).unwrap();
let h_x  = entropy(&x).unwrap();
assert!((h_xx - h_x).abs() < 1e-12);

// Mixed types — e.g. pairing a category with a numeric label
let categories = vec!["A", "A", "B", "B"];
let scores     = vec![1u8, 2,   3,   4 ];
let h = joint_entropy(&categories, &scores).unwrap();
```

## Practical uses

- **Dependency analysis**: compare $H(X,Y)$ to $H(X) + H(Y)$. A gap smaller than $I(X;Y) = H(X) + H(Y) - H(X,Y)$ reveals shared information.
- **Multivariate compression**: $H(X,Y)$ is the lower bound on the number of bits needed to jointly encode both variables.
- **Building block**: joint entropy is used internally to compute `conditional_entropy` via $H(X \mid Y) = H(X,Y) - H(Y)$.

## Properties

| Property | Statement |
|---|---|
| Non-negativity | $H(X,Y) \geq 0$ |
| Symmetry | $H(X,Y) = H(Y,X)$ |
| Subadditivity | $H(X,Y) \leq H(X) + H(Y)$ |
| Independence | $H(X,Y) = H(X) + H(Y)$ iff $X \perp Y$ |
| Identical variables | $H(X,X) = H(X)$ |
| Chain rule | $H(X,Y) = H(X) + H(Y \mid X) = H(Y) + H(X \mid Y)$ |
