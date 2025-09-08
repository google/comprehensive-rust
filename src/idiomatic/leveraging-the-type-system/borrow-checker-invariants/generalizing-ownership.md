---
minutes: 0
---

# Generalizing Ownership

The logic of the borrow checker, while modelled off "memory ownership", can be abstracted away from that use case to model other problems where we want to prevent API misuse.

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

let mut value = Data(Internal);
let deny_mut = shared_use(&value); 
let try_to_deny_immutable = exclusive_use(&mut value); // ❌🔨
let more_mut_denial = &deny_mut;
deny_future_use(value);
let even_more_mut_denial = shared_use(&value); // ❌🔨
```

<details> 

- This example re-frames the borrow checker rules away from references and towards semantic meaning in non-memory-safety settings. Nothing is being mutated, nothing is being sent across threads.

- To use the borrow checker as a problem solving tool, we will need to "forget" that the original purpose of it is to prevent mutable aliasing in the context of concurrency & dangling pointers, instead imagining and working within situations where the rules are the same but the meaning is slightly different.

- In rust's borrow checker we have access to three different ways of "taking" a value:

<!-- TODO: actually link to the RAII section when it has been merged. -->
- Owned value. Very permissive case of what you can do with it, but demands that nothing else is using it in any context and "drops" the value when scope ends (unless that scope returns this value) (see: RAII.)

- Mutable Reference. While holding onto a mutable reference we can still "dispatch" to methods and functions that take an immutable, shared reference of the value but only as long as we're not aliasing immutable, shared references to related data "after" that dispatch.

- Shared Reference. Allows aliasing but prevents mutable access while any of these exist. We can't "dispatch" to methods and functions that take mutable reference when all we have is a shared reference.

- Important to remember that every `&T` and `&mut T` has an _implicit lifetime._ We get to avoid annotating a lot of lifetimes because the rust compiler can infer the majority of them. See: [Lifetime Elision](../../../lifetimes/lifetime-elision.md)

- Potentially relevant: show how we can replace a lot of the `&` and `&mut` here with `&'a` and `&'a mut`.

</details>