<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

- **Safety Comments:** Each `unsafe` block is preceded by a `// SAFETY:` comment
  explaining why the operation is safe. This is standard practice in Rust to aid
  auditing.
- **String conversions:** The code demonstrates the conversions required for
  FFI:
  - `&str` -> `CString`: To create a null-terminated string for C.
  - `CString` -> `*const c_char`: To pass the pointer to C.
  - `*const c_char` -> `&CStr`: To wrap the returned C string.
  - `&CStr` -> `&[u8]` -> `&OsStr` -> `OsString`: To convert the bytes back to a
    Rust OS string.
- **RAII (`Drop`):** We implement `Drop` to call `closedir` automatically when
  the iterator goes out of scope. This ensures we don't leak file descriptors.
- **Iterator Interface:** We wrap the C API in a Rust `Iterator`, providing a
  safe and idiomatic interface (`next` returns `Option<OsString>`) to the
  underlying unsafe C functions.

<details>

- Explain that `CString` owns the data (like `String`), while `CStr` is a
  borrowed reference (like `&str`).
- The `OsStrExt` trait is needed on Unix systems to convert bytes directly to
  `OsStr`.

</details>
