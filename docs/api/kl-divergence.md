# kl_divergence

Computes the **Kullback-Leibler divergence** from $Q$ to $P$.

## What does it measure?

KL divergence answers: **if I believe the world follows distribution $Q$, but it actually follows $P$, how many extra bits do I waste per observation?**

It measures the information cost of using the wrong model. When $P = Q$, there is no cost: $D_{KL}(P \| Q) = 0$. As $P$ and $Q$ diverge, the cost grows without bound.

::: warning KL divergence is not symmetric
$D_{KL}(P \| Q) \neq D_{KL}(Q \| P)$ in general. This asymmetry is intentional and meaningful: the cost of approximating $P$ with $Q$ is different from approximating $Q$ with $P$.

If you need a symmetric measure, use [`js_divergence`](./js-divergence) instead.
:::

::: danger Undefined for disjoint supports
If $P$ assigns positive probability to a value that $Q$ assigns zero probability, then $p(x) \log_2(p(x)/q(x)) \to \infty$. `kl_divergence` returns `Err(InfoError::UndefinedDivergence)` in this case.

This is a fundamental property, not a library limitation. If your distributions may have different supports, use [`js_divergence`](./js-divergence), which is always finite.
:::

## Formula

$$D_{KL}(P \| Q) = \sum_{x} p(x) \log_2 \frac{p(x)}{q(x)}$$

The probabilities are estimated empirically from the input slices.

## Signature

```rust
pub fn kl_divergence<T>(p: &[T], q: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash
```

```rust
pub fn kl_divergence_unchecked<T>(p: &[T], q: &[T]) -> f64
where
    T: Eq + Hash
```

### Parameters

| Parameter | Description |
|---|---|
| `p` | Samples from the **true** distribution $P$ |
| `q` | Samples from the **reference** (model) distribution $Q$ |

The slices do **not** need to have the same length — they are independent samples from each distribution.

### Returns

$D_{KL}(P \| Q)$ in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | Either slice is empty |
| `InfoError::UndefinedDivergence` | `p` contains a value absent from `q` |

## Examples

```rust
use entropium::{kl_divergence, InfoError};

// Same distribution → KL = 0
let p = vec![0, 0, 1, 1, 1];
assert_eq!(kl_divergence(&p, &p).unwrap(), 0.0);

// KL is not symmetric
let p = vec![0, 0, 0, 1, 1, 2];     // P(0)=1/2, P(1)=1/3, P(2)=1/6
let q = vec![0, 1, 2, 2, 2, 2];     // Q(0)=1/6, Q(1)=1/6, Q(2)=2/3
let kl_pq = kl_divergence(&p, &q).unwrap();
let kl_qp = kl_divergence(&q, &p).unwrap();
assert!((kl_pq - kl_qp).abs() > 1e-10);

// Disjoint support → error
assert_eq!(
    kl_divergence(&[0, 1], &[2, 3]).unwrap_err(),
    InfoError::UndefinedDivergence
);

// Sample sizes can differ
let p_large = vec![0u8; 1000].into_iter().chain(vec![1u8; 500]).collect::<Vec<_>>();
let q_small = vec![0u8, 0, 1];
let kl = kl_divergence(&p_large, &q_small).unwrap();
```

## Practical uses

- **Model evaluation**: $D_{KL}(P_{\text{data}} \| P_{\text{model}})$ measures how well a model approximates the data distribution. Minimising this is equivalent to maximum-likelihood estimation.
- **Variational inference**: VI minimises $D_{KL}(q \| p)$ (note the reversed order), where $q$ is a tractable approximation and $p$ is the true posterior.
- **A/B testing**: compare the output distributions of two system versions to quantify how much they differ.
- **Anomaly detection**: compute $D_{KL}(P_{\text{live}} \| P_{\text{baseline}})$ over a rolling window; a spike signals a distribution shift.

## Properties

| Property | Statement |
|---|---|
| Non-negativity | $D_{KL}(P \| Q) \geq 0$ (Gibbs' inequality) |
| Identity | $D_{KL}(P \| Q) = 0$ iff $P = Q$ |
| Asymmetry | $D_{KL}(P \| Q) \neq D_{KL}(Q \| P)$ in general |
| Unbounded | $D_{KL}(P \| Q) \to \infty$ as supports diverge |
| Relation to cross-entropy | $D_{KL}(P \| Q) = H(P, Q) - H(P)$ |
