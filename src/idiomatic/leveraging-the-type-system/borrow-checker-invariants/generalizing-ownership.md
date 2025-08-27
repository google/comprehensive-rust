---
minutes: 0
---

The logic of the borrow checker, while tied to "memory ownership", can be abstracted away from this central use case to model other problems and prevent API misuse.

1. Mutual Exclusion of "&T" and "&mut T" references means we can look for places when designing an API where we need to model mutual exclusion.
2. The "consumption" of owned values means we can model values that can be used "only once"
3. Lifetime parameters & `PhantomData` let us define restrictive relationships between different values.

<details>



</details>