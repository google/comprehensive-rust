# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **Zero-Copy Parsing:** This solution demonstrates zero-copy parsing. The
  `Person` and `PhoneNumber` structs borrow string slices (`&'a str`) directly
  from the input byte array (`&'a [u8]`). No new strings are allocated for the
  field values.
- **Lifetimes:** The lifetime `'a` ensures that the returned `Person` cannot
  outlive the input data buffer. If the buffer is dropped, the `Person` becomes
  invalid, which the compiler enforces.
- **Slicing:** We use `split_at` to divide the input buffer into the data for
  the current field and the remainder. This is efficient as it only manipulates
  pointers and lengths, not the data itself.
- **Recursion:** The parsing is recursive. `Person` contains `PhoneNumber`s,
  which are themselves parsed by calling `parse_message` on the bytes contained
  within the `Len` wire type.

<details>

- Note that `parse_message` requires `T` to implement `Default` so it can start
  with an empty message and fill it in.
- The `WireType` enum and `parse_varint` function handle the low-level encoding
  details, allowing the main logic to focus on structure.

</details>
