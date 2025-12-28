---
minutes: 5
---

# `dyn Trait` for Dynamic Dispatch in Rust

```rust
pub trait Trait {}

impl Trait for i32 {}
impl Trait for String {}

fn main() {
    let int: &dyn Trait = &42i32;
    let string: &dyn Trait = &("Hello dyn!".to_owned());
}
```

<details>
- Dynamic Dispatch is a tool in Object Oriented Programming that is often used in places where one needs to care more about the behavior of a type than what the type is.

In OOP languages, dynamic dispatch is often an _implicit_ process and not
something you can opt out of.

In rust, we use `dyn Trait`: An opt-in form of dynamic dispatch.

- For any trait that is _dyn compatible_ we can coerce a reference to a value of
  that trait into a `dyn Trait` value.

- We call these _trait objects_. Their type is not known at compile time, but
  their behavior is: what is implemented by the trait itself.

- When you _need_ OOP-style heterogeneous data structures, you can reach for
  `Box<dyn Trait>`, but try to keep it homogeneous and generic-based first!

</details>
