---
minutes: 10
---

# Why no Inheritance in Rust?

```rust,compile_fail
pub struct Id {
    pub id: u32
}

impl Id {
    // methods
}

// 🔨❌, rust does not have inheritance!
pub struct Data: Id {
    // Inherited "id" field
    pub name: String,
}

impl Data {
    // methods, but also includes Id's methods, or maybe overrides to 
    // those methods.
}

// ✅
pub struct Data {
    pub id: Id,
    pub name: String,
}

impl Data {
    // All of data's methods that aren't from traits.
}

impl SomeTrait for Data {
    // Implementations for traits in separate impl blocks.
}
```

<details>

- Inheritance comes with a number of downsides.

- Heterogeneous by default:

  Class inheritance implicitly allows types of different classes to be used
  interchangeably, without being able to specify a concrete type or if a type is
  identical to another.

  For operations like equality, comparison this allows for comparison and
  equality that throws and error or otherwise panics.

- Multiple sources of truth for what makes up a data structure and how it
  behaves:

  A type's fields are obscured by the inheritance hierarchy.

  A type's methods could be overriding a parent type or be overridden by a child
  type, it's hard to tell what the behavior of a type is in complex codebases
  maintained by multiple parties.

- Dynamic dispatch as default adds overhead from vtable lookups:

  For dynamic dispatch to work, there needs to be somewhere to store information
  on what methods to call and other pieces of runtime-known pieces of
  information on the type.

  This store is the `vtable` for a value. Method calls will require more
  dereferences than calling a method for a type that is known at compile time.

</details>
