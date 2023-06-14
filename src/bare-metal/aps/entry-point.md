# Getting Ready to Rust

Before we can start running Rust code, we need to do some initialisation.

```armasm
{{#include examples/entry.S:entry}}
```

<details>

* This is the same as it would be for C: initialising the processor state, zeroing the BSS, and
  setting up the stack pointer.
  * The BSS (block starting symbol, for historical reasons) is the part of the object file which
    containing statically allocated variables which are initialised to zero. They are omitted from
    the image, to avoid wasting space on zeroes. The compiler assumes that the loader will take care
    of zeroing them.
* The BSS may already be zeroed, depending on how memory is initialised and the image is loaded, but
  we zero it to be sure.
* We need to enable the MMU and cache before reading or writing any memory. If we don't:
  * Unaligned accesses will fault. We build the Rust code for the `aarch64-unknown-none` target
    which sets `+strict-align` to prevent the compiler generating unaligned accesses, so it should
    be fine in this case, but this is not necessarily the case in general.
  * If it were running in a VM, this can lead to cache coherency issues. The problem is that the VM
    is accessing memory directly with the cache disabled, while the host has cachable aliases to the
    same memory. Even if the host doesn't explicitly access the memory, speculative accesses can
    lead to cache fills, and then changes from one or the other will get lost when the cache is
    cleaned or the VM enables the cache. (Cache is keyed by physical address, not VA or IPA.)
* For simplicity, we just use a hardcoded pagetable (see `idmap.S`) which identity maps the first 1
  GiB of address space for devices, the next 1 GiB for DRAM, and another 1 GiB higher up for more
  devices. This matches the memory layout that QEMU uses.
* We also set up the exception vector (`vbar_el1`), which we'll see more about later.
* All examples this afternoon assume we will be running at exception level 1 (EL1). If you need to
  run at a different exception level you'll need to modify `entry.S` accordingly.

</details>
