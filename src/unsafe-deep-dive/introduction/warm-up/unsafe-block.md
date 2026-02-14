---
minutes: 8
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Using an unsafe block

```rust,editable,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let numbers = vec![0, 1, 2, 3, 4];
    let i = numbers.len() / 2;

    let x = *numbers.get_unchecked(i);
    assert_eq!(i, x);
}
```

<details>

Walk through the code. Confirm that the audience is familiar with the
dereference operator.

Attempt to compile the code, trigger the compiler error.

Add the unsafe block:

```rust
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
# fn main() {
#     let numbers = vec![0, 1, 2, 3, 4];
#     let i = numbers.len() / 2;
#
 let x = unsafe { *numbers.get_unchecked(i) };
#     assert_eq!(i, x);
# }
```

Prompt audience for a code review. Guide learners towards adding a safety
comment.

Add the safety comment:

```rust
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// SAFETY: `i` must be within 0..numbers.len()
```

_Suggested Solution_

```rust
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let numbers = vec![0, 1, 2, 3, 4];
    let i = numbers.len() / 2;

    let x = unsafe { *numbers.get_unchecked(i) };
    assert_eq!(i, x);
}
```

</details>
