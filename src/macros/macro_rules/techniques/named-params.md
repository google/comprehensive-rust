---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Named Positional Parameters

Using literal identifiers as key-value descriptors allows you to build
declarative macro APIs with named parameters. For macros with many arguments,
this may significantly increase readability at the callsite:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
trait Reset {
    fn reset(&mut self);
}

macro_rules! impl_reset {
    (type: $type:ty, function: $function:ident) => {
        impl Reset for $type {
            fn reset(&mut self) {
                self.$function()
            }
        }
    };
}

trait I64Ext {
    fn zero(&mut self);
}

impl I64Ext for i64 {
    fn zero(&mut self) {
        *self = 0;
    }
}

// Callers pass named parameters:
impl_reset! { type: String, function: clear }
impl_reset! { type: i64, function: zero }

fn main() {
    let mut n = 50i64;
    let mut s = String::from("Hello");
    dbg!((s.len(), n));
    n.reset(); // Calls zero()
    s.reset(); // Calls clear()
    dbg!((s.len(), n));
}
```

- **Structural Identifiers:** `type:` and `function:` are matched as literal
  tokens.
- **Improved Intent:** The callsite clearly documents which identifier is the
  type and which is the method, instead of relying on positional parameter index
  ordering.
