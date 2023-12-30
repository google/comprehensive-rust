# `zerocopy`

The [`zerocopy`][1] crate (from Fuchsia) provides traits and macros for safely
converting between byte sequences and other types.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
{{#include zerocopy-example/src/main.rs:main}}
```

This is not suitable for MMIO (as it doesn't use volatile reads and writes), but
can be useful for working with structures shared with hardware e.g. by DMA, or
sent over some external interface.

<details>

- `FromBytes` can be implemented for types for which any byte pattern is valid,
  and so can safely be converted from an untrusted sequence of bytes.
- Attempting to derive `FromBytes` for these types would fail, because
  `RequestType` doesn't use all possible u32 values as discriminants, so not all
  byte patterns are valid.
- `zerocopy::byteorder` has types for byte-order aware numeric primitives.
- Run the example with `cargo run` under
  `src/bare-metal/useful-crates/zerocopy-example/`. (It won't run in the
  Playground because of the crate dependency.)

</details>

[1]: https://docs.rs/zerocopy/
