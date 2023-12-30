---
minutes: 10
---

# Exclusive References

Exclusive references, also known as mutable references, allow changing the value
they refer to. They have type `&mut T`.

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}
```

<details>

Key points:

- "Exclusive" means that only this reference can be used to access the value. No
  other references (shared or exclusive) can exist at the same time, and the
  referenced value cannot be accessed while the exclusive reference exists. Try
  making an `&point.0` or changing `point.0` while `x_coord` is alive.

- Be sure to note the difference between `let mut x_coord: &i32` and
  `let x_coord: &mut i32`. The first one represents a shared reference which can
  be bound to different values, while the second represents an exclusive
  reference to a mutable value.

</details>
