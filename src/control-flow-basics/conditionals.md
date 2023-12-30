---
minutes: 5
---

# Conditionals

Much of the Rust syntax will be familiar to you from C, C++ or Java:

- Blocks are delimited by curly braces.
- Line comments are started with `//`, block comments are delimited by
  `/* ... */`.
- Keywords like `if` and `while` work the same.
- Variable assignment is done with `=`, comparison is done with `==`.

## `if` expressions

You use
[`if` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-expressions)
exactly like `if` statements in other languages:

```rust,editable
fn main() {
    let x = 10;
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}
```

In addition, you can use `if` as an expression. The last expression of each
block becomes the value of the `if` expression:

```rust,editable
fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}
```

<details>

Because `if` is an expression and must have a particular type, both of its
branch blocks must have the same type. Show what happens if you add `;` after
`"small"` in the second example.

When `if` is used in an expression, the expression must have a `;` to separate
it from the next statement. Remove the `;` before `println!` to see the compiler
error.

</details>
