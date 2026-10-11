<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Calling Unsafe Functions

Failing to uphold the safety requirements breaks memory safety!

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
const PK_BYTE_LEN: usize = 8;

fn log_public_key(pk_ptr: *const u16) {
    let pk: &[u16] = unsafe { std::slice::from_raw_parts(pk_ptr, PK_BYTE_LEN) };
    println!("{pk:?}");
}

fn main() {
    let pk: [u16; 4] = [1, 2, 3, 4]; // 8 bytes
    log_public_key(pk.as_ptr());
}
```

Always include a safety comment for each `unsafe` block. It must explain why the
code is actually safe. This example is missing a safety comment and is unsound.

<details>

Key points:

- The second argument to `slice::from_raw_parts` is the number of _elements_,
  not bytes! `PK_BYTE_LEN` is 8, but the array only contains four elements.
- The slice extends past the end of the array that the pointer was derived from,
  violating `slice::from_raw_parts`' safety requirements.
- Demo: Change `PK_BYTE_LEN` to `PK_ELEMENT_LEN` with a value of 4 to keep the
  slice within the bounds of the array.
- `log_public_key` should be unsafe, because `pk_ptr` must meet certain
  prerequisites to avoid undefined behaviour. A safe function which can cause
  undefined behaviour is said to be `unsound`. What should its safety
  documentation say?
- The standard library contains a number of low-level unsafe functions. Prefer
  the safe alternatives when possible!
- If you use an unsafe function as an optimization, make sure to add a benchmark
  to demonstrate the gain.

</details>
