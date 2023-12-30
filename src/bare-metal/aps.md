---
session: Afternoon
---

# Application processors

So far we've talked about microcontrollers, such as the Arm Cortex-M series. Now
let's try writing something for Cortex-A. For simplicity we'll just work with
QEMU's aarch64
['virt'](https://qemu-project.gitlab.io/qemu/system/arm/virt.html) board.

<details>

- Broadly speaking, microcontrollers don't have an MMU or multiple levels of
  privilege (exception levels on Arm CPUs, rings on x86), while application
  processors do.
- QEMU supports emulating various different machines or board models for each
  architecture. The 'virt' board doesn't correspond to any particular real
  hardware, but is designed purely for virtual machines.

</details>
