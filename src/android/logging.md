<!--
Copyright 2022 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Logging

You should use the `log` crate to automatically log to `logcat` (on-device) or
`stdout` (on-host):

_hello_rust_logs/Android.bp_:

```javascript
{{#include logging/Android.bp}}
```

_hello_rust_logs/src/main.rs_:

```rust,ignore
# // Copyright 2022 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include logging/src/main.rs:main}}
```

Build, push, and run the binary on your device:

```shell
{{#include build_all.sh:hello_rust_logs}}
```

The logs show up in `adb logcat`:

```shell
adb logcat -s rust
```

```text
09-08 08:38:32.454  2420  2420 D rust: hello_rust_logs: Starting program.
09-08 08:38:32.454  2420  2420 I rust: hello_rust_logs: Things are going fine.
09-08 08:38:32.454  2420  2420 E rust: hello_rust_logs: Something went wrong!
```

<details>

- The logger implementation in `liblogger` is only needed in the final binary,
  if you're logging from a library you only need the `log` facade crate.

</details>
