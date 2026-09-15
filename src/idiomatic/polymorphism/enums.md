# Enums

Sometimes we need to handle multiple different types at runtime. Enums are a
powerful tool that allows us to safely and robustly describe situations like
this.

```rust,editable
use std::collections::HashMap;

fn main() {
    let number = parse_json("123");
    let array = parse_json("[456, true, false]");
    let object = parse_json(r#"{ "key": "value" }"#);
}

fn parse_json(doc: &str) -> JsonValue {
    todo!("Parse the JSON string...")
}

enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}
```

<details>

- The other category of polymorphism is **dynamic polymorphism**, where we can
  have different types of value at runtime, and we can't know statically which
  type we'll have at any given time.

- As an example, consider parsing a JSON string. There are several different
  types of JSON value, and which one we return depends on the contents of the
  input string.

- Our `parse_json` function has to return a single, concrete type, and that type
  needs to describe all of the possible types a JSON value can be. Enums are a
  natural way of describing this kind of situation in Rust.

</details>
