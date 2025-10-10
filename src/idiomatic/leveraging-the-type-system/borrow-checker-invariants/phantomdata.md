---
minutes: 10
---

# PhantomData: Tagging identical data with different types

```rust,editable
pub struct TaggedData<T> {
    data: String,
    _phantom: PhantomData<T>,
}

fn main() {}
```

<details>

- Motivation: We want to be able to tag structures with different type
  parameters as a way to tell them apart or pass on lifetime information to
  them.

  In practice, these "tags" tend to be zero-sized types. What they mean will
  depend on the shape and context of the API they're a part of.

- Demonstrate: <!-- TODO -->

- `PhantomData` lets developers "tag" types with type and lifetime parameters
  that are not "really" present in the struct or enum.

  `PhantomData` can be used with the Typestate pattern to have data with the
  same structure i.e. `TaggedData<Start>` can have methods or trait
  implementations that `TaggedData<End>` doesn't.

- This can be thought of as an extension over how zero-sized types are all "the
  same structure" but with different types.

</details>
