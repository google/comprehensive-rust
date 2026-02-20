---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

## Serializer: implement Struct

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
# use std::fmt::Write as _;
{{#include ../typestate-generics.rs:Serializer-def}}

{{#include ../typestate-generics.rs:Struct-def}}
{{#include ../typestate-generics.rs:Property-def}}

{{#include ../typestate-generics.rs:Struct-impl}}
```

The diagram can now be expanded as follows:

```bob
                                                     +------+
                                             finish  |      |
                          serialize          struct  V      |
                          struct
+--------------------+ --------------> +-------------------------+
| "Serializer<Root>" |                 | "Serializer<Struct<S>>" |
+--------------------+ <-------------- +-------------------------+
                         finish struct
         |                                     serialize   |
         |                                     property    V
         |
finish   |                           +-----------------------------------+
         V                           | "Serializer<Property<Struct<S>>>" |
                                     +-----------------------------------+
     +--------+
     | String |
     +--------+
```

<details>

- A `Struct` can only contain a `Property`;

- Finishing a `Struct` returns control back to its parent, which in our previous
  slide was assumed the `Root`, but in reality however it can be also something
  else such as `Struct` in case of nested "structs".

</details>
