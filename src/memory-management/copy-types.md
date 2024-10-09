---
minutes: 5
---

# Copy Types

While move semantics are the default, certain types are copied by default:

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}"); // would not be accessible if not Copy
    println!("y: {y}");
}
```

These types implement the `Copy` trait.

You can opt-in your own types to use copy semantics:

<!-- mdbook-xgettext: skip -->

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

- After the assignment, both `p1` and `p2` own their own data.
- We can also use `p1.clone()` to explicitly copy the data.

<details>

Copying and cloning are not the same thing:

- Copying refers to bitwise copies of memory regions and does not work on
  arbitrary objects.
- Copying does not allow for custom logic (unlike copy constructors in C++).
- Cloning is a more general operation and also allows for custom behavior by
  implementing the `Clone` trait.
- Copying does not work on types that implement the `Drop` trait.

In the above example, try the following:

- Add a `String` field to `struct Point`. It will not compile because `String`
  is not a `Copy` type.
- Remove `Copy` from the `derive` attribute. The compiler error is now in the
  `println!` for `p1`.
- Show that it works if you clone `p1` instead.

# More to Explore

- Shared references are `Copy`/`Clone`, mutable references are not. This is
  because Rust requires that mutable references be exclusive, so while it's
  valid to make a copy of a shared reference, creating a copy of a mutable
  reference would violate Rust's borrowing rules.

</details>
