---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Defining Unsafe Rust

<!-- mdbook-xgettext: skip -->

```bob
╭───────────────────────────────────────────────────────────╮
│╭─────────────────────────────────────────────────────────╮│
││                                                         ││
││  Safe                                                   ││
││  Rust                                                   ││
││                                                         ││
││                                                         ││
│╰─────────╮                                               ││
│          │                                               ││
│  Unsafe  │                                               ││
│   Rust   │                                               ││
│          ╰───────────────────────────────────────────────╯│
╰───────────────────────────────────────────────────────────╯
```

<details>

“Unsafe Rust is a superset of Safe Rust.”

“Unsafe Rust adds extra capabilities, such as allowing you to dereference raw
pointers and call functions that can break Rust’s safety guarantees if called
incorrectly.”

“These extra capabilities are referred to as _unsafe operations_.”

“Unsafe operations provide the foundation that the Rust standard library is
built on. For example, without the ability to dereference a raw pointer, it
would be impossible to implement `Vec` or `Box`.”

“The compiler will still assist you while writing Unsafe Rust. Borrow checking
and type safety still apply. Unsafe operations have their own rules, which we’ll
learn about in this class.”

</details>
