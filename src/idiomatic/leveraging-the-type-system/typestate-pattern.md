---
minutes: 30
---

## Typestate Pattern: Problem

How can we ensure that only valid operations are allowed on a value based on its
current state?

```rust,editable
use std::fmt::Write as _;

#[derive(Default)]
struct Serializer {
    output: String,
}

impl Serializer {
    fn serialize_struct_start(&mut self, name: &str) {
        let _ = writeln!(&mut self.output, "{name} {{");
    }

    fn serialize_struct_field(&mut self, key: &str, value: &str) {
        let _ = writeln!(&mut self.output, "  {key}={value};");
    }

    fn serialize_struct_end(&mut self) {
        self.output.push_str("}\n");
    }

    fn finish(self) -> String {
        self.output
    }
}

fn main() {
    let mut serializer = Serializer::default();
    serializer.serialize_struct_start("User");
    serializer.serialize_struct_field("id", "42");
    serializer.serialize_struct_field("name", "Alice");

    // serializer.serialize_struct_end(); // ← Oops! Forgotten

    println!("{}", serializer.finish());
}
```

<details>

- This `Serializer` is meant to write a structured value. The expected usage
  follows this sequence:

```bob
serialize struct start
-+---------------------
 |
 +--> serialize struct field
      -+---------------------
       |
       +--> serialize struct field
            -+---------------------
             |
             +--> serialize struct end
```

- However, in this example we forgot to call `serialize_struct_end()` before
  `finish()`. As a result, the serialized output is incomplete or syntactically
  incorrect.

- One approach to fix this would be to track internal state manually, and return
  a `Result` from methods like `serialize_struct_field()` or `finish()` if the
  current state is invalid.

- But this has downsides:

  - It is easy to get wrong as an implementer. Rust’s type system cannot help
    enforce the correctness of our state transitions.

  - It also adds unnecessary burden on the user, who must handle `Result` values
    for operations that are misused in source code rather than at runtime.

- A better solution is to model the valid state transitions directly in the type
  system.

  In the next slide, we will apply the **typestate pattern** to enforce correct
  usage at compile time and make invalid states unrepresentable.

</details>
