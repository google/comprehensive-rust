# Function Overloading

Overloading is not supported:

* Each function has a single implementation:
  * Always takes a fixed number of parameters.
  * Always takes a single set of parameter types.
* Default values are not supported:
  * All call sites have the same number of arguments.
  * Macros are sometimes used as an alternative.

However, function parameters can be generic:

```rust,editable
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}
```

<details>

* When using generics, the standard library's `Into<T>` can provide a kind of limited
  polymorphism on argument types. We will see more details in a later section.

</details>

