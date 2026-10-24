<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Using the Bindings

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

- Notice that the Rust code in `main` is still hard to write. It is good
  practice to encapsulate the output of `bindgen` in a Rust library which
  exposes a safe interface to caller.

</details>
