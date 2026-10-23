---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# A Rust Macro Example

Here is a comparable implementation of the previous custom syntax example
written in Rust:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! procedure {
    ($name:ident BEGIN $code:stmt; END) => {
        fn $name() { $code }
    };
}

macro_rules! expect {
    ($test:expr => OR DIE $message:literal) => {
        if !$test {
            panic!("{}", $message);
        }
    };
}

procedure! {
    main BEGIN
        for idx in 0..10 {
            expect!(idx < 10 => OR DIE "Uh-Oh!");
        };
    END
}
```

### Compared to C:

- **Syntactic Safety:** The input is structurally validated. `$code` must be a
  valid Rust statement and `$name` must be a valid Rust identifier.
- **Hygiene:** In `expect!`, the test `idx < 10` is explicitly passed as an
  expression `$test`, rather than having the macro rely on a hidden, implicitly
  named local variable.

<details>

- Walk through how `procedure!` defines a pattern that matches an identifier
  `$name`, followed by the literal token `BEGIN`, followed by a statement
  `$code`, a semicolon (which is required to separate a statement from the next
  syntactic construct) and then our `END` token.
- Note that even though the syntax is custom, the Rust compiler ensures that the
  inputs to macros are well-formed before passing them to the macro.

</details>
