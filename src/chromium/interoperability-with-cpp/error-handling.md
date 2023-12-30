# CXX Error Handling

CXX's [support for `Result<T,E>`][0] relies on C++ exceptions, so we can't use
that in Chromium. Alternatives:

- The `T` part of `Result<T, E>` can be:
  - Returned via out parameters (e.g. via `&mut T`). This requires that `T` can
    be passed across the FFI boundary - for example `T` has to be:
    - A primitive type (like `u32` or `usize`)
    - A type natively supported by `cxx` (like `UniquePtr<T>`) that has a
      suitable default value to use in a failure case (_unlike_ `Box<T>`).
  - Retained on the Rust side, and exposed via reference. This may be needed
    when `T` is a Rust type, which cannot be passed across the FFI boundary, and
    cannot be stored in `UniquePtr<T>`.

- The `E` part of `Result<T, E>` can be:
  - Returned as a boolean (e.g. `true` representing success, and `false`
    representing failure)
  - Preserving error details is in theory possible, but so far hasn't been
    needed in practice.

[0]: https://cxx.rs/binding/result.html
