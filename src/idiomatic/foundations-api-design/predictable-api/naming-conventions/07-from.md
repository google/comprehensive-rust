---
minutes: 2
---

# From

Prefix for a constructor-style function.

```rust
// TODO: more examples
Vec::from_raw_parts, from_iter, from_le_bytes
```

<details>
- Prefix for constructor-style, `From`-trait-style functions.

- These functions can take multiple arguments, but usually imply the user is
  doing more of the work than a usual constructor would.

  `new` is still preferred for most constructor-style functions, the implication
  for `from` is transformation of one data type to another.

- Ask: What will `from_iter` do?

</details>
