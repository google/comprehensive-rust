---
minutes: 2
---

# Get

Getting an element from a collection or container.

```rust
// Safe, fallible indexing.
let items: Vec<u8> = vec![1, 2, 3];
let option = items.get(0); // &Vec<u8> -> Option<&u8>
assert_eq!(Some(1), option);
let failed_option = items.get(1000); // &Vec<u8> -> Option<&u8>
assert_eq!(None, option);
```

<details>
- Gets are trivial, they get a value!

Immutable by default, for the most part.

Should not panic. May return an option or result, depending on the framework.

- Not for fields!

  For private fields you don't want users to have direct, assign-level access t
  a method with a more descriptive name (or the same name as the field) is
  preferred.

</details>
