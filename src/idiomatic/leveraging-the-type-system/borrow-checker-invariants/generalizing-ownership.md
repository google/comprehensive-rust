---
minutes: 15
---

# Generalizing Ownership

The logic of the borrow checker, while modelled off "memory ownership", can be
abstracted away from that use case to model other problems where we want to
prevent API misuse.

```rust,editable,compile_fail
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
    let deny_mut = shared_use(&value);
    let try_to_deny_immutable = exclusive_use(&mut value); // ❌🔨
    let more_mut_denial = &deny_mut;
    deny_future_use(value);
    let even_more_mut_denial = shared_use(&value); // ❌🔨
}
```

<details>

- This example re-frames the borrow checker rules away from references and
  towards semantic meaning in non-memory-safety settings. Nothing is being
  mutated, nothing is being sent across threads.

- Language features are often introduced for a specific purpose.

  Over time, users may develop ways of using that feature in ways that may have
  not been foreseen.

  In 2004, Java 5 introduced Generics with the
  [main stated purpose of enabling type-safe collections](https://jcp.org/en/jsr/detail?id=14).

  Since then, users and developers of the language expanded the use of generics
  to other areas of type-safe API design.
  <!-- TODO: Reference how this was adopted -->

  What we aim to do here is similar: Even though the borrow checker was
  introduced to prevent use-after-free and data races, it is just another API
  design tool. It can be used to model program properties that have nothing to
  do with preventing memory safety bugs.

- To use the borrow checker as a problem solving tool, we will need to "forget"
  that the original purpose of it is to prevent mutable aliasing in the context
  of preventing use-after-frees and data races, instead imagining and working
  within situations where the rules are the same but the meaning is slightly
  different.

- In rust's borrow checker we have access to three different ways of "taking" a
  value:

  <!-- TODO: actually link to the RAII section when it has been merged. -->
  - Owned value `T`. Very permissive case, to the point where mutability can be
    re-set, but demands that nothing else is using it in any context and drops
    the value when scope ends (unless that scope returns this value) (see:
    RAII.)

  - Mutable Reference `&mut T`. While holding onto a mutable reference we can
    still "dispatch" to methods and functions that take an immutable, shared
    reference of the value but only as long as we're not aliasing immutable,
    shared references to related data "after" that dispatch.

  - Shared Reference `&T`. Allows aliasing but prevents mutable access while any
    of these exist. We can't "dispatch" to methods and functions that take
    mutable references when all we have is a shared reference.

- Remember that every `&T` and `&mut T` has a lifetime, just one the user
  doesn't have to annotate or think about most of the time. We get to avoid
  annotating a lot of lifetimes because the rust compiler allows a user to elide
  the majority of them. See:
  [Lifetime Elision](../../../lifetimes/lifetime-elision.md).

</details>
