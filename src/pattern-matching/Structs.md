---
minutes: 8
---

# Structs

Like tuples, Struct can also be destructured by matching:

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-structs.rs}}
```

<details>

- Change the literal values in `foo` to match with the other patterns.
- Add a new field to `Foo` and make changes to the pattern as needed.
- The distinction between a capture and a constant expression can be hard to
  spot. Try changing the `2` in the second arm to a variable, and see that it
  subtly doesn't work. Change it to a `const` and see it working again.

</details>
