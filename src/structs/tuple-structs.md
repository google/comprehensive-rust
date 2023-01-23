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
struct PoundOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundOfForce {
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

Newtypes are a great way to encode additional information about the value in a primitive type, for example:
  * The number is measured in some units: `Newtons` in the example above.
  * The value passed some validation when it was created, so you no longer have to validate it again at every use: 'PhoneNumber(String)` or `OddNumber(u32)`.
    
</details>
