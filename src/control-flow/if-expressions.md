# `if` expressions

You use `if` very similarly to how you would in other languages:

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

In addition, you can use it as an expression. This does the same as above:

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
