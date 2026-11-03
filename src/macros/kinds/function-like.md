---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Function-Like Macros

**Function-like macros** are invoked with an exclamation mark (`!`) following
their identifier. They look like function calls, but they can accept arbitrary
tokens inside their brackets.

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    // Invoked as an expression:
    let v = vec![1, 2, 3];

    // Invoked as a statement:
    println!("Hello, World! {:?}", v);
}
```

- **Flexible brackets:** Can be invoked with parentheses `()`, square brackets
  `[]`, or curly braces `{}`.
- **Input:** Can contain any sequence of Rust tokens, but unmatched
  opening/closing brackets are forbidden (so the compiler can tell where the
  macro invocation ends, regardless of how the macro processes those tokens).
- **Placement:** Can appear as expressions, statements, types, patterns, or
  items.
- **Implementation:** These are the only kind of macro that can be implemented
  as either a **declarative** macro (`macro_rules!`) or a **procedural** macro.

<details>

- Mention that while they look like function calls, they are expanded during
  compilation. They do not exist at runtime.
- The choice of brackets is a matter of stylistic convention: `vec!` uses `[]`
  because it behaves like an array, `println!` uses `()` because it behaves like
  a function call, and custom macros defining items often use `{}`.

</details>
