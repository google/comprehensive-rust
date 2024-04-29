---
Minutes: 5
---

# Generic Traits

Traits can also be generic, just like types and functions. A trait's parameters
get concrete types when it is used.

```rust,editable
#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    println!("{from_int:?}, {from_bool:?}");
}
```

<details>

- The `From` trait will be covered later in the course, but its
  [definition in the `std` docs](https://doc.rust-lang.org/std/convert/trait.From.html)
  is simple.

- Implementations of the trait do not need to cover all possible type
  parameters. Here, `Foo::From("hello")` would not compile because there is no
  `From<&str>` implementation for `Foo`.

- Generic traits take types as "input", while associated types are a kind of
  "output" type. A trait can have multiple implementations for different input
  types.

- In fact, Rust requires that at most one implementation of a trait match for
  any type T. Unlike some other languages, Rust has no heuristic for choosing
  the "most specific" match. There is work on adding this support, called
  [specialization](https://rust-lang.github.io/rfcs/1210-impl-specialization.html).

</details>
