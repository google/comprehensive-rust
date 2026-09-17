# `for` Review

Previously when we looked at `for` loops, we saw that they can accept anything
that represents a sequence of values. Under the hood Rust uses a concept called
**iterators** to provide a unified way to walk through a sequence like this.

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    // Accepts a range.
    for x in 1..5 {
        dbg!(x);
    }

    // Also accepts an array.
    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }
}
```
