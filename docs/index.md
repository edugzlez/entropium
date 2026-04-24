---
layout: home

hero:
  name: entropium
  text: Information theory for Rust
  tagline: Shannon entropy, joint entropy, conditional entropy and mutual information — zero dependencies, ergonomic API.
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: API Reference
      link: /api/
    - theme: alt
      text: View on GitHub
      link: https://github.com/edugzlez/entropium

features:
  - icon: ⚡
    title: Zero dependencies
    details: No production dependencies. Just Rust and the standard library.

  - icon: 🔒
    title: Type-safe generics
    details: Works with any type that implements <code>Eq + Hash</code> — integers, strings, structs, tuples.

  - icon: 🧮
    title: Checked & unchecked
    details: Every function has a checked variant returning <code>Result</code> and an unchecked variant that panics, so you choose the tradeoff.

  - icon: 📐
    title: Mathematically correct
    details: Verified against known properties — H(X|X) = 0, I(X;Y) = H(X) for identical variables, H(X,Y) = 2 for independent fair bits.
---
