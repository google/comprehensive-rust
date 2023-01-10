# Variables

Rust provides type safety via static typing. Variable bindings are immutable by
default:

```rust,editable
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}
```

<details>

* It might be worth pointing out quickly that due to type inference the `i32` is optional
* Note that since `println!` is a macro, x is not moved, even using the function like syntax of `println!("x: {}", x)`

</details>
