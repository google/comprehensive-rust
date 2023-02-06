# The `Default` Trait

`Default` trait provides a default implementation of a trait.

```rust,editable
#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("John Smith".into())
    }
}

fn main() {
    let default_struct: Derived = Default::default();
    println!("{default_struct:#?}");

    let almost_default_struct = Derived {
        y: "Y is set!".into(),
        ..Default::default()
    };
    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}

```

<details>

  * It can be implemented directly or it can be derived via `#[derive(Default)]`.
  * Derived implementation will produce an instance where all fields are set to their default values.
    * This means all types in the struct must implement `Default` too.
  * Standard Rust types often implement `Default` with reasonable values (e.g. `0`, `""`, etc).
  * The partial struct copy works nicely with default.
  * Rust standard library is aware that types can implement `Default` and provides convenience methods that use it.

</details>
