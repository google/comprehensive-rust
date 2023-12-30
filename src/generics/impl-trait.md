---
minutes: 5
---

# `impl Trait`

Similar to trait bounds, an `impl Trait` syntax can be used in function
arguments and return values:

```rust,editable
// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("debuggable: {debuggable:?}");
}
```

<details>

`impl Trait` allows you to work with types which you cannot name. The meaning of
`impl Trait` is a bit different in the different positions.

- For a parameter, `impl Trait` is like an anonymous generic parameter with a
  trait bound.

- For a return type, it means that the return type is some concrete type that
  implements the trait, without naming the type. This can be useful when you
  don't want to expose the concrete type in a public API.

  Inference is hard in return position. A function returning `impl Foo` picks
  the concrete type it returns, without writing it out in the source. A function
  returning a generic type like `collect<B>() -> B` can return any type
  satisfying `B`, and the caller may need to choose one, such as with
  `let x: Vec<_> = foo.collect()` or with the turbofish,
  `foo.collect::<Vec<_>>()`.

What is the type of `debuggable`? Try `let debuggable: () = ..` to see what the
error message shows.

</details>
