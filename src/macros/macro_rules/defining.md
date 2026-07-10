---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Defining Macros By Example

A declarative macro consists of one or more **macro rules**, similar to how a
Rust `match` expression consists of a sequence of match arms:

```rust
#[rustfmt::skip]
macro_rules! my_macro {
    // Left-side: Matcher        Right-side: Transcriber
    (foo $bar:ident)       => {  fn $bar() {}  };
    (baz $quux:expr)       => {  println!("{}", $quux)  };
}
```

- **Macro Rule:** An arm of the macro containing a Matcher and a Transcriber
  separated by `=>`.
- **Matcher:** A pattern that matches an input token stream.
- **Transcriber:** The code segment template that expands when matched.
- **Meta-variables:** Variables defined on the left and expanded on the right,
  prefixed with `$` (e.g., `$bar`).
- **Fragment Specifiers:** Indicate to which syntactic category the
  meta-variable is restricted (e.g., `ident` or `expr`).

<details>

- Like `match` in normal Rust, the compiler attempts to match each rule
  sequentially, from top to bottom.
- If a rule matches the input token stream completely, it selects that arm and
  transcribes the output.
- If no arms match, or if any pattern is ambiguous when matching tokens (there
  is no backtrack or lookahead), the compiler emits an error.

</details>
