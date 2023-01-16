# `impl Trait`

Similar to trait bounds, an `impl Trait` syntax can be used in function
arguments and return values:

```rust,editable
use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let x = get_x("foo");
    println!("{x}");
}
```

* `impl Trait` cannot be used with the `::<>` turbo fish syntax.
* `impl Trait` allows you to work with types which you cannot name.

<details>

The meaning of `impl Trait` is a bit different in the different positions.

* For a parameter, `impl Trait` is like an anonymous generic parameter with a trait bound.
* For a return type, it means that the return type is some concrete type that implements the trait,
  without naming the type. This can be useful when you don't want to expose the concrete type in a
  public API.

</details>
