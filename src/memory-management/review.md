---
minutes: 5
---

# Review of Program Memory

Programs allocate memory in two ways:

- Stack: Continuous area of memory for local variables.
  - Values have fixed sizes known at compile time.
  - Extremely fast: just move a stack pointer.
  - Easy to manage: follows function calls.
  - Great memory locality.

- Heap: Storage of values outside of function calls.
  - Values have dynamic sizes determined at runtime.
  - Slightly slower than the stack: some book-keeping needed.
  - No guarantee of memory locality.

## Example

Creating a `String` puts fixed-sized metadata on the stack and dynamically sized
data, the actual string, on the heap:

```rust,editable
fn main() {
    let s1 = String::from("Hello");
}
```

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - -.
:                           :     :                               :
:    s1                     :     :                               :
:   +-----------+-------+   :     :   +----+----+----+----+----+  :
:   | ptr       |   o---+---+-----+-->| H  | e  | l  | l  | o  |  :
:   | len       |     5 |   :     :   +----+----+----+----+----+  :
:   | capacity  |     5 |   :     :                               :
:   +-----------+-------+   :     :                               :
:                           :     `- - - - - - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

<details>

- Mention that a `String` is backed by a `Vec`, so it has a capacity and length
  and can grow if mutable via reallocation on the heap.

- If students ask about it, you can mention that the underlying memory is heap
  allocated using the [System Allocator] and custom allocators can be
  implemented using the [Allocator API]

## More to Explore

We can inspect the memory layout with `unsafe` Rust. However, you should point
out that this is rightfully unsafe!

```rust,editable
fn main() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");
    // DON'T DO THIS AT HOME! For educational purposes only.
    // String provides no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (ptr, capacity, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
}
```

</details>

[System Allocator]: https://doc.rust-lang.org/std/alloc/struct.System.html
[Allocator API]: https://doc.rust-lang.org/std/alloc/index.html
