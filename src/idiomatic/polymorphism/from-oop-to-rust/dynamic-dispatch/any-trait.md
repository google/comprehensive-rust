---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Any Trait and Downcasting

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::any::Any;

#[derive(Debug)]
pub struct ThisImplementsAny;

fn take_any(dyn_any: &dyn Any) {
    // We can get a unique identifier for the type.
    dbg!(dyn_any.type_id());

    // We can check if our object is a particular type.
    dbg!(dyn_any.is::<ThisImplementsAny>());

    // We can attempt to downcast to a concrete type.
    if let Some(concrete) = dyn_any.downcast_ref::<ThisImplementsAny>() {
        dbg!(concrete);
    }
}

fn main() {
    take_any(&ThisImplementsAny);
    take_any(&123);
    take_any(&"A string");
}
```

<details>

- By default, trait objects cannot be downcasted to their concrete type. This is
  because they do not have any runtime type information that would allow us to
  determine what the concrete type is, and Rust won't allow us to blindly cast
  to a type that may not be correct.

- The `Any` trait allows us to downcast values back from dyn values into
  concrete values by adding the necessary runtime type information.

- This is an auto trait: like Send/Sync/Sized, it is automatically implemented
  for any type that meets specific criteria.

- The criteria for Any is that a type is `'static`. That is, the type does not
  contain any non-`'static` lifetimes within it.

- Any offers two related behaviors: downcasting, and runtime checking of types
  being the same.

  In the example above, we see the ability to downcast from `Any` into
  `ThisImplementsAny` automatically.

  We also see `Any::is` being used to check to see what type the value is.

- `Any` does not implement reflection for a type, it just adds the runtime type
  information necessary to safely determine the concrete type at runtime.

- You can add downcasting support to your own traits by using `Any` as a
  supertrait.

- This also works for generics! We can use `Any` as a trait bound in our generic
  code, and then "downcast" to a concrete type. This is uncommon, though, as
  generics already have full type information and access to all functionality
  provided by traits.

</details>
