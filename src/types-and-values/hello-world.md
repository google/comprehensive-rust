---
minutes: 5
---

<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Hello, World

Let us jump into the simplest possible Rust program, a classic Hello World
program:

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    println!("Hello 🌍!");
}
```

What you see:

- Functions are introduced with `fn`.
- The `main` function is the entry point of the program.
- Blocks are delimited by curly braces like in C and C++.
- Statements end with `;`.
- `println` is a macro, indicated by the `!` in the invocation.
- Rust strings are UTF-8 encoded and can contain any Unicode character.

<details>

This slide tries to make the students comfortable with Rust code. They will see
a ton of it over the next four days so we start small with something familiar.

Key points:

- Rust is very much like other languages in the C/C++/Java tradition. It is
  imperative and it doesn't try to reinvent things unless absolutely necessary.

- Rust is modern with full support for Unicode.

- Rust uses macros for situations where you want to have a variable number of
  arguments (no function [overloading](../control-flow-basics/functions.md)).

- `println!` is a macro because it needs to handle an arbitrary number of
  arguments based on the format string, which can't be done with a regular
  function. Otherwise it can be treated like a regular function.

- Rust is multi-paradigm. For example, it has powerful
  [object-oriented programming features](https://doc.rust-lang.org/book/ch17-00-oop.html),
  and, while it is not a functional language, it includes a range of
  [functional concepts](https://doc.rust-lang.org/book/ch13-00-functional-features.html).

</details>
