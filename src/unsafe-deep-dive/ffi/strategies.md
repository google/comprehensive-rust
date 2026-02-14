---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Strategies of interop

<!-- Note: Retain the quotation marks around C++ in the svgbob diagrams to preserve formatting in the output -->

Sharing data structures and symbols directly is very difficult:

```bob
╭────────────╮                                          ╭────────────╮
│            │                                          │            │
│            │ <--------------------------------------> │            │
│            │                                          │            │
╰────────────╯                                          ╰────────────╯
     Rust                                                    "C++"
```

FFI through the C ABI is much more feasible:

```bob
╭────────────╮          ╭───╮           ╭───╮          ╭────────────╮
│            │          │   │           │   │          │            │
│            │ <----->  │   │ <~~~~~~~> │   │ <------> │            │ 
│            │          │   │           │   │          │            │
╰────────────╯          ╰───╯           ╰───╯          ╰────────────╯
    Rust                  C               C                 "C++"
```

Other strategies:

- Distributed system (RPC)
- Custom ABI (i.e. WebAssembly Interface Types)

<details>

_High-fidelity interop_

The ideal scenario is currently experimental.

Two projects exploring this are [crubit](https://github.com/google/crubit) and
[Zngur](https://hkalbasi.github.io/zngur/). The first provides glue code on each
side for enabling compatible types to work seamlessly across domains. The second
relies on dynamic dispatch and imports C++ objects into Rust as trait objects.

_Low-fidelity interop_ work through a C API

The typical strategy for interop is to use the C language as the interface. C is
a lossy codec. This strategy typically results in complicated code on both
sides.

_Other strategies_ are less viable in a zero cost environment.

_Distributed systems_ impose runtime costs.

They incur significant overhead as calling a method in a foreign library incurs
a round trip of serialization/transport/deserialization. Generally speaking, a
transparent RPC is not a good idea. There’s network in the middle.

_Custom ABI_, such as wasm require a runtime or significant implementation cost.

</details>
