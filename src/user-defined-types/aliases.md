---
minutes: 2
---

# Type Aliases

A type alias creates a name for another type. The two types can be used
interchangeably.

```rust,editable
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
```

<details>

C programmers will recognize this as similar to a `typedef`.

</details>
