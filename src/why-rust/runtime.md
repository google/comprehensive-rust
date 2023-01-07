# Runtime Guarantees

No undefined behavior at runtime:

* Array access is bounds checked.
* Integer overflow is defined.

<details>

Key points:

* Integer overflow is defined via a compile-time flag. The options are
  either a panic (a controlled crash of the program) or wrap-around
  semantics. By default, you get panics in debug mode (`cargo build`)
  and wrap-around in release mode (`cargo build --release`).

* Bounds checking cannot be disabled with a compiler flag. It can also
  not be disabled directly with the `unsafe` keyword. However,
  `unsafe` allows you to call functions such as `slice::get_unchecked`
  which does not do bounds checking.

</details>
