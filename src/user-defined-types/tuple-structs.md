---
minutes: 10
---

# Tuple Structs

If the field names are unimportant, you can use a tuple struct:

```rust,editable
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
```

This is often used for single-field wrappers (called newtypes):

```rust,editable,compile_fail
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
}
```

<details>

- Newtypes are a great way to encode additional information about the value in a
  primitive type, for example:
  - The number is measured in some units: `Newtons` in the example above.
  - The value passed some validation when it was created, so you no longer have
    to validate it again at every use: `PhoneNumber(String)` or
    `OddNumber(u32)`.
- The newtype pattern is covered extensively in the
  ["Idiomatic Rust" module](../idiomatic/leveraging-the-type-system/newtype-pattern.md).
- Demonstrate how to add a `f64` value to a `Newtons` type by accessing the
  single field in the newtype.
  - Rust generally doesn’t like inexplicit things, like automatic unwrapping or
    for instance using booleans as integers.
  - Operator overloading is discussed on Day 3 (generics).
- When a tuple struct has zero fields, the `()` can be omitted. The result is a
  zero-sized type (ZST), of which there is only one value (the name of the
  type).
  - This is common for types that implement some behavior but have no data
    (imagine a `NullReader` that implements some reader behavior by always
    returning EOF).
- The example is a subtle reference to the
  [Mars Climate Orbiter](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter)
  failure.

</details>
