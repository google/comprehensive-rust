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

</details>
