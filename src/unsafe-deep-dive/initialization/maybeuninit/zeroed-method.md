<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# MaybeUninit::zeroed()

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::mem::{MaybeUninit, transmute};

fn main() {
    let mut x = [const { MaybeUninit::<u32>::zeroed() }; 10];

    x[6].write(7);

    // SAFETY: All values of `x` have been written to
    let x: [u32; 10] = unsafe { transmute(x) };
    println!("{x:?}")
}
```

<details>

“MaybeUninit<T>::zeroed() is an alternative constructor to
MaybeUninit<T>::uninit(). It instructs the compiler to fill the bits of T with
zeros.”

Q: “Although the memory has been written to, the type remains `MaybeUninit<T>`.
Can anyone think of why?”

A: Some types require their values to be non-zero or non-null. The classic case
is references, but this applies to many other types as well. Consider the
`NonZeroUsize` integer type and others in its family.

</details>
