---
minutes: 3
---

# Rust ↔ C

| Concern         | Rust                                  | C                                                   |
| --------------- | ------------------------------------- | --------------------------------------------------- |
| **Errors**      | `Result<T, E>`, `Option<T>`           | Magic return values, out-parameters, global `errno` |
| **Strings**     | `&str`/`String` (UTF-8, length-known) | Null-terminated `char*`, encoding undefined         |
| **Nullability** | Explicit via `Option<T>`              | Any pointer may be null                             |
| **Ownership**   | Affine types, lifetimes               | Conventions                                         |
| **Callbacks**   | `Fn`/`FnMut`/`FnOnce` closures        | Function pointer + `void* userdata`                 |
| **Panics**      | Stack unwinding (or abort)            | Abort                                               |

<details>

Errors: Must convert `Result` to abide by C conventions; easy to forget to check
errors on C side.

Strings: Conversion cost; null bytes in Rust strings cause truncation; UTF-8
validation on ingress.

Nullability: Every pointer from C must be checked to create an
`Option<NonNull<T>>`, implying unsafe blocks or runtime cost.

Ownership: Must document and enforce object lifetimes manually.

Callbacks: Must decompose closures into fn pointer + context; lifetime of
context is manual.

Panics: Panic across FFI boundary is undefined behavior; must catch at boundary
with `catch_unwind`.

</details>
