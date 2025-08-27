---
minutes: 0
---

# Using the Borrow checker to enforce Invariants

We can use the rules of the borrow checker to model things other than what is valid to mutate and read at various points in a program.

<details>

- The borrow checker has been used to prevent use-after-free and multiple mutable references up until this point.

- But the constraints of the borrow checker can be used to prevent misuse of APIs.

- Interior, private mutability or reference counting in data types lets an API designer shift the meaning of ownership to a different (but analogous) set of semantics as they need to, even to the point where some API designers have managed to model self-referential types this way.

</details>