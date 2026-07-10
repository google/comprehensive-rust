---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Declarative Macro Techniques

As your declarative macros become more sophisticated, simple patterns may not be
enough to express your intended logic.

Rust programmers have established several design patterns for structuring more
complex macros:

- **Optional Parameters:** Using multiple match arms or optional matchers to
  handle default values.
- **Named Parameters:** Explicitly labeling parameters at the callsite (e.g.,
  `key=value`) to improve readability.
- **Internal Rules:** Prefixing helper match arms with a unique symbol (like
  `@`) to modularize code.
- **Incremental Token Munchers:** Eating tokens off the front of a stream
  recursively.
- **Push-Down Accumulators:** Collecting tokens recursively into a buffer to
  build a structured output.
- **TT Bundling:** Grouping multiple arguments inside a single Token Tree to
  simplify passing them down.

We will consider each of these advanced techniques in turn.

<details>

- Reassure students that these patterns are standard idioms in advanced Rust
  code bases, but not necessary for simple macros. A programmer must use their
  discretion to choose between a complex declarative macro and implementing
  logic in a procedural macro, which may be more straightforward at the cost of
  additional ceremony.
- Explain that because declarative macros don't have standard local variables or
  mutable state during expansion, recursion and token-level manipulation are the
  primary ways to achieve complex code generation.

</details>
