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
    
Key Points:
 * Tuple structs are a different type of struct. Note the lack of curly braces and the way fields are accessed.
 * A tuple struct can be used to create a new type by wrapping around a single field. The rest of the code can then talk about things in terms of these new types and avoid mistakes.
* This slide shows an example of creating new types from `f64`s. You can demonstrate how to create and modify a tuple struct. You can also add a `f64` value to the tuple struct.
* This can be a good time to talk about unwrapping.  Rust generally doesn’t like inexplicit things, for instance booleans aren’t used as integers. Similarly using the data in a struct requires accessing fields appropriately so there isn't automatic unwrapping. 
* If appropriate, you might bring up that using existing operations on these new types, such as `+` is possible, and we’ll learn about generics on Day 3. 
    
</details>
