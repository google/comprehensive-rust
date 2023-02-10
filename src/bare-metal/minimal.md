# A minimal `no_std` program

```rust,editable,compile_fail
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
```

<details>

* This will compile to an empty binary.
* `std` provides a panic handler; without it we must provide our own.
* It can also be provided by another crate, such as `panic-halt`.
* Depending on the target, you may need to compile with `panic = "abort"` to avoid an error about
  `eh_personality`.
* Note that there is no `main` or any other entry point; it's up to you to define your own entry
  point. This will typically involve a linker script and some assembly code to set things up ready
  for Rust code to run.

</details>
