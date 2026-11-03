<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

Here is the implementation of the generalized `define_ops!` macro using
`std::ops` traits and nested macro repetition:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Scalar<T: Copy>(pub T);

macro_rules! define_ops {
    // Base case: no types left to handle
    (
        : $( $trait_name:ident :: $fn_name:ident => $op:tt ),* $(,)?
    ) => { };
    // Handle the first type, expanding to all trait impls for that type
    (
        $t:ty $( , $ts:ty )* : $( $trait_name:ident :: $fn_name:ident => $op:tt ),* $(,)?
    ) => {
        $(
            impl std::ops::$trait_name for Scalar<$t> {
                type Output = Scalar<$t>;

                fn $fn_name(self, rhs: Self) -> Self::Output {
                    Scalar(self.0 $op rhs.0)
                }
            }
        )*

        // Recursively handle the rest of the types
        define_ops! { $($ts),* : $( $trait_name :: $fn_name => $op ),* }
    };
}

define_ops! {
    i32, f32, f64:
        Add::add => +,
        Sub::sub => -,
        Mul::mul => *
}

fn main() {
    let a32 = Scalar(10_i32);
    let b32 = Scalar(2_i32);
    assert_eq!(a32 + b32, Scalar(12));

    let a64 = Scalar(10.5_f64);
    let b64 = Scalar(2.5_f64);
    assert_eq!(a64 - b64, Scalar(8.0));
    assert_eq!(a64 * b64, Scalar(26.25));

    let a32_f = Scalar(10.5_f32);
    let b32_f = Scalar(2.5_f32);
    assert_eq!(a32_f + b32_f, Scalar(13.0_f32));
}
```

<details>

The macro transcriber uses nested repetition `$($( ... )*)*` to expand an
`impl std::ops::$trait_name for Scalar<$t>` block for every combination of type
`$t` and operator `$op`.

</details>
