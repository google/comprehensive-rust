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

* This tuple struct is called a Point, which contains a pair of integers. 
    * We then have a shorthand for creating it. Note, no curly braces. 
    * The two fields are called with a dot then 0 and 1.
* Use case: Create a new type, by wrapping around a single field. 
    * Here is an example of new type PoundsOfForce and Newtons, which are now different from the f64s type or eachother. 
    * We can talk about things in terms of these new types. For instance, in what functions should return/accept.
* Inspired example 
    * Say you’re a space agency and you want to send a probe up to Mars. As you have your rocket you might want to have a function to compute/set thruster force and you are using f64s but want to consider them different.
    * [Run example gets an error. Add x= Newtons(2.0); set..(x);]
    * Inspired example is from a crash that was caused because of a rounding error that accumulated between Pound of Force and Newtons. 
* Rust doesn’t do inexplicit things, for instance no using booleans used as integers
    * Similarly, using the data in a struct requires accessing field appropriately 
    * [Add float to force force.0+10  in the set function.] 
    * In some cases, like with the + operation, we can overload the operator, we could also use de-refef. We’ll talk about it later

</details>