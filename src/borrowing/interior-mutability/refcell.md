# `RefCell`

`RefCell` allows accessing and mutating a wrapped value by providing alternative
types `Ref` and `RefMut` that emulate `&T`/`&mut T` without actually being Rust
references.

These types perform dynamic checks using a counter in the `RefCell` to prevent
existence of a `RefMut` alongside another `Ref`/`RefMut`.

By implementing `Deref` (and `DerefMut` for `RefMut`), these types allow calling
methods on the inner value without allowing references to escape.

```rust,editable
use std::cell::RefCell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = RefCell::new(5);

    {
        let mut cell_ref = cell.borrow_mut();
        *cell_ref = 123;

        // This triggers an error at runtime.
        // let other = cell.borrow();
        // println!("{}", other);
    }

    println!("{cell:?}");
}
```

<details>

- `RefCell` enforces Rust's usual borrowing rules (either multiple shared
  references or a single exclusive reference) with a runtime check. In this
  case, all borrows are very short and never overlap, so the checks always
  succeed.

- The extra block in the example is to end the borrow created by the call to
  `borrow_mut` before we print the cell. Trying to print a borrowed `RefCell`
  just shows the message `"{borrowed}"`.

## More to Explore

There are also `OnceCell` and `OnceLock`, which allow initialization on first
use. Making these useful requires some more knowledge than students have at this
time.

</details>
