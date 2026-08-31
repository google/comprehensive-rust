---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Ways To Define Macros

There are two separate ways of implementing macros in Rust. Each has distinct
advantages and trade-offs:

| Feature      | Declarative Macros                                                               | Procedural Macros                                                                                |
| ------------ | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Usage**    | Function-like macros only                                                        | All three kinds (Derive, Attr, Function-like)                                                    |
| **Location** | Implemented within your normal crate                                             | Must be defined in a separate `proc-macro` crate                                                 |
| **Pros**     | - Low boilerplate<br>- No extra compilation step<br>- Easy to write and reuse    | - Extremely powerful and expressive<br>- Written in standard Rust<br>- Full programmatic control |
| **Cons**     | - Bespoke pattern-matching syntax<br>- Cannot inspect arbitrary token structures | - Can slow down build time<br>- Substantial boilerplate required                                 |
| **Hygiene**  | Partially/mixed hygienic by default                                              | Configurable / custom hygiene                                                                    |

<details>

- Explain that procedural macros are literally compiled as libraries and
  executed _inside_ the compiler while compiling the consuming code. This is why
  they require a separate crate.
- Emphasize that you should always prefer declarative macros for simple code
  generation due to their much smaller impact on build times.

</details>
