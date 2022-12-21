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
