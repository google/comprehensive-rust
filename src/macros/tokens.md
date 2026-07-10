---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Macros, Tokens, and Syntax

Rather than manipulating raw text, Rust macros operate on **tokens** and **token
trees**.

- **Tokens** are the smallest indivisible lexical elements of the language
  (e.g., identifiers like `foo`, operators like `+`, punctuation like `,`, and
  literals like `123`).
- **Token trees** are formed by grouping streams of tokens and token groups.
- When a macro is called, the compiler passes it such a stream.
- The macro processes this input stream and must return another stream of
  tokens-and-groups that replaces the macro invocation.

This structure allows the compiler to validate lexical boundaries and ensures
that macros can never construct or return syntactically invalid, un-lexable
text.

<details>

- Reiterate that because macro expansion happens _during_ AST generation, the
  macro receives structured tokens, not characters.
- Explain that this differs completely from preprocessors (like C/C++), which
  perform string/text-level substitution before tokenization.
- Mention that this token-level manipulation is what makes Rust macros more
  robust and syntax-safe.

</details>
