# A Simple Rust Library

Exporting Rust functions and types to C is easy. Here's a simple Rust library:

_interoperability/rust/libanalyze/analyze.rs_

```rust,editable
{{#include rust/libanalyze/analyze.rs:analyze_numbers}}
```

_interoperability/rust/libanalyze/Android.bp_

```javascript
{{#include rust/libanalyze/Android.bp}}
```

<details>

`#[unsafe(no_mangle)]` disables Rust's usual name mangling, so the exported
symbol will just be the name of the function. You can also use
`#[unsafe(export_name = "some_name")]` to specify whatever name you want.

</details>


