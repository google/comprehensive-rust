# Interoperability with C++

The Rust community offers multiple options for C++/Rust interop, with new
being developed all the time. At the moment, Chromium uses a tool called "cxx".

Behind the scenes, this uses a "lowest common denominator" C ABI, but it builds
nice abstractions on both sides.

The overall approach looks like this:

<img src="../android/interoperability/cpp/overview.svg">

See the [CXX tutorial][1] for an full example of using this.

cxx requires you to declare the whole C++/Rust boundary in one of your `.rs`
files. For instance:

```rust
// src/main.rs

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
    }
}
```

Note:
* Native support for C++'s `std::unique_ptr` in Rust
* Native support for Rust slices in C++
* Calls from C++ to Rust (in the top part)
* Calls from Rust to C++, and C++ types (in the bottom part)

**Common misconception**: It _looks_ like a C++ header is being parser by Rust,
but this is misleading. This header is never interpreted by Rust, but simply
`#include`d in the generated C++ code for the benefit of C++ compilers.

## cxx reference

By far the most useful page when using cxx is the [type reference][2].

## Using cxx from Chromium

The `rust_static_library` target intrinsically knows about cxx interop. Just add

```gn
cxx_bindings = [ "my_rust_file.rs" ]
```

to your existing `rust_static_library` target. **You also need
`allow_unsafe = true`**.

C++ headers will be generated at a sensible location, so you can just

```cpp
#include "ui/base/my_rust_file.rs.h"
```

and because you probably already depend on the Rust component from the C++
component, you can just go ahead and use them - all the usual build rule things
like visibility and gn dependencies are automatically set up.

## Limitations of cxx

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

<details>
Students may ask - why do we still need `allow_unsafe = true`?

The broad answer is that no C/C++ code is "safe" by the normal Rust standards.
Calling back and forth C/C++ from Rust may do arbitrary things to memory, and
compromise the safety of Rust's own data layouts. Presence of _too many_
`unsafe` keywords in C/C++ interop can harm the signal-to-noise ratio of
such a keyword, and is [controversial][3], but strictly, bringing any foreign
code into a Rust binary can cause unexpected behavior from Rust's perspective.

The narrow answer lies in the diagram at the top of this page - behind the
scenes, cxx generates Rust `unsafe` and `extern "C"` functions just like
we did manually in the previous section.
</details>

[1]: https://cxx.rs/tutorial.html
[2]: https://cxx.rs/bindings.html
[3]: https://steveklabnik.com/writing/the-cxx-debate