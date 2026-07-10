<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

Here is the implementation of the `define_op!` macro that dynamically defines
mathematical functions:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
macro_rules! define_op {
    ( $name:ident => $op:tt ) => {
        fn $name(x: i32, y: i32) -> i32 {
            x $op y
        }
    };
}

define_op! { add => + }
define_op! { sub => - }
define_op! { mul => * }

fn main() {
    println!("10 + 2 = {}", add(10, 2));
    println!("10 - 2 = {}", sub(10, 2));
    println!("10 * 2 = {}", mul(10, 2));

    assert_eq!(add(10, 2), 12);
    assert_eq!(sub(10, 2), 8);
    assert_eq!(mul(10, 2), 20);
}
```
