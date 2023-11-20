## Limitations of cxx

By far the most useful page when using cxx is the [type reference][1].

cxx fundamentally suits cases where:

* Your Rust-C++ interface is sufficiently simple that you can declare all of it.
* You're using only the types natively supported by cxx already, for example
  `std::unique_ptr`, `std::string`, `&[u8]` etc.

It has many limitations - for example lack of support for Rust's `Option` type.

These limitations constrain us to using Rust in Chromium only for well isolated
"leaf nodes" rather than for arbitrary Rust-C++ interop. When considering
a use-case for Rust in Chromium, a good starting point is to draft the cxx
bindings for the language boundary to see if it appears simple enough.


[1]: https://cxx.rs/bindings.html

<details>
In addition, right now, Rust code in one component cannot depend on Rust
code in another, due to linking details in our component build. That's another
reason to restrict Rust to use in leaf nodes.

You should also discuss some of the other sticky points with cxx, for example:

* Its error handling is based around C++ exceptions (given on the next slide)
* Function pointers are awkward to use.

</details>