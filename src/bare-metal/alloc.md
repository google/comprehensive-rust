# `alloc`

To use `alloc` you must implement a global (heap) allocator.

```rust,editable
{{#include alloc-example/src/main.rs:Alloc}}
```

<details>

* `buddy_system_allocator` is a third-party crate implementing a basic buddy system allocator. Other
  crates are available, or you can write your own or hook into your existing allocator.
* The const parameter of `LockedHeap` is the max order of the allocator; i.e. in this case it can
  allocate regions of up to 2**32 bytes.

</details>
