---
minutes: 10
---

Mini-exercise

1. What do these names imply they do?
2. What should we name these signatures?

```rust,compile_fail
// What are the types for these methods?
Option::is_some // ?
Slice::get // ?
Slice::get_unchecked_mut // ?
Option::as_ref // ?

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

</details>
