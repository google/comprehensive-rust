# `alloc`

To use `alloc` you must implement a
[global (heap) allocator](https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html).

```rust,editable,compile_fail
{{#include alloc-example/src/main.rs:Alloc}}
```

<details>

- `buddy_system_allocator` is a third-party crate implementing a basic buddy
  system allocator. Other crates are available, or you can write your own or
  hook into your existing allocator.
- The const parameter of `LockedHeap` is the max order of the allocator; i.e. in
  this case it can allocate regions of up to 2**32 bytes.
- If any crate in your dependency tree depends on `alloc` then you must have
  exactly one global allocator defined in your binary. Usually this is done in
  the top-level binary crate.
- `extern crate panic_halt as _` is necessary to ensure that the `panic_halt`
  crate is linked in so we get its panic handler.
- This example will build but not run, as it doesn't have an entry point.

</details>
