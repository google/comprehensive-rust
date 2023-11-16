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
