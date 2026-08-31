---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# What Is Macro Hygiene

A macro system is **unhygienic** if a macro can:

1. Implicitly access identifiers in the surrounding callsite scope.
2. Define a new local identifier that bleeds out and is implicitly accessible by
   the surrounding callsite.

### Example 1: Implicitly Accessing Callsite State (Unhygienic)

```rust,ignore
macro_rules! use_local {
    () => {
        // Unhygienic: attempts to implicitly read `local` from callsite
        println!("{}", local);
    };
}

fn main() {
    let local = "Hello, Macros!".to_string();
    use_local!(); // In an unhygienic system, this would compile!
}
```

### Example 2: Leaking Local Variables (Unhygienic)

```rust,ignore
macro_rules! make_local {
    () => {
        // Unhygienic: attempts to leak `local` to callsite
        let local = "Hello, Macros!".to_string();
    };
}

fn main() {
    make_local!();
    println!("{}", local); // In an unhygienic system, this would compile!
}
```

In Rust, **neither of these examples compile**. Both produce the error:
`error[E0425]: cannot find value 'local' in this scope`.

Rust's macro system treats variables hygienically, protecting from silent
namespace pollution.

However, declarative macros in Rust are not fully hygienic!
