---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Hygiene In Rust Macros

Declarative macros in Rust are partially hygieninic.

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

### The `$crate` variable

Item and crate paths are unhygienic, so item paths within a macro definition
could will refer to a different item than intended if their leading module or
crate name is defined differently at the call site than the macro author
expected.

In general, declarative macros themselves cannot carry along crate dependencies
in a hygienic way. However, there is a way out: to unambiguously refer to the
macro's **defining crate** only, the `$crate` metavariable may be used.

`$crate` expands to the root path of the crate that defined the macro. This can
be used to refer to local helper items without fear of interference, regardless
of the macro call site. These local helpers may call or re-exports items from
the standard library or other dependencies.

```rust,compile_fail
// Macro-defining crate `my_macros`
pub fn my_macro_helper(s: &str) {
    std::io::print(s)
}

macro_rules! print_something {
    ($args:tt) => {
        // Safe from shadowing of the standard library or any other crate,
        // because items from this crate accessed with $crate are hygienic!
        $crate::my_macro_helper(stringify!($args))
    };
}

// Macro-consuming crate that alters meaning of the `std` crate name
#![no_std]

// libcore exports many similar APIs to libstd, but not `io::print`
extern crate core as std;

fn main() {
    my_macros::print_something!()
}
```

<details>

- Carefully delineate dependencies in the example: the program as a whole
  depends on libstd, but in the top-level crate it is not a direct dependency,
  and libcore is imported with its name instead. libcore does not export
  `io::print`, so a straightforward reference to `std::io::print` in the macro
  would expand to a non-existing path. But because the macro crate does depend
  on libstd, and the macro only accesses its own local helper through the
  `$crate` metavariable, it is able to reliably refer to the stdlib (or another
  crate) indirectly.
- Explain that to enforce hygiene on local variables, the compiler keeps track
  of "syntax contexts." A local variable defined inside the macro has a
  different syntax context than a variable of the same name defined outside,
  which prevents collisions.

</details>
