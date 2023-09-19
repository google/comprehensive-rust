# `if` expressions

You use [`if`
expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-expressions)
exactly like `if` statements in other languages:

```rust,editable
fn main() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
}
```

In addition, you can use `if` as an expression. The last expression of each
block becomes the value of the `if` expression:


```rust,editable
fn main() {
    let mut x = 10;
    x = if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    };
}
```

<details>

Because `if` is an expression and must have a particular type, both of its branch blocks must have the same type. Consider showing what happens if you add `;` after `x / 2` in the second example.
    
</details>
