---
minutes: 2
---

# Unchecked

Component for methods and functions where users need to manage safety manually.

```rust
NonNull::new_unchecked
Vec::split_at_unchecked
```

<details>
- Function is likely only callable in an unsafe block, or does not perform any runtime checks even in debug compilation.

- Users must be careful when using these functions, as they are responsible for
  making sure invariants are maintained.

- Inputs that do not maintain expected invariants, like passing a nullptr to
  NonNull::new_unchecked, will result in undefined behavior.

- Methods with this should have a "safety" comment in their documentation

</details>
