# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **Recursive Types with `Box`:** `Node` refers to `Subtree`, which refers to
  `Node` (via `Box`). `Box<Node<T>>` has a fixed size (pointer size), which
  breaks the cycle of infinite size that would occur if `Node` directly
  contained other `Node`s.
- **Nullability with `Option`:** We use `Option<Box<Node<T>>>` to represent a
  potentially empty subtree. Rust does not have null pointers; `Option` is the
  standard way to express absence.
- **Newtype Pattern:** `Subtree` wraps `Option<Box<Node<T>>>`. This allows us to
  define methods like `insert` and `len` directly on `Subtree`, keeping the
  `Node` logic simple. It abstracts away the implementation detail that an empty
  tree is `None`.
- **References in `match`:** Notice `match &self.0` and `match &mut self.0`. We
  match on references to the inner `Option` so that we can inspect or modify the
  tree without taking ownership of the nodes (which would destroy the tree).

<details>

- Discuss why `Box` is needed. Without it, `sizeof(Node)` would depend on
  `sizeof(Node)`, which is impossible to resolve.
- This is a standard BST implementation. Since it's not balanced, performance
  can degrade to O(N) if elements are inserted in sorted order.

</details>
