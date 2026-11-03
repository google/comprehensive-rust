---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Internal Rules

A common pattern for implementing complex macros is to define helper arms (often
called **internal rules**) prefixed with a unique, non-standard token, such as
`@`.

These internal arms are not meant to be called by users, but are used internally
by the macro's public entry points to delegate and reuse logic:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! print_literals {
    // Internal helper rules:
    (@all_literals $($tt:tt)*) => {
        println!("all-lit: {}", format!($($tt),*))
    };
    (@some_literals $($tt:tt)*) => {
        println!("some-lit: {}", format!($($tt),*))
    };

    // Public entrypoints:
    ($($tt:literal),*) => {
        print_literals!(@all_literals $($tt)*)
    };
    ($($tt:expr),*) => {
        print_literals!(@some_literals $($tt)*)
    };
}

fn main() {
    print_literals!("format: {} {}", 123, "wow");
    print_literals!("format: {} {}", 123, format!("non-literal"));
}
```

<details>

- Note that `@` is purely a convention, which works well because `@` cannot
  begin a standard Rust item or expression, ensuring public users don't
  accidentally invoke these helper arms.
- Trace the public matcher arms' flow through the recursive call to
  (`print_literals!`) while selecting the next branch to take using the
  `@`-prefixed tokens.

</details>
