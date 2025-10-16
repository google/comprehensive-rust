## Serializer: implement Root

```rust
# use std::fmt::Write as _;
{{#include ../typestate-generics.rs:Serializer-def}}

{{#include ../typestate-generics.rs:Root-def}}
{{#include ../typestate-generics.rs:Struct-def}}

{{#include ../typestate-generics.rs:Root-impl}}
```

Referring back to our original diagram of valid transitions, we can visualize
the beginning of our implementation as follows:

```bob
                           serialize
                           struct
+---------------------+ --------------> +--------------------------------+
| Serializer [ Root ] |                 | Serializer [ Struct [ Root ] ] |
+---------------------+ <-------------- +--------------------------------+
                          finish struct
         |
         |
         |
finish   |
         V

     +--------+
     | String |
     +--------+
```

<details>

- At the "root" of our `Serializer`, the only construct allowed is a `Struct`.

- The `Serializer` can only be finalized into a `String` from this root level.

</details>
