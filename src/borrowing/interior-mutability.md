---
minutes: 10
---

# Interior Mutability

In some situations, it's necessary to modify data behind a shared (read-only)
reference. For example, a shared data structure might have an internal cache,
and wish to update that cache from read-only methods.

The "interior mutability" pattern allows exclusive (mutable) access behind a
shared reference. The standard library provides several ways to do this, all
while still ensuring safety, typically by performing a runtime check.

## `Cell`

`Cell` wraps a value and allows getting or setting the value using only a shared
reference to the `Cell`. However, it does not allow any references to the inner
value. Since there are no references, borrowing rules cannot be broken.

```rust,editable
use std::cell::Cell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);

    cell.set(123);
    println!("{}", cell.get());
}
```

## `RefCell`

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
        // println!("{}", *other);
    }

    println!("{cell:?}");
}
```

<details>

The main thing to take away from this slide is that Rust provides _safe_ ways to
modify data behind a shared reference. There are a variety of ways to ensure
that safety, and `RefCell` and `Cell` are two of them.

- `RefCell` enforces Rust's usual borrowing rules (either multiple shared
  references or a single exclusive reference) with a runtime check. In this
  case, all borrows are very short and never overlap, so the checks always
  succeed.

  - The extra block in the `RefCell` example is to end the borrow created by the
    call to `borrow_mut` before we print the cell. Trying to print a borrowed
    `RefCell` just shows the message `"{borrowed}"`.

- `Cell` is a simpler means to ensure safety: it has a `set` method that takes
  `&self`. This needs no runtime check, but requires moving values, which can
  have its own cost.

- Both `RefCell` and `Cell` are `!Sync`, which means `&RefCell` and `&Cell`
  can't be passed between threads. This prevents two threads trying to access
  the cell at once.

</details>
