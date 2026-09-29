---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# C Preprocessor vs. Rust Macros

The fundamental difference between C-style preprocessor macros and Rust macros
lies in **where they hook into compilation**:

### C Preprocessor Macros

- Hook into the **Tokenizing** step.
- Cannot inspect tokens in detail, but can rearrange and duplicate them (and
  sometimes append them).
- Can create syntactically invalid output.
- Lack scoping and hygiene, so constructing robust abstractions is difficult.

### Rust Macros

- Hook into the **Abstract Syntax Tree (AST)** generation step.
- Operate on structured **Token Trees** rather than raw text.
- Must emit valid Rust syntax.
- Maintain **partial hygiene** to prevent accidental name collisions.

<details>

- Explain that C macros must guess unused identifiers for internal use, and can
  access variables not mentioned in their arguments.
- Emphasize that C preprocessor macros don't have access to information from
  later in compilation, i.e. C syntax or types; they are basically dumb
  copy-paste.
- To provide better error reporting, the C Preprocessor is actually integrated
  into the Clang and GCC compilers, but its semantics also constrain it to be
  usable as a separate pass, so this integration can't be used to improve the
  behavior of C macros.
- Contrast this with Rust, where macro expansion is inherently part of the
  compilation process. This lets Rust macros work structurally.
- Introduce the concept of "partial hygiene," explaining that local variable
  bindings inside Rust macros do not bleed out into the surrounding code (and
  vice versa) by default.

</details>
