## Beyond Simple Typestate

How do we manage increasingly complex configuration flows with many possible
states and transitions, while still preventing incompatible operations?

```rust
struct Serializer {/* [...] */}
struct SerializeStruct {/* [...] */}
struct SerializeStructProperty {/* [...] */}
struct SerializeList {/* [...] */}

impl Serializer {
    // TODO, implement:
    //
    // fn serialize_struct(self, name: &str) -> SerializeStruct
    // fn finish(self) -> String
}

impl SerializeStruct {
    // TODO, implement:
    //
    // fn serialize_property(mut self, name: &str) -> SerializeStructProperty

    // TODO,
    // How should we finish this struct? This depends on where it appears:
    // - At the root level: return `Serializer`
    // - As a property inside another struct: return `SerializeStruct`
    // - As a value inside a list: return `SerializeList`
    //
    // fn finish(self) -> ???
}

impl SerializeStructProperty {
    // TODO, implement:
    //
    // fn serialize_string(self, value: &str) -> SerializeStruct
    // fn serialize_struct(self, name: &str) -> SerializeStruct
    // fn serialize_list(self) -> SerializeList
    // fn finish(self) -> SerializeStruct
}

impl SerializeList {
    // TODO, implement:
    //
    // fn serialize_string(mut self, value: &str) -> Self
    // fn serialize_struct(mut self, value: &str) -> SerializeStruct
    // fn serialize_list(mut self) -> SerializeList

    // TODO:
    // Like `SerializeStruct::finish`, the return type depends on nesting.
    //
    // fn finish(mut self) -> ???
}
```

<details>

- Building on our previous serializer, we now want to support **nested
  structures** and **lists**.

- However, this introduces both **duplication** and **structural complexity**.

  `SerializeStructProperty` and `SerializeList` now share similar logic (e.g.
  adding strings, nested structs, or nested lists).

- Even more critically, we now hit a **type system limitation**: we cannot
  cleanly express what `finish()` should return without duplicating variants for
  every nesting context (e.g. root, struct, list).

- To better understand this limitation, let’s map the valid transitions:

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

- From this diagram, we can observe:
  - The transitions are recursive
  - The return types depend on _where_ a substructure or list appears
  - Each context requires a return path to its parent

- With only concrete types, this becomes unmanageable. Our current approach
  leads to an explosion of types and manual wiring.

- In the next chapter, we’ll see how **generics** let us model recursive flows
  with less boilerplate, while still enforcing valid operations at compile time.

</details>
