# `tinyvec`

Sometimes you want something which can be resized like a `Vec`, but without heap
allocation. [`tinyvec`][1] provides this: a vector backed by an array or slice,
which could be statically allocated or on the stack, which keeps track of how
many elements are used and panics if you try to use more than are allocated.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
use tinyvec::{ArrayVec, array_vec};

fn main() {
    let mut numbers: ArrayVec<[u32; 5]> = array_vec!(42, 66);
    println!("{numbers:?}");
    numbers.push(7);
    println!("{numbers:?}");
    numbers.remove(1);
    println!("{numbers:?}");
}
```

<details>

- `tinyvec` requires that the element type implement `Default` for
  initialisation.
- The Rust Playground includes `tinyvec`, so this example will run fine inline.

</details>

[1]: https://crates.io/crates/tinyvec
