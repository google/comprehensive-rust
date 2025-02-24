---
minutes: 5
---

# Tuples

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}
```

<details>

- Like arrays, tuples have a fixed length.

- Tuples group together values of different types into a compound type.

- Fields of a tuple can be accessed by the period and the index of the value,
  e.g. `t.0`, `t.1`.

- The empty tuple `()` is referred to as the "unit type" and signifies absence
  of a return value, akin to `void` in other languages.

</details>
