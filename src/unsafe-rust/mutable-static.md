---
minutes: 5
---

# Mutable Static Variables

It is safe to read an immutable static variable:

```rust,editable
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("HELLO_WORLD: {HELLO_WORLD}");
}
```

However, since data races can occur, it is unsafe to read and write mutable
static variables:

```rust,editable
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    // SAFETY: There are no other threads which could be accessing `COUNTER`.
    #[allow(static_mut_refs)]
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_counter(42);

    // SAFETY: There are no other threads which could be accessing `COUNTER`.
    #[allow(static_mut_refs)]
    unsafe {
        dbg!(COUNTER);
    }
}
```

<details>

- The program here is safe because it is single-threaded. However, the Rust
  compiler reasons about functions individually so can't assume that. Try
  removing the `unsafe` and see how the compiler explains that it is undefined
  behavior to access a mutable static from multiple threads.
- Rust 2024 edition goes further and makes accessing a mutable static by
  reference an error by default. We work around this in the example with
  `#[allow(static_mut_refs)]`. Don't do this.
- Using a mutable static is almost always a bad idea, you should use interior
  mutability instead.
- There are some cases where it might be necessary in low-level `no_std` code,
  such as implementing a heap allocator or working with some C APIs. In this
  case you should use pointers rather than references.

</details>
