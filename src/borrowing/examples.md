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
    dbg!(elem);
}
```

We can also look at a case where these rules prevent incorrect optimizations:

```rust,editable,compile_fail
fn sum_and_zero(a: &mut i32, b: &mut i32) {
    *a = *a + *b;
    *b = 0;
}

fn main() {
    let mut x = 5;
    sum_and_zero(&mut x, &mut x);
}
```

<details>

- In the first case, modifying the collection by pushing new elements into
  it can potentially invalidate existing references to the collection's elements
  if the collection has to reallocate.

- In the second case, the aliasing rule prevents mis-compilation: The output of
  `sum_and_zero` depends on the ordering of the two operations, which means if
  the compiler swaps the order of these operations (which it's allowed to do) it
  changes the result.

  - The equivalent code in C exhibits undefined behavior, which may result in
    mis-compilation and unexpected behavior, even if it doesn't cause a crash.

  - Rust's aliasing rules provide strong guarantees about how references can
    alias, allowing the compiler to apply optimizations without breaking the
    semantics of your program.

</details>
