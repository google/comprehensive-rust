---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Crying Wolf

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub unsafe fn copy(dest: &mut [u8], source: &[u8]) {
    for (dest, src) in dest.iter_mut().zip(source) {
        *dest = *src;
    }
}

fn main() {
    let a = &[114, 117, 115, 116];
    let b = &mut [82, 85, 83, 84];

    println!("{}", String::from_utf8_lossy(b));
    unsafe { copy(b, a) };
    println!("{}", String::from_utf8_lossy(b));
}
```

<details>

“It is also possible to create so-called crying wolf functions.

“These are functions which are tagged as unsafe, but which have no safety
preconditions that programmers need to check.

</details>
