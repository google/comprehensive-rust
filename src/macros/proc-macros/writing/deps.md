---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Dependencies

Here is how the standard dependency tree of a modern procedural macro crate is
structured:

```ignore
your_proc_macro_crate
   │
   ▼
proc_macro  (Provided by the compiler)
   │
   ▼
proc_macro2 (Wraps proc_macro to allow out-of-compiler executions)
   │
   ▼
 ┌─┴─────────────┐
 ▼               ▼
syn            quote
(AST Parser)   (Code Generator)
```

- Each library builds on top of the layer below.
- Using `syn` and `quote` increases macro build times slightly, but greatly
  simplifies implementation, making macro code safer and much easier to
  maintain.

<details>

- Explain that `proc_macro` is provided by the standard distribution, but is
  restricted to compiler execution.
- `proc_macro2` is a standard third-party crate that replicates all `proc_macro`
  types, but does not depend on the compiler's private internals. This is why
  testing libraries or other tools can work with token streams.
- `syn` and `quote` operate fully on `proc_macro2` types, allowing seamless
  integration.

</details>
