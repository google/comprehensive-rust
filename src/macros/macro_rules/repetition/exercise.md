---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Generalized Operations

In the earlier pattern matching exercise, you created a `define_op!` macro that
defined a single binary operation for `i32` at a time.

In this exercise, you will generalize that concept into a `define_ops!` macro
using the repetition constructs of macro matchers. The new macro should allow
implementing an arbitrary number of `std::ops` traits for a generic wrapper type
across multiple underlying numeric types in a single macro invocation.

The desired macro syntax is:

```rust,ignore
define_ops! {
    i32, f32, f64:
        Add::add => +,
        Sub::sub => -,
        Mul::mul => *
}
```

This invocation should implement `std::ops::Add`, `std::ops::Sub`, and
`std::ops::Mul` for `Scalar<i32>`, `Scalar<f32>`, and `Scalar<f64>`.

### Instructions:

- Given a wrapper struct `#[derive(Debug, PartialEq)] struct Scalar<T>(pub T);`,
  implement the `define_ops!` macro.
- Parse a comma-separated list of types (`$( $t:ty ),+`) followed by a colon
  (`:`).
- Parse a comma-separated list of trait mappings
  (`$( $trait_name:ident :: $fn_name:ident => $op:tt ),*`).
- In the macro expansion, use nested repetition `$($( ... )*)*` to generate
  `impl std::ops::$trait_name for Scalar<$t>` blocks.

```rust,compile_fail,editable
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Scalar<T: Copy>(pub T);

// TODO: Implement the `define_ops!` macro.

define_ops! {
    i32, f32, f64:
        Add::add => +,
        Sub::sub => -,
        Mul::mul => *
}

fn main() {
    let a32 = Scalar(10_i32);
    let b32 = Scalar(2_i32);
    assert_eq!(a32 + b32, Scalar(12));

    let a64 = Scalar(10.5_f64);
    let b64 = Scalar(2.5_f64);
    assert_eq!(a64 - b64, Scalar(8.0));
    assert_eq!(a64 * b64, Scalar(26.25));

    let a32_f = Scalar(10.5_f32);
    let b32_f = Scalar(2.5_f32);
    assert_eq!(a32_f + b32_f, Scalar(13.0_f32));
}
```
