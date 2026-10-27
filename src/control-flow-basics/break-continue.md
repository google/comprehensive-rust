---
minutes: 4
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `break` and `continue`

If you want to immediately start the next iteration use
[`continue`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions).

If you want to exit any kind of loop early, use
[`break`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions).
With `loop`, this can take an optional expression that becomes the value of the
`loop` expression.

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        dbg!(i);
    }
}
```

<details>

Note that `loop` is the only looping construct that may evaluate to a
non-trivial value. This is because control flow of a `loop` only proceeds to its
context via a `break` expression (unlike `while` and `for` loops, which can also
exit when the condition fails).

</details>
