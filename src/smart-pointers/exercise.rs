// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: solution
use std::cmp::Ordering::*;

// ANCHOR: types
/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord + Copy> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
type Subtree<T> = Option<Box<Node<T>>>;

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord + Copy> {
    root: Subtree<T>,
}
// ANCHOR_END: types

impl<T: Ord + Copy> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(Box::new(Node::new(value))),
            Some(n) => n.insert(value),
        }
    }

    fn has(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |n| n.has(value))
    }

    fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.len())
    }
}

impl<T: Ord + Copy> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match (&mut self.left, value.cmp(&self.value), &mut self.right) {
            (Some(ref mut l), Less, _) => l.insert(value),
            (None, Less, _) => self.left = Some(Box::new(Node::new(value))),
            (_, Greater, Some(ref mut r)) => r.insert(value),
            (_, Greater, None) => self.right = Some(Box::new(Node::new(value))),
            (_, Equal, _) => {}
        }
    }

    fn has(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Less => self.left.as_ref().map_or(false, |n| n.has(value)),
            Equal => true,
            Greater => self.right.as_ref().map_or(false, |n| n.has(value)),
        }
    }

    fn len(&self) -> usize {
        1 + self.left.as_ref().map_or(0, |n| n.len())
            + self.right.as_ref().map_or(0, |n| n.len())
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert("foo");
    assert_eq!(tree.len(), 1);
    tree.insert("bar");
    assert!(tree.has("foo"));
}

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(i as i32)).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(50));
    }
}
// ANCHOR_END: tests
