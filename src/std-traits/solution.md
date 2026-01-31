# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **Wrapper Pattern:** `RotDecoder` wraps another reader `R`.
- **Delegation:** The `read` implementation first calls `self.input.read(buf)`
  to read data into the buffer. This is essential; we need raw data before we
  can transform it.
- **In-place Mutation:** After reading, the code iterates over the populated
  part of the buffer (`buf[..size]`) and modifies the bytes in place.
- **Error Handling:** The `?` operator is used to check the result of
  `self.input.read`. If it fails, the error is immediately returned.

<details>

- Mention that `rot` is a simple Caesar cipher.
- Chaining two ROT13 instances (rotations by 13) results in a rotation by 26,
  which wraps around to the original text (for English alphabets). This property
  is why ROT13 is its own inverse.

</details>
