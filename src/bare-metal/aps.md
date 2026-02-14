---
session: Afternoon
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Application processors

So far we've talked about microcontrollers, such as the Arm Cortex-M series.
These are typically small systems with very limited resources.

Larger systems with more resources are typically called application processors,
built around processors such as the ARM Cortex-A or Intel Atom.

For simplicity we'll just work with QEMU's aarch64
['virt'](https://qemu-project.gitlab.io/qemu/system/arm/virt.html) board.

<details>

- Broadly speaking, microcontrollers don't have an MMU or multiple levels of
  privilege (exception levels on Arm CPUs, rings on x86).
- Application processors have more resources, and often run an operating system,
  instead of directly executing the target application on startup.
- QEMU supports emulating various different machines or board models for each
  architecture. The 'virt' board doesn't correspond to any particular real
  hardware, but is designed purely for virtual machines.
- We will still address this board as bare-metal, as if we were writing an
  operating system.

</details>
