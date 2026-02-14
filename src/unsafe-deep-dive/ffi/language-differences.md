---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Language differences

```bob
╭────────────╮          ╭───╮           ╭───╮          ╭────────────╮
│            │          │   │           │   │          │            │
│            │ <----->  │   │ <~~~~~~~> │   │ <------> │            │ 
│            │          │   │           │   │          │            │
╰────────────╯          ╰───╯           ╰───╯          ╰────────────╯
    Rust                  C               C                 "C++"
```

<details>

Using C as the lowest common denominator means that lots of the richness
available to Rust and C++ is lost.

Each translation has the potential for semantic loss, runtime overhead, and
subtle bugs.

</details>
