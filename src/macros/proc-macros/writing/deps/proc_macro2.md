---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# The `proc_macro2` Crate

The built-in `proc_macro` crate is tightly coupled to the compiler internals,
meaning its types cannot be instantiated or manipulated outside of the
compiler's plugin environment.

The **`proc_macro2`** library is a standard third-party crate that mirrors the
entire API of the built-in `proc_macro` crate, with some key advantages:

- **Unit testing:** You can write standard unit tests for your procedural macros
  because `proc_macro2::TokenStream` can be instantiated and asserted on in
  standard library tests.
- **Standalone execution:** It allows libraries to work with token streams in
  other execution contexts, such as build scripts (`build.rs`) or binary CLI
  tools.
- **Seamless interoperability:** Types from `proc_macro2` are easily converted
  to and from standard `proc_macro` types using `.into()`.

<details>

- Mention that `proc_macro2` is an essential dependency for almost any
  procedural macro crate.
- This is because testing is incredibly important for macro development, and
  `proc_macro2` makes it possible to write unit tests that parse strings into
  token streams and assert on the outputs.
- Popular libraries like `syn` and `quote` fully rely on `proc_macro2` types as
  their base inputs/outputs.

</details>
