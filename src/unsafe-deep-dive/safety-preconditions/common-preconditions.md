---
minutes: 6
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Common safety preconditions

- Aliasing and Mutability
- Alignment
- Array access is in-bound
- Initialization
- Lifetimes
- Pointer provenance
- Validity
- Memory

<details>

Avoid spending too much time explaining every precondition: we will be working
through the details during the course. The intent is to show that there are
several.

"An incomplete list, but these are a few of the major safety preconditions to
get us thinking."

- Validity. Values must be valid values of the type that they represent. Rust's
  references may not be null. Creating one with `unsafe` causes the.
- Alignment. References to values must be well-aligned, which means th
- Aliasing. All Rust code must uphold Rust's borrowing rules. If you are
  manually creating mutable references (`&mut T`) from pointers, then you may
  only create one
- Initialization. All instances of Rust types must be fully initialized. To
  create a value from raw memory, we need to make sure that we've written
- Pointer provenance. The origin of a pointer is important. Casting a `usize` to
  a raw pointer is no longer allowed.
- Lifetimes. References must not outlive their referent.

Some conditions are even more subtle than they first seem.

Consider "in-bounds array access". Reading from the memory location, i.e.
dereferencing, is not required to break the program. Creating an out-of-bounds
reference already breaks the compiler's assumptions, leading to erratic
behavior.

Rust tells LLVM to use its `getelementptr inbounds` assumption. That assumption
will cause later optimization passes within the compiler to misbehave (because
out-of-bounds memory access cannot occur).

Optional: open [the playground][1], which shows the code below. Explain that
this is essentially a C function written in Rust syntax that gets items from an
array. Generate the LLVM IR with the **Show LLVM IR** button. Highlight
`getelementptr inbounds i32, ptr %array, i64 %offset`.

```rust,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[unsafe(no_mangle)]
pub unsafe fn get(array: *const i32, offset: isize) -> i32 {
    unsafe { *array.offset(offset) }
}
```

Expected output (the line to highlight starts with `%_3):

```llvm
define noundef i32 @get(ptr noundef readonly captures(none) %array, i64 noundef %offset) unnamed_addr #0 {
start:
  %_3 = getelementptr inbounds i32, ptr %array, i64 %offset
  %_0 = load i32, ptr %_3, align 4, !noundef !3
  ret i32 %_0
}
```

[1]: https://play.rust-lang.org/?version=stable&mode=release&edition=2024&gist=4116c4de01c863cac918f193448210b1

Bounds: You correctly noted that creating an out-of-bounds pointer (beyond the
"one-past-the-end" rule) is UB, even without dereferencing, due to LLVM's
inbounds assumptions.

</details>
