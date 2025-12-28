---
minutes: 5
---

# Limits of Trait Objects

```rust
use std::any::Any;

pub trait Trait: Any {}

impl Trait for i32 {}

fn main() {
    dbg!(size_of::<i32>()); // 4 bytes, owned value
    dbg!(size_of::<&i32>()); // 8 bytes, reference
    dbg!(size_of::<&dyn Trait>()); // 16 bytes, wide pointer
}
```

<details>
- Trait objects are a limited way of solving problems.

- If you want to downcast to a concrete type from a trait object, you will need
  to specify that the trait in question has Any as a supertrait or that the
  trait object is over the main trait and `Any`.

  Even then, you will still need to cast a `dyn MyTrait` to `dyn Any`

- Trait objects have overhead in memory, they are "wide pointers" that need to
  hold not just the pointer to the data itself but another pointer for the
  vtable.

- Trait objects, being dynamically sized types, can only be used practically via
  reference or pointer types.

  There is a baseline overhead of dereferencing the value and relevant trait
  methods when using trait objects.

</details>
