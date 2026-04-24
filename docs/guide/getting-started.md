# Getting Started

## Installation

Add `entropium` to your `Cargo.toml`:

```toml
[dependencies]
entropium = "0.1"
```

## Quick example

```rust
use entropium::{entropy, joint_entropy, conditional_entropy, mutual_information};

fn main() {
    let x = vec![0, 1, 0, 1, 0, 1];
    let y = vec![0, 0, 1, 1, 0, 1];

    println!("H(X)      = {:.4}", entropy(&x).unwrap());
    println!("H(X,Y)    = {:.4}", joint_entropy(&x, &y).unwrap());
    println!("H(X|Y)    = {:.4}", conditional_entropy(&x, &y).unwrap());
    println!("I(X;Y)    = {:.4}", mutual_information(&x, &y).unwrap());
}
```

## Checked vs unchecked

Every function has two variants:

| Variant | Returns | Errors on |
|---|---|---|
| `entropy(&data)` | `Result<f64, InfoError>` | empty input |
| `entropy_unchecked(&data)` | `f64` | panics |
| `mutual_information(&x, &y)` | `Result<f64, InfoError>` | empty input, length mismatch |
| `mutual_information_unchecked(&x, &y)` | `f64` | panics |

Use the checked variant in library code. Use the unchecked variant in scripts or when you have already validated the inputs.

## Error handling

```rust
use entropium::{entropy, mutual_information, InfoError};

// Empty input
match entropy(&[] as &[u8]) {
    Err(InfoError::EmptyInput) => eprintln!("need at least one sample"),
    Ok(h) => println!("H = {h}"),
    _ => {}
}

// Length mismatch
match mutual_information(&[1, 2, 3], &[1, 2]) {
    Err(InfoError::LengthMismatch { left, right }) => {
        eprintln!("x has {left} samples, y has {right}")
    }
    Ok(mi) => println!("I = {mi}"),
    _ => {}
}
```

## Generic over any hashable type

`entropium` works with any type that implements `Eq + Hash`:

```rust
// Strings
let words = vec!["the", "quick", "brown", "the", "fox", "the"];
let h = entropium::entropy(&words).unwrap();

// Tuples
let pairs = vec![(0, 'a'), (1, 'b'), (0, 'a')];
let h = entropium::entropy(&pairs).unwrap();

// Enums
#[derive(Eq, PartialEq, Hash)]
enum Coin { Heads, Tails }
let flips = vec![Coin::Heads, Coin::Tails, Coin::Heads];
let h = entropium::entropy(&flips).unwrap();
```
