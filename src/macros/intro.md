---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# What Is A Macro

Before looking at macros in detail, it is helpful to build a conceptual model of
what macros are and how they fit into the compilation pipeline.

At their core, macros are **meta-programming**:

- They take input code (expressed as tokens) and emit output code.
- They execute during compilation, expanding before type checking and semantic
  analysis.
- They allow expressing patterns and abstractions that cannot be written with
  generics or standard functions.

We will explore how compilers process source code, and how different macro
systems hook into this process.

<details>

- Ask students if they have used macros in other languages (e.g., C/C++
  preprocessor, Lisp, or Clojure).

</details>
