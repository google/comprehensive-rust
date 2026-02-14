---
minutes: 5
---

<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `dyn Trait`

In addition to using traits for static dispatch via generics, Rust also supports
using them for type-erased, dynamic dispatch via trait objects:

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

// Uses generics and static dispatch.
fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch.
fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

fn main() {
    let cat = Cat { lives: 9 };
    let dog = Dog { name: String::from("Fido"), age: 5 };

    generic(&cat);
    generic(&dog);

    dynamic(&cat);
    dynamic(&dog);
}
```

<details>

- Generics, including `impl Trait`, use monomorphization to create a specialized
  instance of the function for each different type that the generic is
  instantiated with. This means that calling a trait method from within a
  generic function still uses static dispatch, as the compiler has full type
  information and can resolve that type's trait implementation to use.

- When using `dyn Trait`, it instead uses dynamic dispatch through a
  [virtual method table][vtable] (vtable). This means that there's a single
  version of `fn dynamic` that is used regardless of what type of `Pet` is
  passed in.

- When using `dyn Trait`, the trait object needs to be behind some kind of
  indirection. In this case it's a reference, though smart pointer types like
  `Box` can also be used (this will be demonstrated on day 3).

- At runtime, a `&dyn Pet` is represented as a "fat pointer", i.e. a pair of two
  pointers: One pointer points to the concrete object that implements `Pet`, and
  the other points to the vtable for the trait implementation for that type.
  When calling the `talk` method on `&dyn Pet` the compiler looks up the
  function pointer for `talk` in the vtable and then invokes the function,
  passing the pointer to the `Dog` or `Cat` into that function. The compiler
  doesn't need to know the concrete type of the `Pet` in order to do this.

- A `dyn Trait` is considered to be "type-erased", because we no longer have
  compile-time knowledge of what the concrete type is.

[vtable]: https://en.wikipedia.org/wiki/Virtual_method_table

</details>
