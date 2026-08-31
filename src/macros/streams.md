---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Token Streams

A **token stream** is a sequence of token trees. It is the fundamental input and
output format for both declarative and procedural macros in Rust:

- **Macro Input:** The tokens passed inside the brackets of the macro
  invocation.
- **Macro Output:** The new tokens returned by the macro that replace the macro
  invocation.

For declarative macros, the output is structured not just as a token stream, but
also must be syntactically valid Rust. The generated syntax is inserted into the
AST of the program where the macro is invoked.

```rust
// The input token stream does not need to be valid Rust syntax, as long as it
// tokenizes (which requires any brackets to be balanced).
macro_rules! add_rpn {
    ($a:literal $b:literal +) => {
        $a + $b
    };
}

fn main() {
    // The macro returns tokens: 1 + 2
    // Since expansion inserts a structured AST fragment, precedence is preserved!
    println!("3 * 10 = {}", add_rpn!(2 1 +) * 10); // Outputs "30"
}
```

- Because macro output is inserted as an AST subtree, we don't need to wrap
  expansions in safety parentheses (unlike C macros, where literal replacement
  would cause `1 + 2 * 10` to equal `21`).

<details>

- This slide shows `macro_rules!` defining a declarative macro. We do not need
  to visit this in detail yet, as we will do so on the next slide.
- Explain that a Token Stream can contain arbitrary sequences of tokens. The
  tokens do not need to form valid Rust code on their own (for example,
  `special_print!(#foo(bar:123))` uses `#` and `:` in custom ways).
- When a macro expands, the _emitted_ token stream must make syntactic sense at
  the position where the macro is invoked.

</details>
