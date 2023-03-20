# `tinyvec`

Sometimes you want something which can be resized like a `Vec`, but without heap allocation.
[`tinyvec`][1] provides this: a vector backed by an array or slice, which could be statically
allocated or on the stack, which keeps track of how many elements are used and panics if you try to
use more than are allocated.

```rust,editable,compile_fail
{{#include tinyvec-example/src/main.rs:main}}
```

<details>

* `tinyvec` requires that the element type implement `Default` for initialisation.

</details>

[1]: https://crates.io/crates/tinyvec
