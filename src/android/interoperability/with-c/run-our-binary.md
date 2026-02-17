<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Running Our Binary

Build, push, and run the binary on your device:

```shell
{{#include ../../build_all.sh:print_birthday_card}}
```

Finally, we can run auto-generated tests to ensure the bindings work:

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:libbirthday_bindgen_test}}
```

```shell
{{#include ../../build_all.sh:libbirthday_bindgen_test}}
```
