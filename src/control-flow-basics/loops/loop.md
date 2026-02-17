<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `loop`

The [`loop` statement](https://doc.rust-lang.org/std/keyword.loop.html) just
loops forever, until a `break`.

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}
```

<details>

- The `loop` statement works like a `while true` loop. Use it for things like
  servers that will serve connections forever.

</details>
