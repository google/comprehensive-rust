---
minutes: 10
---

<!-- NOTES:
Introduce the concept, with an example based on Mutex showing an `&self` method doing mutation; reference Cell/RefCell without detail.
-->

# Interior Mutability

Rust provides a few safe means of modifying a value given only a shared
reference to that value. All of these replace compile-time checks with runtime
checks.

## `Cell` and `RefCell`

[`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html) and
[`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) implement
what Rust calls _interior mutability:_ mutation of values in an immutable
context.

`Cell` is typically used for simple types, as it requires copying or moving
values. More complex interior types typically use `RefCell`, which tracks shared
and exclusive references at runtime and panics if they are misused.

```rust,editable
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, ..Node::default() }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

fn main() {
    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(5));
    let subtree = Node::new(10);
    subtree.borrow_mut().children.push(Node::new(11));
    subtree.borrow_mut().children.push(Node::new(12));
    root.borrow_mut().children.push(subtree);

    println!("graph: {root:#?}");
    println!("graph sum: {}", root.borrow().sum());
}
```

<details>

- If we were using `Cell` instead of `RefCell` in this example, we would have to
  move the `Node` out of the `Rc` to push children, then move it back in. This
  is safe because there's always one, un-referenced value in the cell, but it's
  not ergonomic.
- To do anything with a Node, you must call a `RefCell` method, usually `borrow`
  or `borrow_mut`.
- Demonstrate that reference loops can be created by adding `root` to
  `subtree.children` (don't try to print it!).
- To demonstrate a runtime panic, add a `fn inc(&mut self)` that increments
  `self.value` and calls the same method on its children. This will panic in the
  presence of the reference loop, with
  `thread 'main' panicked at 'already borrowed: BorrowMutError'`.

</details>
