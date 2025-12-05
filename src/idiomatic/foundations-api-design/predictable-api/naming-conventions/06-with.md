---
minutes: 5
---

# With

Prefix for builder-style configuration.

```rust
{Vec, BTreeSet}::with_capacity
PathBuf::with_extension
PathBuf::with_file_name
```

<details>
- Prefix for methods that set something internally.

Can be constructors, builders, or setters.

- Constructor-style `with` methods usually set one specific field but leave
  everything else "default"

  `with_capacity` allocates enough space for the number of elements given, but
  does not otherwise add anything to the data structure.

- When `with` methods take an owned value, they're builder-style.

- When `with` methods take a reference, like pathbuf's `with` methods, they
  return a new owned value while the original value remains.

</details>
