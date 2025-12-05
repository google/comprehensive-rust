---
minutes: 10
---

## Typestate Pattern with Generics

By combining typestate modeling with generics, we can express a wider range of
valid states and transitions without duplicating logic. This approach is
especially useful when the number of states grows or when multiple states share
behavior but differ in structure.

```rust
{{#include typestate-generics.rs:Serializer-def}}

{{#include typestate-generics.rs:Root-def}}
{{#include typestate-generics.rs:Struct-def}}
{{#include typestate-generics.rs:Property-def}}
{{#include typestate-generics.rs:List-def}}
```

We now have all the tools needed to implement the methods for the `Serializer`
and its state type definitions. This ensures that our API only permits valid
transitions, as illustrated in the following diagram:

Diagram of valid transitions:

```bob
    +-----------+   +---------+------------+-----+
    |           |   |         |            |     |
    V           |   V         |            V     |
                +                                |
serializer --> structure --> property --> list +-+

    |           |   ^           |          ^
    V           |   |           |          |
                |   +-----------+          |
  String        |                          |
                +--------------------------+
```

<details>

- By leveraging generics to track the parent context, we can construct
  arbitrarily nested serializers that enforce valid transitions between struct,
  list, and property states.

- This enables us to build a recursive structure while maintaining strict
  control over which methods are accessible in each state.

- Methods common to all states can be defined for any `S` in `Serializer<S>`.

- Marker types (e.g., `List<S>`) introduce no memory or runtime overhead, as
  they contain no data other than a possible Zero-Sized Type. Their only role is
  to enforce correct API usage through the type system.

</details>
