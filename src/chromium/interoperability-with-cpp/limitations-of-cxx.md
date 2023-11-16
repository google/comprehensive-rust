## Limitations of cxx

By far the most useful page when using cxx is the [type reference][1].

cxx fundamentally suits cases where:

* Your Rust-C++ interface is sufficiently simple that you can declare all of it.
* You're using only the types natively supported by cxx already, for example
  `std::unique_ptr`, `std::string`, `&[u8]` etc.

It has many limitations, some of which are quite frustrating - for example
lack of support for Rust's `Option` type.

These limitations constrain us to using Rust in Chromium only for well isolated
"leaf nodes" rather than for arbitrary Rust-C++ interop. When considering
a use-case for Rust in Chromium, a good starting point is to draft the cxx
bindings for the language boundary to see if it appears simple enough.

The best way to learn cxx is by doing, so, another exercise!


[1]: https://cxx.rs/bindings.html