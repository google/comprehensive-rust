---
minutes: 15
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold to avoid undefined behaviour.

Unsafe functions may come from two places:

- Rust functions declared unsafe.
- Unsafe foreign functions in `extern "C"` blocks.

<details>

We will look at the two kinds of unsafe functions next.

</details>
