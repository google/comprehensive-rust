# Inspecting Enums

We can easily inspect the contents of an enum using **pattern matching**:

```rust,editable,compile_fail
fn do_json_stuff(json: &str) {
    match parse_json(json) {
        JsonValue::Object(obj) => println!("We got an object: {obj:?}"),
        JsonValue::Array(array) => println!("We got an array: {array:?}"),
        JsonValue::String(string) => println!("We got a string: {string:?}"),
        JsonValue::Number(num) => println!("We got a number: {num}"),
        JsonValue::Bool(b) => println!("We got a bool: {b}"),
        JsonValue::Null => println!("We got a null"),
    }
}
```

<details>

- Continuing with our JSON parsing example, we can easily determine which kind
  of value we got by pattern matching on the resulting enum.

- This makes enums a good fit for scenarios where we want to handle different
  types at runtime, but want to retain type information and the ability to
  directly inspect the concrete value.

- This is a big advantage enums have over `dyn`: With `dyn` we can't easily
  downcast to the specific concrete type, and are generally restricted to going
  through the trait interface. Later we'll see that we can support downcasting
  with `dyn`, but doing so requires extra setup that isn't necessary with enums.

</details>
