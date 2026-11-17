---
minutes: 30
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Binary Search Tree

A _binary tree_ is a tree-type data structure where each node has at most two
children (left and right). In this exercise, we will implement a _binary search
tree_, which maintains an ordering invariant: for every node, all values in its
left subtree are smaller than its value, and all values in its right subtree are
larger. The `insert` and `has` methods rely on this ordering to decide which
subtree to visit. A given value should only be stored in the tree once, i.e. no
duplicate nodes.

Implement the following types, so that the given tests pass.

```rust,compile_fail,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:types}}

// Implement `new` for `Node`.
// Implement `new`, `insert`, `len`, and `has` for `Subtree`.

{{#include exercise.rs:tests}}
```
