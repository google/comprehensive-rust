---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# What Is Macro Hygiene

A macro systems is said to be **hygienic** if macros defined with it cannot
accidentally capture or shadow identifiers from their expansion sites.

This is desirable because it helps reason about macros separately from their
uses, avoiding bugs when code invoking macros might unintentionally use the same
identifier used internally by a macro.

Macros in C and C++ using the C preprocessor are not hygienic, but Rust's macro
system implements a limited form of hygiene.

Macro hygiene imposes two restrictions, corresponding to the two directions of
influence between the macro's and the calling code's lexical environment.

A macro system is **unhygienic** if a macro can either:

1. Implicitly access identifiers in the surrounding callsite scope.
2. Define a new local identifier that bleeds out and is implicitly accessible by
   the code surrounding its callsite.

### Example 1: Implicitly Accessing Calling Environment (Unhygienic)

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
        // Unhygienic: intent is to leak `local` to callsite
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
