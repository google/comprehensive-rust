# `buddy_system_allocator`

[`buddy_system_allocator`][1] is a third-party crate implementing a basic buddy
system allocator. It can be used both for [`LockedHeap`][2] implementing
[`GlobalAlloc`][3] so you can use the standard `alloc` crate (as we saw
[before][4]), or for allocating other address space. For example, we might want
to allocate MMIO space for PCI BARs:

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
{{#include allocator-example/src/main.rs:main}}
```

<details>

- PCI BARs always have alignment equal to their size.
- Run the example with `cargo run` under
  `src/bare-metal/useful-crates/allocator-example/`. (It won't run in the
  Playground because of the crate dependency.)

</details>

[1]: https://crates.io/crates/buddy_system_allocator
[2]: https://docs.rs/buddy_system_allocator/0.9.0/buddy_system_allocator/struct.LockedHeap.html
[3]: https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html
[4]: ../alloc.md
