# Copying and Cloning

While move semantics are the default, certain types are copied by default:

```rust,editable
fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");
}
```

These types implement the `Copy` trait.

You can opt-in your own types to use copy semantics:

```rust,editable
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
```

* After the assignment, both `p1` and `p2` own their own data.
* We can also use `p1.clone()` to explicitly copy the data.
