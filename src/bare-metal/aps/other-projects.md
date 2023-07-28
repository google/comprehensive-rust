# Other projects

 * [oreboot](https://github.com/oreboot/oreboot)
   * "coreboot without the C"
   * Supports x86, aarch64 and RISC-V.
   * Relies on LinuxBoot rather than having many drivers itself.
 * [Rust RaspberryPi OS tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
   * Initialisation, UART driver, simple bootloader, JTAG, exception levels, exception handling,
     page tables
   * Some dodginess around cache maintenance and initialisation in Rust, not necessarily a good
     example to copy for production code.
 * [`cargo-call-stack`](https://crates.io/crates/cargo-call-stack)
   * Static analysis to determine maximum stack usage.

<details>

* The RaspberryPi OS tutorial runs Rust code before the MMU and caches are enabled. This will read
  and write memory (e.g. the stack). However:
  * Without the MMU and cache, unaligned accesses will fault. It builds with `aarch64-unknown-none`
    which sets `+strict-align` to prevent the compiler generating unaligned accesses so it should be
    alright, but this is not necessarily the case in general.
  * If it were running in a VM, this can lead to cache coherency issues. The problem is that the VM
    is accessing memory directly with the cache disabled, while the host has cachable aliases to the
    same memory. Even if the host doesn't explicitly access the memory, speculative accesses can
    lead to cache fills, and then changes from one or the other will get lost. Again this is alright
    in this particular case (running directly on the hardware with no hypervisor), but isn't a good
    pattern in general.

</details>
