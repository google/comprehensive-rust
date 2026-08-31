---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Brackets and Token Trees

A **token tree** can represent either a single token, or a **group** of tokens
enclosed by matching delimiters (brackets):

- **Single tokens (leaf nodes):** `foo`, `+`, `,`, `123`.
- **Grouped tokens (internal nodes):** Enclosed by parentheses `()`, braces
  `{}`, or square brackets `[]`.

For example, the token stream: `foo + (bar * baz)`

Is parsed into **3 separate token trees**:

1. `foo` (a single token)
2. `+` (a single token)
3. `(bar * baz)` (a token group containing three child token trees: `bar`, `*`,
   and `baz`)

Because grouping happens _during_ lexical analysis, **unbalanced groups are
strictly disallowed**. You cannot pass unbalanced parentheses or braces into or
out of a macro!

<details>

- Note that this means you cannot use a macro to generate half a block, like
  `let x = {` and close it with another macro or tokens outside the macro. The
  entire block must be passed or returned as a single, well-formed token group.

</details>
