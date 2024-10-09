# Exercise: Linked Lists

Linked lists are
[notoriously](https://rust-unofficial.github.io/too-many-lists/) awkward in
Rust. For this exercise, modify the following simple linked-list implementation
to take advantage of the niche optimization to avoid unnecessary allocations.
The resulting list type should use no more than 16*n bytes, where n is the
length of the list.

```rust,editable
{{#include exercise.rs}}
```
