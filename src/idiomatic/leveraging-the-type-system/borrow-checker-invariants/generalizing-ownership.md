---
minutes: 10
---

# Lifetimes and Borrows: the Abstract Rules

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

fn demo_exclusive() {
    let mut value = Data(Internal);
    let shared = shared_use(&value);
    // let exclusive = exclusive_use(&mut value); // ❌🔨
    let shared_again = &shared;
}

fn demo_denied() {
    let value = Data(Internal);
    deny_future_use(value);
    // let shared = shared_use(&value); // ❌🔨
}

# fn main() {}
```

<details>

- This example re-frames the borrow checker rules away from references and
  towards semantic meaning in non-memory-safety settings.

  Nothing is being mutated, nothing is being sent across threads.

- In rust's borrow checker we have access to three different ways of "taking" a
  value:

  - Owned value `T`. Value is dropped when the scope ends, unless it is not
    returned to another scope.

  - Shared Reference `&T`. Allows aliasing but prevents mutable access while
    shared references are in use.

  - Mutable Reference `&mut T`. Only one of these is allowed to exist for a
    value at any one point, but can be used to create shared references.

- Ask: The two commented-out lines in the `demo` functions would cause
  compilation errors, Why?

  `demo_exclusive`: Because the `shared` value is still aliased after the
  `exclusive` reference is taken.

  `demo_denied`: Because `value` is consumed the line before the
  `shared_again_again` reference is taken from `&value`.

- Remember that every `&T` and `&mut T` has a lifetime, just one the user
  doesn't have to annotate or think about most of the time.

  We rarely specify lifetimes because the Rust compiler allows us to _elide_
  them in most cases. See:
  [Lifetime Elision](../../../lifetimes/lifetime-elision.md)

</details>
