---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Unsafe is sometimes necessary

The Rust compiler can only enforce its rules for code that it has compiled.

```rust,editable,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let pid = unsafe { libc::getpid() };
    println!("{pid}");
}
```

<details>

“There are some activities that _require_ unsafe.

“The Rust compiler cannot verify that external functions comply with Rust's
memory guarantees. Therefore, invoking external functions requires an unsafe
block.”

Optional:

“Working with the external environment frequently involves sharing memory. The
interface that computers provide is a memory address (a pointer).”

“Here's an example that asks the Linux kernel to write to memory that we
control:

```rust,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn main() {
    let mut buf = [0u8; 8];
    let ptr = buf.as_mut_ptr() as *mut libc::c_void;

    let status = unsafe { libc::getrandom(ptr, buf.len(), 0) };
    if status > 0 {
        println!("{buf:?}");
    }
}
```

“This FFI call reaches into the operating system to fill our buffer (`buf`). As
well as calling an external function, we must mark the boundary as `unsafe`
because the compiler cannot verify how the OS touches that memory.”

</details>
