---
minutes: 15
---

## Typestate Pattern

The typestate pattern uses Rust’s type system to make **invalid states
unrepresentable**.

```rust
# use std::fmt::Write;
#[derive(Default)]
struct Serializer { output: String }
struct SerializeStruct { ser: Serializer }

impl Serializer {
    fn serialize_struct(mut self, name: &str) -> SerializeStruct {
        let _ = writeln!(&mut self.output, "{name} {{");
        SerializeStruct { ser: self }
    }
}

impl SerializeStruct {
    fn serialize_field(mut self, key: &str, value: &str) -> Self {
        let _ = writeln!(&mut self.ser.output, "  {key}={value};");
        self
    }

    fn finish_struct(mut self) -> Serializer {
        self.ser.output.push_str("}\n");
        self.ser
    }
}

let ser = Serializer::default()
    .serialize_struct("User")
    .serialize_field("id", "42")
    .serialize_field("name", "Alice")
    .finish_struct();
println!("{}", ser.output);
```

<details>

- This example is inspired by
  [Serde's `Serializer` trait](https://docs.rs/serde/latest/serde/ser/trait.Serializer.html).
  For a deeper explanation of how Serde models serialization as a state machine,
  see <https://serde.rs/impl-serializer.html>.

- The typestate pattern allows us to model state machines using Rust’s type
  system. In this case, the state machine is a simple serializer.

- The key idea is that each state in the process, starting a struct, writing
  fields, and finishing, is represented by a different type. Transitions between
  states happen by consuming one value and producing another.

- In the example above:

  - Once we begin serializing a struct, the `Serializer` is moved into the
    `SerializeStruct` state. At that point, we no longer have access to the
    original `Serializer`.

  - While in the `SerializeStruct` state, we can only call methods related to
    writing fields. We cannot use the same instance to serialize a tuple, list,
    or primitive. Those constructors simply do not exist here.

  - Only after calling `finish_struct` do we get the `Serializer` back. At that
    point, we can inspect the output or start a new serialization session.

  - If we forget to call `finish_struct` and drop the `SerializeStruct` instead,
    the original `Serializer` is lost. This ensures that incomplete or invalid
    output can never be observed.

- By contrast, if all methods were defined on `Serializer` itself, nothing would
  prevent users from mixing serialization modes or leaving a struct unfinished.

- This pattern avoids such misuse by making it **impossible to represent invalid
  transitions**.

- One downside of typestate modeling is potential code duplication between
  states. In the next section, we will see how to use **generics** to reduce
  duplication while preserving correctness.

</details>
