---
minutes: 5
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Loops

There are three looping keywords in Rust: `while`, `loop`, and `for`:

## `while`

The
[`while` keyword](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-loops)
works much like in other languages, executing the loop body as long as the
condition is true.

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    dbg!(x);
}
```
