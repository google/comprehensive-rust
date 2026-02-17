---
minutes: 5
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Mutable Static Variables

It is safe to read an immutable static variable:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("HELLO_WORLD: {HELLO_WORLD}");
}
```

However, mutable static variables are unsafe to read and write because multiple
threads could do so concurrently without synchronization, constituting a data
race.

Using mutable statics soundly requires reasoning about concurrency without the
compiler's help:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    // SAFETY: There are no other threads which could be accessing `COUNTER`.
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_counter(42);

    // SAFETY: There are no other threads which could be accessing `COUNTER`.
    unsafe {
        dbg!(COUNTER);
    }
}
```

<details>

- The program here is sound because it is single-threaded. However, the Rust
  compiler reasons about functions individually so can't assume that. Try
  removing the `unsafe` and see how the compiler explains that it is undefined
  behavior to access a mutable static from multiple threads.
- The 2024 Rust edition goes further and makes accessing a mutable static by
  reference an error by default.
- Using a mutable static is rarely a good idea, you should use interior
  mutability instead.
- There are some cases where it might be necessary in low-level `no_std` code,
  such as implementing a heap allocator or working with some C APIs. In this
  case you should use pointers rather than references.

</details>
