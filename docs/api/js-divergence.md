# js_divergence

Computes the **Jensen-Shannon divergence** between two distributions.

## What does it measure?

Jensen-Shannon divergence answers: **how different are two distributions, on a scale from 0 to 1?**

It is the symmetric, bounded sibling of [KL divergence](./kl-divergence). Rather than measuring the one-directional cost of approximating $P$ with $Q$, JSD considers both directions symmetrically by comparing each distribution to their average $M = (P+Q)/2$.

Key advantages over KL divergence:
- **Always well-defined**, even when the distributions have disjoint supports.
- **Symmetric**: $JSD(P \| Q) = JSD(Q \| P)$.
- **Bounded**: $JSD \in [0, 1]$ bits, making it easy to interpret.
- Its square root $\sqrt{JSD}$ is a proper metric distance.

## Formula

$$JSD(P \| Q) = \frac{1}{2} D_{KL}(P \| M) + \frac{1}{2} D_{KL}(Q \| M), \quad M = \frac{P + Q}{2}$$

Because $M$ is a mixture of $P$ and $Q$, every value in $\text{supp}(P) \cup \text{supp}(Q)$ has positive probability in $M$, making both KL terms always finite.

## Signature

```rust
pub fn js_divergence<T>(p: &[T], q: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash
```

```rust
pub fn js_divergence_unchecked<T>(p: &[T], q: &[T]) -> f64
where
    T: Eq + Hash
```

### Parameters

| Parameter | Description |
|---|---|
| `p` | Samples from the first distribution $P$ |
| `q` | Samples from the second distribution $Q$ |

The slices do **not** need to have the same length.

### Returns

$JSD(P \| Q) \in [0, 1]$ in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | Either slice is empty |

## Examples

```rust
use entropium::js_divergence;

// Identical distributions → JSD = 0
let p = vec![0, 1, 0, 1];
assert_eq!(js_divergence(&p, &p).unwrap(), 0.0);

// Completely disjoint supports → maximum divergence = 1 bit
let p = vec![0, 0, 0];
let q = vec![1, 1, 1];
assert!((js_divergence(&p, &q).unwrap() - 1.0).abs() < 1e-12);

// Always symmetric
let p = vec![0, 0, 1, 2];
let q = vec![1, 1, 2, 0];
let jsd_pq = js_divergence(&p, &q).unwrap();
let jsd_qp = js_divergence(&q, &p).unwrap();
assert!((jsd_pq - jsd_qp).abs() < 1e-12);

// Result is always in [0, 1]
let p = vec![0u8, 0, 0, 1, 2, 2];
let q = vec![0u8, 1, 1, 1, 2, 2];
let jsd = js_divergence(&p, &q).unwrap();
assert!(jsd >= 0.0 && jsd <= 1.0);
```

## Practical uses

- **Distribution comparison**: anywhere you would use KL divergence but need a symmetric, bounded result — e.g. comparing language models, comparing histograms of sensor readings before and after an event.
- **Generative model evaluation**: JSD is the theoretical loss of the original GAN formulation. Minimising JSD between the real and generated distributions is equivalent to training a perfect discriminator.
- **Text similarity**: compare the word-frequency distributions of two documents. A JSD of 0 means identical word distributions; 1 means no shared vocabulary.
- **Dataset shift detection**: compute JSD between a training-time feature distribution and a production-time feature distribution. Values above a threshold trigger a retraining alert.

## Properties

| Property | Statement |
|---|---|
| Non-negativity | $JSD(P \| Q) \geq 0$ |
| Symmetry | $JSD(P \| Q) = JSD(Q \| P)$ |
| Bounded | $0 \leq JSD(P \| Q) \leq 1$ bit |
| Identity | $JSD(P \| Q) = 0$ iff $P = Q$ |
| Maximum | $JSD(P \| Q) = 1$ iff $\text{supp}(P) \cap \text{supp}(Q) = \emptyset$ |
| Metric | $\sqrt{JSD}$ satisfies the triangle inequality |
