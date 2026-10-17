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

<details>

- The Android build rules will automatically call `bindgen` for you behind the
  scenes.

</details>
