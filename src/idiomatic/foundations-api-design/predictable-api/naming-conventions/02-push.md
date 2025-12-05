---
minutes: 2
---

# Push

Common on array-like structures.

```rust
use std::collections::VecDeque;

let mut items: Vec<u8> = Vec::new();
items.push(42);
let mut items_queue: VecDeque<u8> = VecDeque::new();
items_queue.push_front(233);
// [233]
items_queue.push_back(42);
// [233, 42]
items_queue.push_front(128);
// [128, 233, 42]
dbg!(items_queue);
```

<details>
- Modifies an array-like structure by adding an element.

Needs mutable access!

</details>
