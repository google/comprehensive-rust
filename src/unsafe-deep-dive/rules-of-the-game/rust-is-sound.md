---
minutes: 5
---

# Rust is sound

- Soundness is fundamental to Rust
- Soundness ≈ impossible to cause memory safety problems
- Sound functions have common “shapes”

<details>

“A fundamental principle of Rust code is that it is sound.

“We’ll create a formal definition of the term soundness shortly. In the
meantime, think of sound code as code that cannot trigger memory safety
problems.

“Sound code is made up of _sound functions_ and _sound operations_.

“A sound function as a function where none of its possible inputs could provoke
soundness problems.

Sound functions have common shapes.

Those shapes are what we’ll look at now.

“We’ll start with one that’s implemented in Safe Rust, and then see what could
happen when we introduce `unsafe` to different parts.

</details>
