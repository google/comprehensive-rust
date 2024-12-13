---
minutes: 20
---

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
evaluate to `85`. We represent this as a much bigger tree:

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

```rust
{{#include exercise.rs:Operation}}

{{#include exercise.rs:Expression}}
```

The `Box` type here is a smart pointer, and will be covered in detail later in
the course. An expression can be "boxed" with `Box::new` as seen in the tests.
To evaluate a boxed expression, use the deref operator (`*`) to "unbox" it:
`eval(*boxed_expr)`.

Copy and paste the code into the Rust playground, and begin implementing `eval`.
The final product should pass the tests. It may be helpful to use `todo!()` and
get the tests to pass one-by-one. You can also skip a test temporarily with
`#[ignore]`:

```none
#[test]
#[ignore]
fn test_value() { .. }
```

```rust
{{#include exercise.rs:Operation}}

{{#include exercise.rs:Expression}}

{{#include exercise.rs:eval}}
    todo!()
}

{{#include exercise.rs:tests}}
```
