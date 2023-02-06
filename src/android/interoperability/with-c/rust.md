# Calling Rust

Exporting Rust functions and types to C is easy:

_interoperability/rust/libanalyze/analyze.rs_

```rust,editable
{{#include rust/libanalyze/analyze.rs:analyze_numbers}}
```

_interoperability/rust/libanalyze/analyze.h_

```c
{{#include rust/libanalyze/analyze.h:analyze_numbers}}
```

_interoperability/rust/libanalyze/Android.bp_

```javascript
{{#include rust/libanalyze/Android.bp}}
```

We can now call this from a C binary:

_interoperability/rust/analyze/main.c_

```c
{{#include rust/analyze/main.c:main}}
```

_interoperability/rust/analyze/Android.bp_

```javascript
{{#include rust/analyze/Android.bp}}
```


Build, push, and run the binary on your device:

```shell
{{#include ../../build_all.sh:analyze_numbers}}
```

<details>

`#[no_mangle]` disables Rust's usual name mangling, so the exported symbol will just be the name of
the function. You can also use `#[export_name = "some_name"]` to specify whatever name you want.

</details>
