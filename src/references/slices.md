---
minutes: 7
---

# Slices

A slice gives you a view into a larger collection:

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}
```

- Slices borrow data from the sliced type.

<details>

- We create a slice by borrowing `a` and specifying the starting and ending
  indexes in brackets.

- If the slice starts at index 0, Rust’s range syntax allows us to drop the
  starting index, meaning that `&a[0..a.len()]` and `&a[..a.len()]` are
  identical.

- The same is true for the last index, so `&a[2..a.len()]` and `&a[2..]` are
  identical.

- To easily create a slice of the full array, we can therefore use `&a[..]`.

- `s` is a reference to a slice of `i32`s. Notice that the type of `s`
  (`&[i32]`) no longer mentions the array length. This allows us to perform
  computation on slices of different sizes.

- Slices always borrow from another object. In this example, `a` has to remain
  'alive' (in scope) for at least as long as our slice.

- You can't "grow" a slice once it's created:
  - You can't append elements of the slice, since it doesn't own the backing
    buffer.
  - You can't grow a slice to point to a larger section of the backing buffer. A
    slice does not have information about the length of the underlying buffer
    and so you can't know how large the slice can be grown.
  - To get a larger slice you have to go back to the original buffer and create
    a larger slice from there.

</details>
