---
minutes: 5
---

# Multiple Borrows

But what about when there are multiple borrows passed into a function and one
being returned?

```rust,editable,compile_fail
fn multiple(a: &i32, b: &i32) -> &i32 {
    todo!("Return either `a` or `b`")
}

fn main() {
    let mut a = 5;
    let mut b = 10;

    let r = multiple(&a, &b);

    // Which one is still borrowed?
    // Should either mutation be allowed?
    a += 7;
    b += 7;

    dbg!(r);
}
```

<details>

- This code does not compile right now because it is missing lifetime
  annotations. Before we get it to compile, use this opportunity to have
  students to think about which of our argument borrows should be extended by
  the return value.

- We pass two borrows into `multiple` and one is going to come back out, which
  means we will need to extend the borrow of one of the argument lifetimes.
  Which one should be extended? Do we need to see the body of `multiple` to
  figure this out?

- When borrow checking, the compiler doesn't look at the body of `multiple` to
  reason about the borrows flowing out, instead it looks only at the signature
  of the function for borrow analysis.

- In this case there is not enough information to determine if `a` or `b` will
  be borrowed by the returned reference. Show students the compiler errors and
  introduce the lifetime syntax:

  ```rust,ignore
  fn multiple<'a>(a: &'a i32, b: &'a i32) -> &'a i32 { ... }
  ```

</details>
