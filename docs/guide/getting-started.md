# Getting Started

## What is entropium?

`entropium` is a Rust library for computing **information-theoretic quantities** from discrete data. It answers questions like:

- How unpredictable is this sequence of observations?
- How much do two variables depend on each other?
- How similar are two probability distributions?

All results are expressed in **bits** (base-2 logarithm), the natural unit of information.

## Installation

Add `entropium` to your `Cargo.toml`:

```toml
[dependencies]
entropium = "0.1"
```

## A first example

```rust
use entropium::{entropy, mutual_information};

fn main() {
    // A biased coin: heads 75% of the time
    let flips = vec![1, 1, 1, 0, 1, 1, 0, 1];
    println!("H(coin) = {:.4} bits", entropy(&flips).unwrap());
    // → ~0.81 bits (less than 1 because it's biased)

    // Two correlated sensors
    let sensor_a = vec![0, 0, 1, 1, 0, 1];
    let sensor_b = vec![0, 0, 1, 1, 0, 1]; // identical
    println!("I(A;B) = {:.4} bits", mutual_information(&sensor_a, &sensor_b).unwrap());
    // → equals H(A): knowing B tells you everything about A
}
```

## Checked vs unchecked API

Every function comes in two flavours:

| Style | Signature | Use when |
|---|---|---|
| **Checked** | `fn entropy(&[T]) -> Result<f64, InfoError>` | Library code, user input, anything that can fail |
| **Unchecked** | `fn entropy_unchecked(&[T]) -> f64` | Scripts, tests, pre-validated inputs |

```rust
// Checked — handle errors explicitly
match entropium::entropy(&data) {
    Ok(h)  => println!("H = {h:.4}"),
    Err(e) => eprintln!("error: {e:?}"),
}

// Unchecked — panics on error
let h = entropium::entropy_unchecked(&data);
```

## Error handling

```rust
use entropium::InfoError;

// Empty input
assert_eq!(
    entropium::entropy(&[] as &[u8]).unwrap_err(),
    InfoError::EmptyInput
);

// Length mismatch (for two-sample functions)
assert_eq!(
    entropium::mutual_information(&[1, 2, 3], &[1, 2]).unwrap_err(),
    InfoError::LengthMismatch { left: 3, right: 2 }
);

// Undefined divergence (KL / cross-entropy when supports don't overlap)
assert_eq!(
    entropium::kl_divergence(&[0], &[1]).unwrap_err(),
    InfoError::UndefinedDivergence
);
```

## Works with any hashable type

`entropium` is generic over any type implementing `Eq + Hash`:

```rust
// u8, i32, &str, tuples, enums — anything hashable works
let letters = vec!['a', 'b', 'a', 'c', 'a', 'b'];
let h = entropium::entropy(&letters).unwrap();

#[derive(Eq, PartialEq, Hash)]
enum Label { Spam, Ham }
let labels = vec![Label::Ham, Label::Ham, Label::Spam];
let h = entropium::entropy(&labels).unwrap();
```

## Units and conventions

- All values are in **bits** (log base 2).
- Inputs are treated as **empirical samples**: the library counts occurrences and estimates probabilities from frequencies.
- All quantities are **non-negative** (guaranteed by the math, not just clamped).
