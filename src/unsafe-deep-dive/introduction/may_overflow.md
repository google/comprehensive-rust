---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Example: may_overflow function

```rust,should_panic,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Adds 2^31 - 1 to negative numbers without checking for overflow.
///
/// # Safety
///
/// Calling this with `a >= 1` causes signed integer overflow, which is
/// undefined behavior with `unchecked_add`.
unsafe fn may_overflow(a: i32) -> i32 {
    unsafe { a.unchecked_add(i32::MAX) }
}

fn main() {
    let x = unsafe { may_overflow(123) };
    println!("{x}");
}
```

<details>

“The `unsafe` keyword may have a subtly different meaning than what some people
assume.”

“The code author believes that the code is correct. In principle, the code is
safe.”

“In this toy example, the `may_overflow` function is only intended to be called
with negative numbers.

Ask learners if they can explain why `may_overflow` requires the unsafe keyword.

“In case you’re unsure what the problem is, let’s pause briefly to explain. An
`i32` only has 31 bits available for positive numbers.

In standard safe Rust, integer overflow (`a + i32::MAX`) is not undefined behavior:
it panics with overflow checks enabled (debug mode) and performs two's-complement
wrapping in release mode. However, `unchecked_add` is an `unsafe` operation that
omits overflow checks entirely and declares overflow to be undefined behavior (UB).
Compilers optimize code on the assumption that undefined behavior is impossible,
which can cause dead-code elimination and unexpected runtime behavior.

Compile and run the code: in debug mode, the standard library catches the violated
safety precondition and panics (`unsafe precondition(s) violated: i32::unchecked_add cannot overflow`).
In release mode, `unchecked_add` produces actual UB.

“This code can be used correctly, however, improper usage is highly dangerous.”

“And it's impossible for the compiler to verify that the usage is correct.”

This is what we mean when we say that the `unsafe` keyword marks the location
where responsibility for memory safety shifts from the compiler to the
programmer.

</details>
