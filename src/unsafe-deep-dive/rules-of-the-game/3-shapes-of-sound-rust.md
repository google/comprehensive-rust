---
minutes: 5
---

# 3 Shapes of Sound Rust

- Functions written only in Safe Rust
- Functions that contain `unsafe` blocks which are impossible to misuse
- Unsafe functions that have their safety preconditions documented

<details>

- We want to write sound code.
- Sound code can only have the following shapes:
  - safe functions that contain no unsafe blocks
  - safe functions that completely encapsulate unsafe blocks, meaning that the
    caller does not need to know about them
  - unsafe functions that contain unsafe blocks but don't encapsulate them, and
    pass the proof burden to their caller
- Burden of proof
  - safe functions with only Safe Rust -> compiler
  - safe functions with unsafe blocks -> function author
  - unsafe functions -> function caller

</details>
