# Binary Tree

A Binary Tree is a tree-type data structure where every node has two children (left and right).
We want to create a tree where each node would be assigned a variant `Left`, `Right` or `Root`
depdending on its original position related to its parent. The side is defined in the following enum:

```rust
#[derive(Debug)]
enum Side {
    Left,
    Right,
    Root
}
```

Using the data structures in the standard library, how can we represent such a Binary Tree in Rust?

<details solution>

```rust
#[derive(Debug)]
struct BinaryTree {
    value: Side,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>
}
```

</details>

Let's now write a method to to create a BinaryTree with arbitrary depth:

<details solution>

```rust
use Side::*;

impl BinaryTree {
    fn new_with_depth(depth: i32, side: Side) -> Option<Box<BinaryTree>> {
        if depth == 0 {
            return None;
        }
        let left = Self::new_with_depth(depth - 1, Left);
        let right = Self::new_with_depth(depth - 1, Right);
        Some(Box::new(BinaryTree {
            value: side,
            left,
            right
        }))
    }
}
```

</details>

Let's now write a method to invert the tree. To make sure that the tree is inverted, 
left nodes values should now be `Right` and right nodes values should now be `Left`.


<details solution>

```rust
use Side::*;

impl BinaryTree {
    fn invert(&mut self) {
        let mut left = self.left.take();
        if let Some(left) = &mut left {
            left.invert();   
        }
        let mut right = self.right.take();
        if let Some(right) = &mut right {
            right.invert();
        }
        self.left = right;
        self.right = left;
    }
}
```

</details>
