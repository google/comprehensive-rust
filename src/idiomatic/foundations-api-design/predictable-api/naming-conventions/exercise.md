---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise

1. What do these names imply they do?
2. What should we name these signatures?

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// What are the types of these methods?
Option::is_some // ?
slice::get // ?
slice::get_unchecked_mut // ?
Option::as_ref // ?
str::from_utf8_unchecked_mut // ?
Rc::get_mut // ?
Vec::dedup_by_key // ?

// What should we name methods with these types?
fn ____(String) -> Self;
fn ____(&self) -> Option<&InnerType>; // details for InnerType do not matter.
fn ____(self, String) -> Self;
fn ____(&mut self) -> Option<&mut InnerType>;
```

<details>

- Go through the methods in the example with the class and discuss what the
  types of the functions should be.

- Go through the unnamed methods and brainstorm what names those methods should
  have.

  Answers for missing types:
  - `Option::is_some(&self) -> bool`
  - `slice::get(&self /* &[T] */, usize) -> Option<&T>`
  - `slice::get_unchecked_mut(&self /* &[T] */, usize) -> &T` (unsafe and
    simplified)
  - `Option::as_ref(&self /* &Option<T> */) -> Option<&T>`
  - `str::from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str` (unsafe)
  - `Rc::get_mut(&mut self /* &mut Rc<T> */) -> Option<&mut T>` (simplified)
  - `Vec::dedup_by_key<K: PartialEq>(&mut self /* &mut Vec<T> */, key: impl FnMut(&mut T) -> K)`
    (simplified)

  Answers for missing names:
  - `fn from_string(String) -> Self`
  - `fn inner(&self) -> Option<&InnerType>` or `as_ref`, depending on context
  - `fn with_string(self, String) -> Self`
  - `fn inner_mut(&mut self) -> Option<&mut InnerType>` or `as_ref_mut`,
    depending on context

</details>
