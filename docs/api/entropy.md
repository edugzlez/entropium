# entropy

Computes the **Shannon entropy** of a discrete random variable from a slice of observed samples.

## What does it measure?

Entropy answers the question: **how unpredictable is this variable?**

Intuitively, it measures the average number of yes/no questions you need to ask to identify an unknown outcome. A fair coin requires exactly 1 question ("is it heads?") — so its entropy is 1 bit. A fair die requires about 2.58 questions — so its entropy is $\log_2 6 \approx 2.58$ bits.

Two extremes:
- **Zero entropy** — the outcome is always the same. No questions needed.
- **Maximum entropy** — all outcomes are equally likely. No shortcut is possible.

## Formula

$$H(X) = -\sum_{x \in \mathcal{X}} p(x) \log_2 p(x)$$

The library estimates $p(x)$ empirically from the input slice by counting occurrences.

## Signature

```rust
pub fn entropy<T>(data: &[T]) -> Result<f64, InfoError>
where
    T: Eq + Hash
```

```rust
pub fn entropy_unchecked<T>(data: &[T]) -> f64
where
    T: Eq + Hash
```

### Parameters

| Parameter | Description |
|---|---|
| `data` | Observed samples. The type `T` can be any `Eq + Hash` value. |

### Returns

Shannon entropy in bits, or:

| Error | When |
|---|---|
| `InfoError::EmptyInput` | `data` is empty |

## Examples

```rust
use entropium::entropy;

// A constant signal carries no information
let constant = vec![42u8; 100];
assert_eq!(entropy(&constant).unwrap(), 0.0);

// A fair coin flip carries exactly 1 bit
let fair_coin = vec![0, 1, 0, 1, 0, 1];
assert_eq!(entropy(&fair_coin).unwrap(), 1.0);

// A biased coin carries less than 1 bit
let biased_coin = vec![1, 1, 1, 0]; // P(1)=0.75, P(0)=0.25
let h = entropy(&biased_coin).unwrap();
assert!(h < 1.0); // ~0.81 bits

// A fair die carries log₂(6) ≈ 2.58 bits
let die = vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];
let h = entropy(&die).unwrap();
println!("H(die) = {h:.4}"); // → 2.5850

// Works with any hashable type
let words = vec!["the", "quick", "brown", "fox", "the", "fox"];
let h = entropy(&words).unwrap();
```

## Practical uses

- **Measuring data quality**: low entropy in a feature column may indicate near-constant values, which are unlikely to be useful for a model.
- **Compression**: entropy is the theoretical lower bound on average codeword length (Shannon's source coding theorem).
- **Anomaly detection**: a sudden drop or spike in entropy can signal a change in the underlying process.
- **Decision trees**: entropy is used in the ID3 algorithm as the impurity measure, via information gain $\Delta H = H(\text{parent}) - \sum H(\text{child})$.

## Properties

| Property | Statement |
|---|---|
| Non-negativity | $H(X) \geq 0$ always |
| Zero entropy | $H(X) = 0$ iff $X$ is deterministic |
| Maximum | $H(X) \leq \log_2 \lvert\mathcal{X}\rvert$, with equality iff $X$ is uniform |
| Additivity | If $X \perp Y$, then $H(X, Y) = H(X) + H(Y)$ |
