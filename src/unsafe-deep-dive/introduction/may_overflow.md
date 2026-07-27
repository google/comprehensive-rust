---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Example: a safety precondition

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Returns the element at `index` without bounds checking.
///
/// # Safety
///
/// The caller must guarantee that `index < slice.len()`.
unsafe fn element_at(slice: &[i32], index: usize) -> i32 {
    unsafe { *slice.get_unchecked(index) }
}

fn main() {
    let numbers = [10, 20, 30];
    // Safety: `1` is a valid index into `numbers`.
    let value = unsafe { element_at(&numbers, 1) };
    println!("{value}");
}
```

<details>

“`slice.get_unchecked(index)` returns the element at `index` without the bounds
check that `slice[index]` performs. It is faster, but it is only sound when
`index < slice.len()`.”

“The compiler cannot verify that every caller respects this rule, so
`element_at` is declared `unsafe` and states the rule in its `# Safety` section.
Callers acknowledge that responsibility by wrapping the call in an `unsafe`
block.”

“Inside `element_at`, the call to `get_unchecked` is wrapped in its own `unsafe`
block, because the body of an `unsafe fn` is a safe context — the inner block is
what marks the unsafe operation.”

Ask the learners what happens if a caller passes an out-of-bounds index. The
skipped check makes the access undefined behavior, so the safety precondition
would be violated. The `unsafe` keyword is what shifts responsibility for
upholding the precondition from the compiler to the programmer.

“Note that integer overflow does not need this treatment: in Rust it is well
defined (debug builds panic, release builds wrap), so it is never undefined
behavior and never requires `unsafe`.”

</details>
