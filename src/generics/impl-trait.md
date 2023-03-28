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

* `impl Trait` allows you to work with types which you cannot name.

<details>

The meaning of `impl Trait` is a bit different in the different positions.

* For a parameter, `impl Trait` is like an anonymous generic parameter with a trait bound.

* For a return type, it means that the return type is some concrete type that implements the trait,
  without naming the type. This can be useful when you don't want to expose the concrete type in a
  public API.

  Inference is hard in return position. A function returning `impl Foo` picks
  the concrete type it returns, without writing it out in the source. A function
  returning a generic type like `collect<B>() -> B` can return any type
  satisfying `B`, and the caller may need to choose one, such as with `let x:
  Vec<_> = foo.collect()` or with the turbofish, `foo.collect::<Vec<_>>()`.

This example is great, because it uses `impl Display` twice. It helps to explain that
nothing here enforces that it is _the same_ `impl Display` type. If we used a single 
`T: Display`, it would enforce the constraint that input `T` and return `T` type are the same type.
It would not work for this particular function, as the type we expect as input is likely not
what `format!` returns. If we wanted to do the same via `: Display` syntax, we'd need two
independent generic parameters.

</details>
