# `aarch64-paging`

The [`aarch64-paging`][1] crate lets you create page tables according to the
AArch64 Virtual Memory System Architecture.

```rust,editable,compile_fail
use aarch64_paging::{
    idmap::IdMap,
    paging::{Attributes, MemoryRegion},
};

const ASID: usize = 1;
const ROOT_LEVEL: usize = 1;

// Create a new page table with identity mapping.
let mut idmap = IdMap::new(ASID, ROOT_LEVEL);
// Map a 2 MiB region of memory as read-only.
idmap.map_range(
    &MemoryRegion::new(0x80200000, 0x80400000),
    Attributes::NORMAL | Attributes::NON_GLOBAL | Attributes::READ_ONLY,
).unwrap();
// Set `TTBR0_EL1` to activate the page table.
idmap.activate();
```

<details>

- For now it only supports EL1, but support for other exception levels should be
  straightforward to add.
- This is used in Android for the [Protected VM Firmware][2].
- There's no easy way to run this example, as it needs to run on real hardware
  or under QEMU.

</details>

[1]: https://crates.io/crates/aarch64-paging
[2]: https://cs.android.com/android/platform/superproject/+/master:packages/modules/Virtualization/pvmfw/
