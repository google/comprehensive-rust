---
minutes: 3
---

# Borrow Errors

As a concrete example of how these borrowing rules prevent memory errors,
consider the case of modifying a collection while there are references to its
elements:

```rust,editable,compile_fail
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = &vec[2];
    vec.push(6);
    println!("{elem}");
}
```

Similarly, consider the case of iterator invalidation:

```rust,editable,compile_fail
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for elem in &vec {
        vec.push(elem * 2);
    }
}
```

<details>

- In both of these cases, modifying the collection by pushing new elements into
  it can potentially invalidate existing references to the collection's elements
  if the collection has to reallocate.

</details>
