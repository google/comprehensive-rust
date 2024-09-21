---
minutes: 10
---

# Shared References

A reference provides a way to access another value without taking ownership of
the value, and is also called "borrowing". Shared references are read-only, and
the referenced data cannot change.

<!-- mdbook-xgettext: skip -->

```rust,editable
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}
```

A shared reference to a type `T` has type `&T`. A reference value is made with
the `&` operator. The `*` operator "dereferences" a reference, yielding its
value.

Rust will statically forbid dangling references:

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
fn x_axis(x: &i32) -> &(i32, i32) {
    let point = (*x, 0);
    return &point;
}
```

<details>

- References can never be null in Rust, so null checking is not necessary.

- A reference is said to "borrow" the value it refers to, and this is a good
  model for students not familiar with pointers: code can use the reference to
  access the value, but is still "owned" by the original variable. The course
  will get into more detail on ownership in day 3.

- References are implemented as pointers, and a key advantage is that they can
  be much smaller than the thing they point to. Students familiar with C or C++
  will recognize references as pointers. Later parts of the course will cover
  how Rust prevents the memory-safety bugs that come from using raw pointers.

- Rust does not automatically create references for you - the `&` is always
  required.

- Rust will auto-dereference in some cases, in particular when invoking methods
  (try `r.is_ascii()`). There is no need for an `->` operator like in C++.

- In this example, `r` is mutable so that it can be reassigned (`r = &b`). Note
  that this re-binds `r`, so that it refers to something else. This is different
  from C++, where assignment to a reference changes the referenced value.

- A shared reference does not allow modifying the value it refers to, even if
  that value was mutable. Try `*r = 'X'`.

- Rust is tracking the lifetimes of all references to ensure they live long
  enough. Dangling references cannot occur in safe Rust. `x_axis` would return a
  reference to `point`, but `point` will be deallocated when the function
  returns, so this will not compile.

- We will talk more about borrowing when we get to ownership.

</details>
