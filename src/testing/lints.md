---
minutes: 3
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Compiler Lints and Clippy

The Rust compiler produces fantastic error messages, as well as helpful built-in
lints. [Clippy](https://doc.rust-lang.org/clippy/) provides even more lints,
organized into groups that can be enabled per-project.

```rust,editable,should_panic,warnunused
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[deny(clippy::cast_possible_truncation)]
fn main() {
    let mut x = 3;
    while (x < 70000) {
        x *= 2;
    }
    println!("X probably fits in a u16, right? {}", x as u16);
}
```

<details>

There are compiler lints visible here, but not clippy lints. Run `clippy` on the
playground site to show clippy warnings. Clippy has extensive documentation of
its lints, and adds new lints (including default-deny lints) all the time.

Note that errors or warnings with `help: ...` can be fixed with `cargo fix` or
via your editor.

</details>
