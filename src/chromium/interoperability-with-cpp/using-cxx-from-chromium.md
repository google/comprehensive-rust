## Using cxx from Chromium

The `rust_static_library` target intrinsically knows about cxx interop. Just add

```gn
cxx_bindings = [ "my_rust_file.rs" ] # just list files containing #[cxx::bridge]
```

to your existing `rust_static_library` target. **You also need
`allow_unsafe = true`**.

C++ headers will be generated at a sensible location, so you can just

```cpp
#include "ui/base/my_rust_file.rs.h"
```

You will find some utility functions in `//base` to convert to/from Chromium
C++ types to cxx Rust types - for example [`SpanToRustSlice`][0].

[0]: https://source.chromium.org/chromium/chromium/src/+/main:base/containers/span_rust.h;l=21