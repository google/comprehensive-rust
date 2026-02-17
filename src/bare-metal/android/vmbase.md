<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# vmbase

For VMs running under crosvm on aarch64, the [vmbase][1] library provides a
linker script and useful defaults for the build rules, along with an entry
point, UART console logging and more.

<!-- mdbook-xgettext: skip -->

```rust,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#![no_main]
#![no_std]

use vmbase::{main, println};

main!(main);

pub fn main(arg0: u64, arg1: u64, arg2: u64, arg3: u64) {
    println!("Hello world");
}
```

<details>

- The `main!` macro marks your main function, to be called from the `vmbase`
  entry point.
- The `vmbase` entry point handles console initialisation, and issues a
  PSCI_SYSTEM_OFF to shutdown the VM if your main function returns.

</details>

[1]: https://android.googlesource.com/platform/packages/modules/Virtualization/+/refs/heads/main/libs/libvmbase/
