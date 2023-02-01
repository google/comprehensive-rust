# A minimal `no_std` program

```rust,editable
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
```

<details>

* This will compile to an empty binary.
* `std` provides a panic handler; without it we must provide our own.
* Depending on the target, you may need to compile with `panic = "abort"` to avoid an error about
  `eh_personality`.

</details>
