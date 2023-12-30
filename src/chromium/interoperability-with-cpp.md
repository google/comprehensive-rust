# Interoperability with C++

The Rust community offers multiple options for C++/Rust interop, with new tools
being developed all the time. At the moment, Chromium uses a tool called CXX.

You describe your whole language boundary in an interface definition language
(which looks a lot like Rust) and then CXX tools generate declarations for
functions and types in both Rust and C++.

<img src="../android/interoperability/cpp/overview.svg" alt="Overview diagram of cxx, showing that the same interface definition is used to create both C++ and Rust side code which then communicate via a lowest common denominator C API">

See the [CXX tutorial][1] for a full example of using this.

[1]: https://cxx.rs/tutorial.html
[2]: https://cxx.rs/bindings.html

<details>

Talk through the diagram. Explain that behind the scenes, this is doing just the
same as you previously did. Point out that automating the process has the
following benefits:

- The tool guarantees that the C++ and Rust sides match (e.g. you get compile
  errors if the `#[cxx::bridge]` doesn't match the actual C++ or Rust
  definitions, but with out-of-sync manual bindings you'd get Undefined
  Behavior)
- The tool automates generation of FFI thunks (small, C-ABI-compatible, free
  functions) for non-C features (e.g. enabling FFI calls into Rust or C++
  methods; manual bindings would require authoring such top-level, free
  functions manually)
- The tool and the library can handle a set of core types - for example:
  - `&[T]` can be passed across the FFI boundary, even though it doesn't
    guarantee any particular ABI or memory layout. With manual bindings
    `std::span<T>` / `&[T]` have to be manually destructured and rebuilt out of
    a pointer and length - this is error-prone given that each language
    represents empty slices slightly differently)
  - Smart pointers like `std::unique_ptr<T>`, `std::shared_ptr<T>`, and/or `Box`
    are natively supported. With manual bindings, one would have to pass
    C-ABI-compatible raw pointers, which would increase lifetime and
    memory-safety risks.
  - `rust::String` and `CxxString` types understand and maintain differences in
    string representation across the languages (e.g. `rust::String::lossy` can
    build a Rust string from non-UTF8 input and `rust::String::c_str` can
    NUL-terminate a string).

</details>
