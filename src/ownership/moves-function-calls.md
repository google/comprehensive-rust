# Moves in Function Calls

When you pass a value to a function, the value is assigned to the function
parameter. This transfers ownership:

```rust,editable
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}
```

<details>

* With the first call to `say_hello`, `main` gives up ownership of `name`. Afterwards, `name` cannot be used anymore within `main`.
* The heap memory allocated for `name` will be freed at the end of the `say_hello` function.
* `main` can retain ownership if it passes `name` as a reference (`&name`) and if `say_hello` accepts a reference as a parameter.
* Alternatively, `main` can pass a clone of `name` in the first call (`name.clone()`).
* Rust makes it harder than C++ to inadvertently create copies by making move semantics the default, and by forcing programmers to make clones explicit.

</details>
