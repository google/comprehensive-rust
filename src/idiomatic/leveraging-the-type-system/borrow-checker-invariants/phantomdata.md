---
minutes: 10
---

# Lifetime Relationships primer: PhantomData

Lying to the type system safely, to make things easier for us and encode
invariants better.

```rust,editable,compile_fail
// use std::marker::PhantomData;

pub struct TypeTaggedString<T> {
    data: String,
}

fn main() {}
```

<details>

- Ask: Why won't this slide compile?

  Answer: Unused type parameters!

- Motivation: We want to be able to "tag" structures with different type
  parameters as a way to tell them apart or pass on lifetime information to
  them.

  See: [Typestate Generics](../typestate-pattern/typestate-generics.md) for
  instances of telling apart different data relevant to stages of an algorithm
  with type parameter differences.

  In practice, these "tags" tend to be zero-sized types. What they mean will
  depend on the shape and context of the API they're a part of.

- Demonstrate: Add a field of type `marker: T` to `TypeTaggedString`

  Ask: What issues does having it be an actual instance of that type pose?

  Answer: If it's not a zero-sized type (like `()` or `struct MyTag;`), then
  we're allocating more memory than we need to when all we care for is type
  information.

  Alternatively: Makes initializing the data a pain for users and the
  maintainers of the library alike, as there needs to be an additional parameter
  for unnecessary data.

- Demonstrate: Uncomment the `PhantomData` import, and implement the following:

  ```rust,compile_fail
  pub struct TypeTaggedString<T> {
      data: String,
      _phantom: PhantomData<T>,
  }
  ```

- `PhantomData` lets developers "tag" types with type and lifetime parameters
  that are not "really" present in the struct or enum.

  `PhantomData` can be used with the Typestate pattern to have data with the
  same structure i.e. `TaggedData<Start>` can have methods or trait
  implementations that `TaggedData<End>` doesn't.

- This can be thought of as a more general take on how zero-sized types are all
  "the same structure" (zero size = only one possible value) but with different
  types.

- Ask: Why don't we just do newtypes for every case as they come up?

  Answer: Avoiding repeating ourselves! Would have to implement any methods
  again for every new type. Having information in a `<T>` argument for a type
  that gets erased at run-time lets us write implementations that work for all
  possible values of `T`. Newtyping all possible use cases means a lot of
  re-implementation.

  Alternatively, a type parameter for a type helps provide semantic context to a
  user. `Stage<Start>` offers a lot more contextual detail than `StageStart`,
  and implies there is shared behavior between stages.

- Demonstrate: We can also capture lifetime parameters with `PhantomData`
  without needing to hold onto a value with that lifetime.

  Implement the following:

  ```rust,compile_fail
  pub struct LifetimeTaggedString<'a> {
      data: String,
      _phantom: PhantomData<&'a ()>,
  }
  ```

  Ask: Why would we do this?

  Answer: To tie an owned value to another value's lifetime elsewhere in the
  program.

</details>
