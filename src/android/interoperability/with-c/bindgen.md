<!--
Copyright 2022 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Using Bindgen

The [bindgen](https://rust-lang.github.io/rust-bindgen/introduction.html) tool
can auto-generate bindings from a C header file.

Create a wrapper header file for the library (not strictly needed in this
example):

_interoperability/bindgen/libbirthday_wrapper.h_:

```c
{{#include bindgen/libbirthday_wrapper.h:include}}
```

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:libbirthday_bindgen}}
```

Finally, we can use the bindings in our Rust program:

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:print_birthday_card}}
```

_interoperability/bindgen/main.rs_:

```rust,compile_fail
# // Copyright 2022 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include bindgen/main.rs:main}}
```

<details>

- The Android build rules will automatically call `bindgen` for you behind the
  scenes.

- Notice that the Rust code in `main` is still hard to write. It is good
  practice to encapsulate the output of `bindgen` in a Rust library which
  exposes a safe interface to caller.

</details>
