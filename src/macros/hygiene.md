---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Hygiene

A subtle aspect of macro systems is their degree of **hygiene**, or independence
from the lexical environment of their expansion.

Macro hygiene enables macros to avoid accidentally being influenced by or
polluting the scope of the code surrounding their call sites.

In this section, we will cover:

- What macro hygiene is and why it is important.
- How unhygienic macros can result in bugs or impede understanding code.
- The extent to which Rust macros are hygienic and the how partial hygiene in
  Rust macros works.

<details>

- Explain that in many preprocessor-based languages (such as C), macros are
  completely unhygienic, operating solely on raw tokens and potentially
  interacting with the lexical environment differently at each expansion. This
  means that the ability to reason about macros agnostic of the context in which
  they will expand is extremely limited. This can lead to bugs when names used
  in macros coincide with names used at their call sites.
- The notion of hygiene may be familiar to students who know LISP, as it
  famously exhibits fully hygienic macros.
- This slide serves as a transition into the detailed discussion of macro
  hygiene.

</details>
