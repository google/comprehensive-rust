---
minutes: 5
---

The logic of the borrow checker, while modelled off "memory ownership", can be abstracted away from that use case to model other problems where we want to prevent API misuse.

```rust,editable
pub struct InternalData;
pub struct Value(InternalData);

fn shared_use(value: &Value) -> &InternalData {
    &value.0
}

fn exclusive_use(value: &mut Value) -> &mut InternalData {
    &mut value.0
}

fn deny_future_use(value: Value) {}

let mut value = Value(InternalData);
let deny_mut = shared_use(&value);
let try_to_deny_immutable = exclusive_use(&mut value); // ❌🔨
let more_mut_denial = &deny_mut;
deny_future_use(value);
let even_more_mut_denial = shared_use(&value); // ❌🔨
```

<details>

- To use the borrow checker as a problem solving tool, we will need to "forget" that the original purpose of it is to prevent mutable aliasing in the context of concurrency, instead imagining and working within situations where the rules are the same but the meaning is slightly different.

- In rust's borrow checker we have access to three different ways of "taking" a value:

<!-- TODO: actually link to the RAII section when it has been merged. -->
- Owned value. Very permissive case of what you can do with it, but demands that nothing else is using it in any context and "drops" the value when scope ends (unless that scope returns this value) (see: RAII.)

- Mutable Reference, while holding onto a mutable reference we can still "dispatch" to methods and functions that take an immutable, shared reference of the value but only as long as we're not aliasing immutable, sharable references to related data "after" that dispatch.

- Shareable Reference, allows aliasing but prevents mutable access while any of these exist. We can't "dispatch" to methods and functions that take mutable reference when all we have is a shared reference.

- Important to note that every `&T` and `&mut T` has an _implicit lifetime._ We get to avoid annotating a lot of lifetimes because the rust compiler can infer the majority of them. See: [Lifetime Elision]("../../../../../lifetimes/lifetime-elision.md")

- Potentially relevant: show how we can replace a lot of the `&` and `&mut` here with `&'a` and `&'a mut`.

</details>