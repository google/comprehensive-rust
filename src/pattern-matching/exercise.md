---
minutes: 15
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Expression Evaluation

Let's write a simple recursive evaluator for arithmetic expressions.

An example of a small arithmetic expression could be `10 + 20`, which evaluates
to `30`. We can represent the expression as a tree:

<!-- mdbook-xgettext: skip -->

```bob
            .-------.
    .------ |   +   | ------.
    |       '-------'       |
    v                       v
.--------.              .--------.
|   10   |              |   20   |
'--------'              '--------'
```

A bigger and more complex expression would be `(10 * 9) + ((3 - 4) * 5)`, which
evaluates to `85`. We represent this as a much bigger tree:

<!-- mdbook-xgettext: skip -->

```bob
                              .-----.
            .---------------- |  +  | ----------------.
            |                 '-----'                 |
            v                                         v
         .-----.                                   .-----.
   .---- |  *  | ----.                       .---- |  *  | ----.
   |     '-----'     |                       |     '-----'     |
   v                 v                       v                 v
.------.          .-----.                 .-----.           .-----.
|  10  |          |  9  |           .---- |  "-"| ----.     |  5  |
'------'          '-----'           |     '-----'     |     '-----'
                                    v                 v
                                 .-----.           .-----.
                                 |  3  |           |  4  |
                                 '-----'           '-----'
```

In code, we will represent the tree with two types:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:Operation}}

{{#include exercise.rs:Expression}}
```

The `Box` type here is a smart pointer, and will be covered in detail later in
the course. An expression can be "boxed" with `Box::new` as seen in the tests.
To evaluate a boxed expression, use the deref operator (`*`) to "unbox" it:
`eval(*boxed_expr)`.

Create a new Cargo library project with

```sh
cargo new --lib evaluator
```

Copy and paste the code below into a the `src/lib.rs` file.

Then begin implementing `eval`. Use `cargo test` to ensure that the final library
passes the tests. It may be helpful to use `todo!()` and get the tests to pass
one-by-one. You can also skip a test temporarily with `#[ignore]`:

```none
#[test]
#[ignore]
fn test_value() { .. }
```

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:Operation}}

{{#include exercise.rs:Expression}}

{{#include exercise.rs:eval}}
    todo!()
}

{{#include exercise.rs:tests}}
```
