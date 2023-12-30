# Exercise: Interoperability with C++

## Part one

- In the Rust file you previously created, add a `#[cxx::bridge]` which
  specifies a single function, to be called from C++, called `hello_from_rust`,
  taking no parameters and returning no value.
- Modify your previous `hello_from_rust` function to remove `extern "C"` and
  `#[no_mangle]`. This is now just a standard Rust function.
- Modify your `gn` target to build these bindings.
- In your C++ code, remove the forward-declaration of `hello_from_rust`.
  Instead, include the generated header file.
- Build and run!

## Part two

It's a good idea to play with CXX a little. It helps you think about how
flexible Rust in Chromium actually is.

Some things to try:

- Call back into C++ from Rust. You will need:
  - An additional header file which you can `include!` from your `cxx::bridge`.
    You'll need to declare your C++ function in that new header file.
  - An `unsafe` block to call such a function, or alternatively specify the
    `unsafe` keyword in your `#[cxx::bridge]` [as described here][0].
  - You may also need to
    `#include "third_party/rust/cxx/v1/crate/include/cxx.h"`
- Pass a C++ string from C++ into Rust.
- Pass a reference to a C++ object into Rust.
- Intentionally get the Rust function signatures mismatched from the
  `#[cxx::bridge]`, and get used to the errors you see.
- Intentionally get the C++ function signatures mismatched from the
  `#[cxx::bridge]`, and get used to the errors you see.
- Pass a `std::unique_ptr` of some type from C++ into Rust, so that Rust can own
  some C++ object.
- Create a Rust object and pass it into C++, so that C++ owns it. (Hint: you
  need a `Box`).
- Declare some methods on a C++ type. Call them from Rust.
- Declare some methods on a Rust type. Call them from C++.

## Part three

Now you understand the strengths and limitations of CXX interop, think of a
couple of use-cases for Rust in Chromium where the interface would be
sufficiently simple. Sketch how you might define that interface.

## Where to find help

- The [`cxx` binding reference][1]
- The [`rust_static_library` gn template][2]

<details>
As students explore Part Two, they're bound to have lots of questions about how
to achieve these things, and also how CXX works behind the scenes.

Some of the questions you may encounter:

- I'm seeing a problem initializing a variable of type X with type Y, where X
  and Y are both function types. This is because your C++ function doesn't quite
  match the declaration in your `cxx::bridge`.
- I seem to be able to freely convert C++ references into Rust references.
  Doesn't that risk UB? For CXX's _opaque_ types, no, because they are
  zero-sized. For CXX trivial types yes, it's _possible_ to cause UB, although
  CXX's design makes it quite difficult to craft such an example.

</details>

[0]: https://cxx.rs/extern-c++.html#functions-and-member-functions
[1]: https://cxx.rs/bindings.html
[2]: https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_static_library.gni;l=16
