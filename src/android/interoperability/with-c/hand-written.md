<!--
Copyright 2022 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Handwritten FFI

We can declare external functions by hand:

```rust
# // Copyright 2022 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
unsafe extern "C" {
    safe fn abs(x: i32) -> i32;
}

fn main() {
    let x = -42;
    let abs_x = abs(x);
    println!("{x}, {abs_x}");
}
```

We already saw this in the
[Safe FFI Wrapper exercise](../../exercises/day-3/safe-ffi-wrapper.md).

> This assumes full knowledge of the target platform. Not recommended for
> production.

<details>

- This is just a motivating example. For a real library, you want to use
  `bindgen` as shown on the next slide.

</details>
