# entropium

[![Crates.io](https://img.shields.io/crates/v/entropium)](https://crates.io/crates/entropium)
[![docs.rs](https://img.shields.io/docsrs/entropium)](https://docs.rs/entropium)
[![CI](https://github.com/edugzlez/entropium/actions/workflows/publish.yml/badge.svg)](https://github.com/edugzlez/entropium/actions/workflows/publish.yml)
![License](https://img.shields.io/crates/l/entropium)

Information-theory primitives for Rust. All values are in **bits** (base-2 logarithm).

| Function              | Formula                                                |
| --------------------- | ------------------------------------------------------ |
| `entropy`             | $H(X) = -\sum p(x)\log_2 p(x)$                         |
| `joint_entropy`       | $H(X,Y) = -\sum p(x,y)\log_2 p(x,y)$                   |
| `conditional_entropy` | $H(X \mid Y) = H(X,Y) - H(Y)$                          |
| `mutual_information`  | $I(X;Y) = \sum p(x,y)\log_2 \frac{p(x,y)}{p(x)\,p(y)}$ |

## Install

```toml
[dependencies]
entropium = "0.1"
```

## Usage

Every function comes in two variants:

- **Checked** — returns `Result<f64, InfoError>`, errors on empty input or length mismatch.
- **Unchecked** (`_unchecked`) — panics on error, for when you've already validated inputs.

```rust
use entropium::{entropy, joint_entropy, conditional_entropy, mutual_information};

// Shannon entropy — fair coin = 1 bit
let x = vec![0, 1, 0, 1];
assert_eq!(entropy(&x).unwrap(), 1.0);

// Joint entropy — two independent fair bits = 2 bits
let y = vec![0, 1, 1, 0];
assert_eq!(joint_entropy(&x, &y).unwrap(), 2.0);

// Conditional entropy — H(X|X) = 0 (no uncertainty if you already know X)
assert_eq!(conditional_entropy(&x, &x).unwrap(), 0.0);

// Mutual information — I(X;X) = H(X)
assert_eq!(mutual_information(&x, &x).unwrap(), entropy(&x).unwrap());
```

Works with any type that implements `Eq + Hash`:

```rust
let words = vec!["the", "quick", "brown", "fox", "the", "fox"];
let h = entropium::entropy(&words).unwrap();
```

### Error handling

```rust
use entropium::InfoError;

match entropium::entropy(&[] as &[u8]) {
    Err(InfoError::EmptyInput) => println!("need at least one sample"),
    _ => {}
}

match entropium::mutual_information(&[1, 2, 3], &[1, 2]) {
    Err(InfoError::LengthMismatch { left, right }) => {
        println!("got {left} vs {right} samples")
    }
    _ => {}
}
```

## No dependencies

`entropium` has zero production dependencies.

## License

MIT OR Apache-2.0
