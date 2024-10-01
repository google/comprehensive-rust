---
minutes: 30
---

# Exercise: Expression Evaluation

Let's write a simple recursive evaluator for arithmetic expressions.

The `Box` type here is a smart pointer, and will be covered in detail later in
the course. An expression can be "boxed" with `Box::new` as seen in the tests.
To evaluate a boxed expression, use the deref operator (`*`) to "unbox" it:
`eval(*boxed_expr)`.

Some expressions cannot be evaluated and will return an error. The standard
[`Result<Value, String>`](https://doc.rust-lang.org/std/result/enum.Result.html)
type is an enum that represents either a successful value (`Ok(Value)`) or an
error (`Err(String)`). We will cover this type in detail later.

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
