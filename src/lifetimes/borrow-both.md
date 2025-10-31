---
minutes: 5
---

# Borrow Both

In this case, we have a function where either `a` or `b` may be returned. In
this case we use the lifetime annotations to tell the compiler that both borrows
may flow into the return value.

```rust
fn pick<'a>(c: bool, a: &'a i32, b: &'a i32) -> &'a i32 {
    if c { a } else { b }
}

fn main() {
    let mut a = 5;
    let mut b = 10;

    let r = pick(true, &a, &b);

    // Which one is still borrowed?
    // Should either mutation be allowed?
    // a += 7;
    // b += 7;

    dbg!(r);
}
```

<details>

- The `pick` function will return either `a` or `b` depending on the value of
  `c`, which means we can't know at compile time which one will be returned.

- To express this to the compiler, we use the same lifetime for both `a` and
  `b`, along with the return type. This means that the returned reference will
  borrow BOTH `a` and `b`!

- Uncomment both of the commented lines and show that `r` is borrowing both `a`
  and `b`, even though at runtime it will only point to one of them.

- Change the first argument to `pick` to show that the result is the same
  regardless of if `a` or `b` is returned.

</details>
