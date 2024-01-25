---
minutes: 10
---

# Interior Mutability

In some situations, it's necessary to modify data behind a shared (read-only)
reference. For example, a shared data structure might have an internal cache,
and wish to update that cache from read-only methods.

The "interior mutability" pattern allows exclusive (mutable) access behind a
shared reference. The standard library provides several ways to do this, all
while still ensuring safety, typically by performing a runtime check.

## `RefCell`

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

## `Cell`

`Cell` wraps a value and allows getting or setting the value, even with a shared
reference to the `Cell`. However, it does not allow any references to the value.
Since there are no references, borrowing rules cannot be broken.

<details>

The main thing to take away from this slide is that Rust provides _safe_ ways to
modify data behind a shared reference. There are a variety of ways to ensure
that safety, and `RefCell` and `Cell` are two of them.

- `RefCell` enforces Rust's usual borrowing rules (either multiple shared
  references or a single exclusive reference) with a runtime check. In this
  case, all borrows are very short and never overlap, so the checks always
  succeed.

- `Rc` only allows shared (read-only) access to its contents, since its purpose
  is to allow (and count) many references. But we want to modify the value, so
  we need interior mutability.

- `Cell` is a simpler means to ensure safety: it has a `set` method that takes
  `&self`. This needs no runtime check, but requires moving values, which can
  have its own cost.

- Demonstrate that reference loops can be created by adding `root` to
  `subtree.children`.

- To demonstrate a runtime panic, add a `fn inc(&mut self)` that increments
  `self.value` and calls the same method on its children. This will panic in the
  presence of the reference loop, with
  `thread 'main' panicked at 'already borrowed: BorrowMutError'`.

</details>
