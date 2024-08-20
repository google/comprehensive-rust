---
minutes: 4
---

# `if` expressions

You use
[`if` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-expressions)
exactly like `if` statements in other languages:

```rust,editable
fn main() {
    let x = 10;
    if x == 0 {
        println!("zero!");
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

An `if` expression should be used in the same way as the other expressions. For
example, when it is used in a `let` statement, the statement must be terminated
with a `;` as well. Remove the `;` before `println!` to see the compiler error.

</details>
