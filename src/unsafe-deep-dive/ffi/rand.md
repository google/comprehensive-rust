---
minutes: 15
---

# Wrapping `srand(3)` and `rand(3)`

<!-- Tests skipped because the mdbook test runner does not include libc  -->

```rust,editable,ignore
use libc::{rand, srand};

// unsafe extern "C" {
//     /// Seed the rng
//     fn srand(seed: std::ffi::c_uint);

//     fn rand() -> std::ffi::c_int;
// }

fn main() {
    unsafe { srand(12345) };

    let a = unsafe { rand() as i32 };
    println!("{a:?}");
}
```

<details>

This slide attempts to demonstrate that it is very easy for wrappers to trigger
undefined behavior if they are written incorrectly. We’ll see how easy it is to
trigger type safety problems.

Explain that `rand` and `srand` functions are provided by the C standard library
(libc).

Explain that the functions are exported by the libc crate, but we can also write
an FFI wrapper for them manually.

Show calling the functions from the exported.

Code compiles because libc is linked to Rust programs by default.

Explain that Rust will trust you if you use the wrong type(s).

Modify `fn rand() -> std::ffi::c_int;` to return `char`.

Avoiding type safety issues is a reason for using tools for generating wrappers,
rather than doing it by hand.

</details>
