---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

## Serializer: implement Property

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
# use std::fmt::Write as _;
{{#include ../typestate-generics.rs:Serializer-def}}

{{#include ../typestate-generics.rs:Struct-def}}
{{#include ../typestate-generics.rs:Property-def}}
{{#include ../typestate-generics.rs:List-def}}

{{#include ../typestate-generics.rs:Property-impl}}
```

With the addition of the Property state methods, our diagram is now nearly
complete:

```bob
                                                         +------+
                                                 finish  |      |
                           serialize             struct  V      |
                           struct
+---------------------+ --------------> +-----------------------------+
| Serializer [ Root ] |                 | Serializer [ Struct [ S ] ] |
+---------------------+ <-------------- +-----------------------------+ <-----------+
                          finish struct                                             |
         |                                        serialize   |                     |
         |                                        property    V          serialize  |
         |                                                               string or  |
finish   |                            +-------------------------------+  struct     |
         V                            | Serializer [ Property [ S ] ] | ------------+
                                      +-------------------------------+
     +--------+
     | String |                                   serialize   |
     +--------+                                   list        V

                                          +---------------------------+
                                          | Serializer [ List [ S ] ] |
                                          +---------------------------+
```

<details>

- A property can be defined as a `String`, `Struct<S>`, or `List<S>`, enabling
  the representation of nested structures.

- This concludes the step-by-step implementation. The full implementation,
  including support for `List<S>`, is shown in the next slide.

</details>
