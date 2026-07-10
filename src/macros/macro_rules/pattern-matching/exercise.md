---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Saving Some Typing

In standard Rust, there is some unavoidable syntactic boilerplate when defining
functions (the `fn` keyword, specification of arguments and return types, and
the body). Using macros, we can reduce this boilerplate for repetitive function
definitions to just the tokens that might vary between the repetitive
definitions; the resulting boilerplate is only the handful of tokens required to
invoke a macro.

For example, if we define functions for various math operations (such as `add`,
`sub`, and `mul`), their types and general structure will be identical, with
only the name and operation varying.

In some contexts, we might avoid syntactic repetition by passing in closures,
e.g. `|a, b| a + b`, but this has the possibility of runtime overhead if the
closure is not inlined. Macros can similarly avoid repetition without the
possibility of runtime overhead.

In this exercise, you will implement a macro called `define_op!` to generate
arithmetic function definitions using pattern matching on identifiers and
operator tokens.

Recall that matching on arbitrary tokens in the input is achieved with
meta-variables, introduced via `$`, a variable name, and a fragment specifier,
e.g. `$var: ident` or `$e: expr`. We will cover the various fragment specifiers
in detail in the coming sections.

The syntax for `define_op!` should support accept input like this:

```rust,ignore
define_op! { add => + }
```

This invocation should expand to:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### Instructions:

- Implement the `define_op!` macro.
- Capture the function name as an `ident` and the arithmetic operator as a `tt`
  (Token Tree).

```rust,compile_fail,editable
// TODO: Implement the `define_op!` macro.

// Use the macro to define add, sub, and mul functions:
define_op! { add => + }
define_op! { sub => - }
define_op! { mul => * }

fn main() {
    println!("10 + 2 = {}", add(10, 2));
    println!("10 - 2 = {}", sub(10, 2));
    println!("10 * 2 = {}", mul(10, 2));

    assert_eq!(add(10, 2), 12);
    assert_eq!(sub(10, 2), 8);
    assert_eq!(mul(10, 2), 20);
}
```

<details>

- Explain that the `tt` specifier is useful for matching parts of syntax that
  are not free-standing expressions as token trees.
- This demonstrates code generation with less syntactic overhead than
  abstracting with functions or traits.

</details>
