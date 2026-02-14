---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Safe Rust

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub fn copy(dest: &mut [u8], source: &[u8]) {
    for (dest, src) in dest.iter_mut().zip(source) {
        *dest = *src;
    }
}

fn main() {
    let a = &[114, 117, 115, 116];
    let b = &mut [82, 85, 83, 84];

    println!("{}", String::from_utf8_lossy(b));
    copy(b, a);
    println!("{}", String::from_utf8_lossy(b));
}
```

<details>

“The implementation only uses safe Rust.

What can we learn from this?

“It is impossible for `copy` to trigger memory safety issues when implemented in
Safe Rust. This is true for all possible input arguments.”

“For example, by using Rust’s iterators, we can ensure that we’ll never trigger
errors relating to handling pointers directly, such as needing null pointer or
bounds checks.”

Ask: “Can you think of any others?”

- No aliasing issues
- Dangling pointers are impossible
- Alignment will be correct
- Cannot accidentally read from uninitialized memory

“We can say that the `copy` function is _sound_ because Rust ensures that all of
the safety preconditions are satisfied.”

“From the point of view of the programmer, as this function is implemented in
safe Rust, we can think of it as having no safety preconditions.”

“This does not mean that `copy` will always do what the caller might want. If
there is insufficient space available in the `dest` slice, then data will not be
copied across.”

</details>
