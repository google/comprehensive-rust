---
minutes: 2
---

# Try

Prefix for fallible methods that return a `Result`.

```rust
```

<details>
- Prefix for methods that can fail, returning a `Result`.

- `TryFrom` is a `From`-like trait for types whose single-value constructors
  might fail in some way.

- Ask: Why aren't `Vec::get` and other similar methods `try_get`?

  There's only one possible error with get-style methods, out-of-bounds or
  key-does-not-exist style access errors.

  Also: get-style methods will be used frequently with data types like hash
  maps, sets. `get` is shorter and faster to scan visually than try_get, if try
  were more prolific.

</details>
