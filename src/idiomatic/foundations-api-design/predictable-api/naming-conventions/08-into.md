---
Minutes: 2
---

# Into

Prefix for a type transformation style method.

```rust
Vec::into_parts
Cell::into_inner
IntoIter::into_iter
```

<details>
- Prefix for a function that consumes an owned value and transforms it into a value of another type.

Not reinterpret cast! The data can be rearranged, reallocated, changed in any
way, including losing information.

- corollary to `From`

- `into_iter` consumes a collection (like a vec, or a btreeset, or a hashmap)
  and produces an iterator over owned values, unlike `iter` and `iter_mut` which
  produce iterators over reference values.

- Ask the class: what will `Vec::into_parts` do?

</details>
