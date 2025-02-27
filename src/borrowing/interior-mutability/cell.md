# `Cell`

`Cell` wraps a value and allows getting or setting the value using only a shared
reference to the `Cell`. However, it does not allow any references to the inner
value. Since there are no references, borrowing rules cannot be broken.

```rust,editable
use std::cell::Cell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);

    cell.set(123);
    dbg!(cell.get());
}
```

<details>

- `Cell` is a simple means to ensure safety: it has a `set` method that takes
  `&self`. This needs no runtime check, but requires moving values, which can
  have its own cost.

</details>
