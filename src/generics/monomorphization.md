# Monomorphization

Generic code is turned into non-generic code based on the call sites:

```rust,editable
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

behaves as if you wrote

```rust,editable
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

This is a zero-cost abstraction: you get exactly the same result as if you had
hand-coded the data structures without the abstraction.
