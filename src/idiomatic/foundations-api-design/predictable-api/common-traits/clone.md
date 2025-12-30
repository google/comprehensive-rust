---
minutes: 5
---

# Clone

Deep-copy a type or duplicate a smart, shareable pointer.

Derivable: ✅

When to implement: If duplicating doesn't break invariants.

```rust
// pub trait Clone: Sized {
//     // Required method
//     fn clone(&self) -> Self;
// 
//     // Provided methods omitted
// }

use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Clone)]
pub struct LotsOfData {
    string: String,
    vec: Vec<u8>,
    set: BTreeSet<u8>,
}

let lots_of_data = LotsOfData {
    string: "String".to_string(),
    vec: vec![1; 255],
    set: BTreeSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8]),
};

let lots_of_data_cloned = lots_of_data.clone();

let reference_counted = Rc::new(lots_of_data);
// Copies the reference-counted pointer, not the value.
let reference_copied = reference_counted.clone();
```

<details>

- "Deep copy" a value, or in the case of reference counting pointers like
  `Rc`/`Arc` create a new instance of that pointer.

- When to not implement/derive: For types that, to maintain an invariant, the
  value should not be duplicated. We'll touch on this later in Idiomatic Rust.

</details>
