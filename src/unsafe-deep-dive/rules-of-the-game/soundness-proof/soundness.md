---
minutes: 2
---

# Soundness

A sound function is one that can't trigger UB if its safety preconditions are
satisfied.

<details>

- Read the definition of sound functions.

- Remind the students that the programmer who implements the caller is
  responsible for satisfying the safety precondition; the compiler is not
  helping.

- Translate into informal terms. Soundness means that the function is nice and
  plays by the rules. It documents its safety preconditions, and when the caller
  satisfies them, the function behaves well (no UB).

</details>
