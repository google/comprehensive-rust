---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# How Do Macros Work

To understand how macros work, it helps to look at the pipeline of how a
compiler processes your code:

1. **Source Code:** The raw text files written by the programmer, consisting of
   a sequence of characters in some character set (in Rust, Unicode characters
   in UTF-8).
2. **Lexical Analysis (Lexing):** The compiler delimits the raw text into a
   sequence of **tokens** (e.g., `fn`, `main`, identifiers, punctuation,
   literals).
3. **Syntactic Analysis (Parsing):** Based on the language's grammar, the
   compiler organizes the sequence of tokens into an **Abstract Syntax Tree
   (AST)** that captures the hierarchical structure of the program.

| Source Code    | Tokenized Rust (Approx.)                             | AST Rust (Approx.)          |
| -------------- | ---------------------------------------------------- | --------------------------- |
| `fn main() {}` | `Ident: fn`, `Ident: main`, `Group: ()`, `Group: {}` | `Fn: ident=main, block=...` |

Different macro systems hook into different parts of this pipeline, offering
different tradeoffs between expressiveness and safety.

We'll look at the macro system used in C and C++ via the C Preprocessor, which
is well-known, and contrast this with Rust's macro system.

<details>

- Explain that the AST is a structured tree that represents what the program
  actually means semantically.

</details>
