## Typestate Pattern: Example

The typestate pattern encodes part of a value’s runtime state into its type.
This allows us to prevent invalid or inapplicable operations at compile time.

```rust,editable
use std::fmt::Write as _;

#[derive(Default)]
struct Serializer {
    output: String,
}

struct SerializeStruct {
    serializer: Serializer,
}

impl Serializer {
    fn serialize_struct(mut self, name: &str) -> SerializeStruct {
        writeln!(&mut self.output, "{name} {{").unwrap();
        SerializeStruct { serializer: self }
    }

    fn finish(self) -> String {
        self.output
    }
}

impl SerializeStruct {
    fn serialize_field(mut self, key: &str, value: &str) -> Self {
        writeln!(&mut self.serializer.output, "  {key}={value};").unwrap();
        self
    }

    fn finish_struct(mut self) -> Serializer {
        self.serializer.output.push_str("}\n");
        self.serializer
    }
}

fn main() {
    let serializer = Serializer::default()
        .serialize_struct("User")
        .serialize_field("id", "42")
        .serialize_field("name", "Alice")
        .finish_struct();

    println!("{}", serializer.finish());
}
```

`Serializer` usage flowchart:

```bob
+------------+  serialize struct   +-----------------+
| Serializer | ------------------> | SerializeStruct | <------+
+------------+                     +-----------------+        |
                                                              |
   |   ^                             |     |                  |
   |   |     finish struct           |     | serialize field  |
   |   +-----------------------------+     +------------------+
   |
   +---> finish
```

<details>

- This example is inspired by Serde’s
  [`Serializer` trait](https://docs.rs/serde/latest/serde/ser/trait.Serializer.html).
  Serde uses typestates internally to ensure serialization follows a valid
  structure. For more, see: <https://serde.rs/impl-serializer.html>

- The key idea behind typestate is that state transitions happen by consuming a
  value and producing a new one. At each step, only operations valid for that
  state are available.

- In this example:

  - We begin with a `Serializer`, which only allows us to start serializing a
    struct.

  - Once we call `.serialize_struct(...)`, ownership moves into a
    `SerializeStruct` value. From that point on, we can only call methods
    related to serializing struct fields.

  - The original `Serializer` is no longer accessible — preventing us from
    mixing modes (such as starting another _struct_ mid-struct) or calling
    `finish()` too early.

  - Only after calling `.finish_struct()` do we receive the `Serializer` back.
    At that point, the output can be finalized or reused.

- If we forget to call `finish_struct()` and drop the `SerializeStruct` early,
  the `Serializer` is also dropped. This ensures incomplete output cannot leak
  into the system.

- By contrast, if we had implemented everything on `Serializer` directly — as
  seen on the previous slide, nothing would stop someone from skipping important
  steps or mixing serialization flows.

</details>
