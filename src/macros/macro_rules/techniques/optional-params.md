---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Optional Parameters

The `?` repetition operator can be used for optional parameters, but in older
Rust editions (2018 and 2015) it must be emulated; this can be done by defining
multiple matcher arms that cascade. Their right-hand sides may reuse each other
via recursion, which is a common pattern in declarative macros regardless of
edition.

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! log {
    // Single parameter:
    ($msg:literal) => {
        log!($msg, )//println!("{}:{}: {}", file!(), line!(), $msg)
    };
    // With optional formatting parameters:
    ($msg:literal, $($params:expr),*) => {
        println!(concat!("{}:{}: ", $msg), file!(), line!(), $($params),*)
    };
}

fn main() {
    log!("my message");
    log!("my message with param {}!", 123);
}
```

<details>

- Highlight how `concat!` is used at compile time to combine the file/line
  prefix with the user's log message literal.
- Recall that the order of arms is significant, just like in a `match`. The
  compiler matches sequentially, and if a broader arm is placed above a narrower
  one, the narrower arm may never be reached.

</details>
