# RTC driver

The QEMU aarch64 virt machine has a [PL031][1] real-time clock at 0x9010000. For this exercise, you
should write a driver for it.

1. Use it to print the current time to the serial console. You can use the [`chrono`][2] crate for
   date/time formatting.
2. Use the match register and raw interrupt status to busy-wait until a given time, e.g. 3 seconds
   in the future. (Call [`core::hint::spin_loop`][3] inside the loop.)

Download the [exercise template](../../comprehensive-rust-exercises.zip) and look in the `rtc`
directory for the following files.

`src/main.rs`:

<!-- File src/main.rs -->

```rust,compile_fail
{{#include rtc/src/main.rs:top}}

{{#include rtc/src/main.rs:imports}}

{{#include rtc/src/main.rs:main}}

    // TODO: Initialise RTC and print value.

    // TODO: Wait for 3 seconds.

{{#include rtc/src/main.rs:main_end}}
```

`src/exceptions.rs` (you shouldn't need to change this):

<!-- File src/exceptions.rs -->

```rust,compile_fail
{{#include rtc/src/exceptions.rs}}
```

`src/logger.rs` (you shouldn't need to change this):

<!-- File src/logger.rs -->

```rust,compile_fail
{{#include rtc/src/logger.rs}}
```

`src/pl011.rs` (you shouldn't need to change this):

<!-- File src/pl011.rs -->

```rust,compile_fail
{{#include rtc/src/pl011.rs}}
```

`Cargo.toml` (you shouldn't need to change this):

<!-- File Cargo.toml -->

```toml
{{#include rtc/Cargo.toml}}
```

`build.rs` (you shouldn't need to change this):

<!-- File build.rs -->

```rust,compile_fail
{{#include rtc/build.rs}}
```

`entry.S` (you shouldn't need to change this):

<!-- File entry.S -->

```armasm
{{#include rtc/entry.S}}
```

`exceptions.S` (you shouldn't need to change this):

<!-- File exceptions.S -->

```armasm
{{#include rtc/exceptions.S}}
```

`idmap.S` (you shouldn't need to change this):

<!-- File idmap.S -->

```armasm
{{#include rtc/idmap.S}}
```

`image.ld` (you shouldn't need to change this):

<!-- File image.ld -->

```ld
{{#include rtc/image.ld}}
```

`Makefile` (you shouldn't need to change this):

<!-- File Makefile -->

```makefile
{{#include rtc/Makefile}}
```

`.cargo/config.toml` (you shouldn't need to change this):

<!-- File .cargo/config.toml -->

```toml
{{#include rtc/.cargo/config.toml}}
```

Run the code in QEMU with `make qemu`.

[1]: https://developer.arm.com/documentation/ddi0224/c
[2]: https://crates.io/crates/chrono
[3]: https://doc.rust-lang.org/core/hint/fn.spin_loop.html
