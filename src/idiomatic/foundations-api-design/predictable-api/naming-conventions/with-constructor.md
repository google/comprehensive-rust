---
minutes: 2
---

# `with` as constructor

`with` as a constructor sets one value among a type while using default values
for the rest.

`with` as in "`<Type>` with specific setting."

```rust,compile_fail
impl<T> Vec<T> {
    // Initializes memory for at least N elements, len is still 0.
    fn with_capacity(capacity: usize) -> Vec<T>;
}
```

<details>

- `with` can appear as a constructor prefix, typically when initializing heap
  memory for container types.

  In this case, it's distinct from `new` constructors because it specifies the
  value for something that is not usually cared about by API users.

- Ask the class: Why not `from_capacity`?

  Answer: `Vec::with_capacity` as a method call scans well as creating a "Vec
  with capacity". Consider how `Vec::new_capacity` or `Vec::from_capacity` scan
  when written down, they do not communicate what's going on well.

</details>
