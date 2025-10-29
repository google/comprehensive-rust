---
minutes: 5
---

# Generalizing Ownership: Lifetimes Refresher

The logic of the borrow checker, while modelled off "memory ownership", can be
abstracted away from that use case to model other problems where we want to
prevent API misuse.

```rust,editable
// An internal data type to have something to hold onto.
pub struct Internal;
// The "outer" data.
pub struct Data(Internal);

fn shared_use(value: &Data) -> &Internal {
    &value.0
}
fn exclusive_use(value: &mut Data) -> &mut Internal {
    &mut value.0
}
fn deny_future_use(value: Data) {}

fn main() {
    let mut value = Data(Internal);
    let shared = shared_use(&value);
    // let exclusive = exclusive_use(&mut value); // ❌🔨
    let shared_again = &shared;
    deny_future_use(value);
    // let shared_again_again = shared_use(&value); // ❌🔨
}
```

<details>

- This example re-frames the borrow checker rules away from references and
  towards semantic meaning in non-memory-safety settings. Nothing is being
  mutated, nothing is being sent across threads.

- In rust's borrow checker we have access to three different ways of "taking" a
  value:

  - Owned value `T`. Value is dropped when the scope ends, unless it is not
    returned to another scope.

  - Shared Reference `&T`. Allows aliasing but prevents mutable access while
    shared references are in use.

  - Mutable Reference `&mut T`. Only one of these is allowed to exist for a
    value at any one point, but can be used to create shared references.

- Ask: The two commented-out lines in `main` would cause compilation errors,
  Why?

  1: Because the `shared` value is still aliased after the `exclusive` reference
  is taken.

  2: Because `value` is consumed (AKA dropped) the line before the
  `shared_again_again` reference is taken from `&value`.

- Remember that every `&T` and `&mut T` has a lifetime, just one the user
  doesn't have to annotate or think about most of the time. We get to avoid
  annotating a lot of lifetimes because the rust compiler allows a user to elide
  the majority of them. See:
  [Lifetime Elision](../../../lifetimes/lifetime-elision.md)

</details>
