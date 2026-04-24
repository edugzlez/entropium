# cross_entropy

Computes the **cross-entropy** of $P$ relative to $Q$.

## What does it measure?

Cross-entropy answers: **how many bits do I need per observation if the data comes from $P$ but I designed my code assuming $Q$?**

When you build an optimal code for distribution $Q$ (assigning $-\log_2 q(x)$ bits to outcome $x$) and then use it to encode data drawn from the *true* distribution $P$, the average code length is exactly $H(P, Q)$.

- If $P = Q$: the code is optimal, and you use exactly $H(P)$ bits per symbol.
- If $P \neq Q$: the code is suboptimal, and you waste $D_{KL}(P \| Q)$ extra bits per symbol.

This decomposes neatly as:

$$H(P, Q) = H(P) + D_{KL}(P \| Q)$$

::: warning Undefined for disjoint supports
If $P$ assigns positive probability to a value that $Q$ assigns zero probability, the cross-entropy is infinite — the code for $Q$ would need infinitely many bits for that outcome. `cross_entropy` returns `Err(InfoError::UndefinedDivergence)` in this case.
:::

## Formula

$$H(P, Q) = -\sum_{x} p(x) \log_2 q(x)$$

## Signature

```rust
pub fn cross_entropy<T>(p: &[T], q: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash
```

```rust
pub fn cross_entropy_unchecked<T>(p: &[T], q: &[T]) -> f64
where
    T: Eq + Hash
```

### Parameters

| Parameter | Description |
|---|---|
| `p` | Samples from the **true** distribution $P$ |
| `q` | Samples from the **model** distribution $Q$ |

The slices do **not** need to have the same length.

### Returns

$H(P, Q)$ in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | Either slice is empty |
| `InfoError::UndefinedDivergence` | `p` contains a value absent from `q` |

## Examples

```rust
use entropium::{cross_entropy, entropy, InfoError};

// H(P,P) = H(P) — the code is optimal when P = Q
let p = vec![0, 0, 0, 1, 1];
let h  = entropy(&p).unwrap();
let ce = cross_entropy(&p, &p).unwrap();
assert!((ce - h).abs() < 1e-12);

// H(P,Q) >= H(P) — any mismatch increases cost
let p = vec![0, 0, 1, 1, 1, 2];
let q = vec![0, 1, 1, 2, 2, 2];
assert!(cross_entropy(&p, &q).unwrap() >= entropy(&p).unwrap() - 1e-12);

// Disjoint support → error
assert_eq!(
    cross_entropy(&[0, 1], &[2, 3]).unwrap_err(),
    InfoError::UndefinedDivergence
);
```

## Practical uses

- **Classification loss**: in machine learning, training a classifier by minimising cross-entropy $H(P_{\text{true}}, P_{\text{model}})$ over the training set is equivalent to maximum-likelihood estimation of the model parameters. This is the standard loss used in logistic regression, softmax classifiers, and language models.
- **Language model perplexity**: the perplexity of a language model is $2^{H(P_{\text{text}}, P_{\text{model}})}$, where $P_{\text{text}}$ is the empirical distribution of the test corpus. Lower perplexity means the model's distribution is closer to the true one.
- **Information-theoretic lower bound**: $H(P, Q)$ is the minimum average description length achievable when the code is designed for $Q$ but applied to data from $P$.

## Properties

| Property | Statement |
|---|---|
| Lower bound | $H(P,Q) \geq H(P)$ (cross-entropy is always ≥ true entropy) |
| Equality | $H(P,Q) = H(P)$ iff $P = Q$ |
| Asymmetry | $H(P,Q) \neq H(Q,P)$ in general |
| Decomposition | $H(P,Q) = H(P) + D_{KL}(P \| Q)$ |
