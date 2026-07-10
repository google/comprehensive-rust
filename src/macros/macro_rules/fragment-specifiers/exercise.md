---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Pair Macro

In this exercise, you will implement a macro named `pair!` that takes a single
syntactic element and doubles it into a tuple pair containing two of that
element: `(x, x)`.

Your `pair!` macro should work across three different syntactic contexts:

1. **Types:** `pair!(i32)` expands to `(i32, i32)`.
2. **Patterns:** `pair!(7)` and `pair!(_)` expand to `(7, 7)` and `(_, _)`,
   respectively.
3. **Expressions:** `pair!(1 + 2)` expands to `(1 + 2, 1 + 2)`.

### Instructions:

- Define the `pair!` macro with rules for types (`ty`), patterns (`pat`), and
  expressions (`expr`).
- Order the rules appropriately so that type names, patterns, and expressions
  are correctly captured.

```rust,compile_fail,editable
// TODO: Implement the `pair!` macro.

fn main() {
    let p: pair!(i32) = (10, 20);
    println!("Doubled type: {p:?}");

    let doubled = pair!(1 + 2);
    println!("Doubled expr: {doubled:?}");

    let val: pair!(i32) = pair!(7 as i32);

    match val {
        e @ pair!(7) => println!("Pattern position: matched {e:?}"),
        _ => println!("Did not match"),
    }
}
```
