<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Memory Lifecycle

Memory moves through different phases as objects (values) are created and
destroyed.

| Memory State | Readable from Safe Rust? |
| ------------ | ------------------------ |
| Available    | No                       |
| Allocated    | No                       |
| Initialized  | Yes                      |

<details>

This section discusses what happens as memory from the operating system becomes
a valid variable in the program.

When memory is available, the operating system has provided our program with it.

When memory is allocated, it is reserved for values to be written to it. We call
this uninitialized memory.

When memory is initialized, it is safe to read from.

</details>
