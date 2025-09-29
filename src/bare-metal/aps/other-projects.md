# Other projects

- [oreboot](https://github.com/oreboot/oreboot)
  - "coreboot without the C".
  - Supports x86, aarch64 and RISC-V.
  - Relies on LinuxBoot rather than having many drivers itself.
- [Rust RaspberryPi OS tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
  - Initialization, UART driver, simple bootloader, JTAG, exception levels,
    exception handling, page tables.
  - Some caveats around cache maintenance and initialization in Rust, not
    necessarily a good example to copy for production code.
- [`cargo-call-stack`](https://crates.io/crates/cargo-call-stack)
  - Static analysis to determine maximum stack usage.

<details>

- The RaspberryPi OS tutorial runs Rust code before the MMU and caches are
  enabled. This will read and write memory (e.g. the stack). However, this has
  the problems mentioned at the beginning of this session regarding unaligned
  access and cache coherency.

</details>
