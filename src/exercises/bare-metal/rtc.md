# RTC driver

The QEMU aarch64 virt machine has a [PL031][1] real-time clock at 0x9010000. For this exercise, you
should write a driver for it and use it to print the current time to the serial console. You can use
the [`chrono`][2] crate for date/time formatting.

`src/main.rs`:
```rust,compile_fail
{{#include rtc/src/main.rs:top}}

{{#include rtc/src/main.rs:imports}}

{{#include rtc/src/main.rs:main}}

    // TODO: Initialise RTC and print value.

{{#include rtc/src/main.rs:main_end}}
```

`src/exceptions.rs` (you shouldn't need to change this):
```rust,compile_fail
{{#include rtc/src/exceptions.rs}}
```

`src/logger.rs` (you shouldn't need to change this):
```rust,compile_fail
{{#include rtc/src/logger.rs}}
```

`src/pl011.rs` (you shouldn't need to change this):
```rust,compile_fail
{{#include rtc/src/pl011.rs}}
```

`Cargo.toml` (you shouldn't need to change this):
```toml
{{#include rtc/Cargo.toml}}
```

`build.rs` (you shouldn't need to change this):
```rust,compile_fail
{{#include rtc/build.rs}}
```

`entry.S` (you shouldn't need to change this):
```armasm
{{#include rtc/entry.S}}
```

`exceptions.S` (you shouldn't need to change this):
```armasm
{{#include rtc/exceptions.S}}
```

`idmap.S` (you shouldn't need to change this):
```armasm
{{#include rtc/idmap.S}}
```

`image.ld` (you shouldn't need to change this):
```ld
{{#include rtc/image.ld}}
```

`Makefile` (you shouldn't need to change this):
```makefile
{{#include rtc/Makefile}}
```

`.cargo/config.toml` (you shouldn't need to change this):
```toml
{{#include rtc/.cargo/config.toml}}
```

Run the code in QEMU with `make qemu`.

[1]: https://developer.arm.com/documentation/ddi0224/c
[2]: https://crates.io/crates/chrono
