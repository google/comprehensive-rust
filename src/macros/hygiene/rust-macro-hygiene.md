---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Hygiene In Rust Macros

Declarative macros in Rust are partially hygienic.

- They are hygienic with respect to: local variables, parameters, loop labels,
  and the special `$crate` variable.
- They are **not** hygienic with respect to: items, types, methods, and traits.

## Rationale

Frequently, rust macros are used as shorthand to refer to existing types and
traits, e.g. when defining `impl`s. In this situation, hygienic macros would
always need to accept all relevant items as arguments, imposing a floor beneath
which we could not decrease lexical boilerplate.

On the other hand, hygiene helps us write reliable code, so it is desirable for
any internal operations that a macro may want to perform. Luckily for us, Rust
does provide a solution for hygienic references to items.

## Example: Non-Hygienic Paths

```rust
// Item reference is not hygienic! This macro may refer to a different "print"
// function when invoked in different contexts.
macro_rules! call_print {
    () => {
        print()
    };
}

pub mod a {
    pub fn do_it() {
        call_print!()
    }
    pub fn print() {
        println!("::a::print()");
    }
}

pub fn print() {
    println!("::print()");
}

fn main() {
    a::do_it();
    call_print!();
}
```

The two expansions of the `call_print` macro refer to different items, depending
on the calling context. This could be useful as mere shorthand, but is not
robustly useful for referring to one particular item.

<details>

- Explain that to enforce hygiene on local variables, the compiler keeps track
  of "syntax contexts." A local variable defined inside the macro has a
  different syntax context than a variable of the same name defined outside,
  which prevents collisions.

</details>
