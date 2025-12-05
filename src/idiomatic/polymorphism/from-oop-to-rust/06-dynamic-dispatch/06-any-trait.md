---
minutes: 10
---

# Any Trait and Downcasting

```rust
use std::any::Any;

#[derive(Debug)]
pub struct ThisImplementsAny;

fn take_any<T: Any>(t: &T) {}

fn main() {
    let is_an_any = ThisImplementsAny;
    take_any(&is_an_any);

    let dyn_any: &dyn Any = &is_an_any;
    dbg!(dyn_any.type_id());
    dbg!(dyn_any.is::<ThisImplementsAny>());
    let is_downcast: Option<&ThisImplementsAny> = dyn_any.downcast_ref();
    dbg!(is_downcast);
}
```

<details>
- The `Any` trait allows us to downcast values back from dyn values into concrete values.

This is an auto trait: like Send/Sync/Sized, it is automatically implemented for
any type that meets specific criteria.

The criteria for Any is that a type is `'static`. That is, the type does not
contain any non-`'static` lifetimes within it.

- Any offers two related behaviors: downcasting, and runtime checking of types
  being the same.

  In the example above, we see the ability to downcast from `Any` into
  `ThisImplementsAny` automatically.

  We also see `Any::is` being used to check to see what type the value is.

- `Any` does not implement reflection for a type, this is all you can do with
  `Any`.

</details>
