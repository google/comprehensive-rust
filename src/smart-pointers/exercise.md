---
minutes: 30
---

# Exercise: Binary Tree

A binary tree is a tree-type data structure where every node has two children
(left and right). We will create a tree where each node stores a value. For a
given node N, all nodes in a N's left subtree contain smaller values, and all
nodes in N's right subtree will contain larger values.

Implement the following types, so that the given tests pass.

Extra Credit: implement an iterator over a binary tree that returns the values
in order.

```rust,editable,ignore
{{#include exercise.rs:types}}

// Implement `new`, `insert`, `len`, and `has` for `Subtree`.

{{#include exercise.rs:tests}}
```
