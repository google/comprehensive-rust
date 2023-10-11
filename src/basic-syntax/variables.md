# Variables

Rust provides type safety via static typing. Variable bindings are immutable by
default:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}");
}
```

<details>

* Due to type inference the `i32` is optional. We will gradually show the types less and less as the course progresses.

</details>
