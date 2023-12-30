## Using cxx in Chromium

In Chromium, we define an independent `#[cxx::bridge] mod` for each leaf-node
where we want to use Rust. You'd typically have one for each
`rust_static_library`. Just add

```gn
cxx_bindings = [ "my_rust_file.rs" ]
   # list of files containing #[cxx::bridge], not all source files
allow_unsafe = true
```

to your existing `rust_static_library` target alongside `crate_root` and
`sources`.

C++ headers will be generated at a sensible location, so you can just

```cpp
#include "ui/base/my_rust_file.rs.h"
```

You will find some utility functions in `//base` to convert to/from Chromium C++
types to CXX Rust types --- for example [`SpanToRustSlice`][0].

<details>

Students may ask --- why do we still need `allow_unsafe = true`?

The broad answer is that no C/C++ code is "safe" by the normal Rust standards.
Calling back and forth to C/C++ from Rust may do arbitrary things to memory, and
compromise the safety of Rust's own data layouts. Presence of _too many_
`unsafe` keywords in C/C++ interop can harm the signal-to-noise ratio of such a
keyword, and is [controversial][1], but strictly, bringing any foreign code into
a Rust binary can cause unexpected behavior from Rust's perspective.

The narrow answer lies in the diagram at the top of [this page][2] --- behind
the scenes, CXX generates Rust `unsafe` and `extern "C"` functions just like we
did manually in the previous section.

</details>

[0]: https://source.chromium.org/chromium/chromium/src/+/main:base/containers/span_rust.h;l=21
[1]: https://steveklabnik.com/writing/the-cxx-debate
[2]: ../interoperability-with-cpp.md
