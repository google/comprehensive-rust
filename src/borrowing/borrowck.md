---
minutes: 10
---

# Borrow Checking

Rust's _borrow checker_ puts constraints on the ways you can borrow values. For
a given value, at any time:

- You can have one or more shared references to the value, _or_
- You can have exactly one exclusive reference to the value.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}
```

<details>

- Note that the requirement is that conflicting references not _exist_ at the
  same point. It does not matter where the reference is dereferenced.
- The above code does not compile because `a` is borrowed as mutable (through
  `c`) and as immutable (through `b`) at the same time.
- Move the `println!` statement for `b` before the scope that introduces `c` to
  make the code compile.
- After that change, the compiler realizes that `b` is only ever used before the
  new mutable borrow of `a` through `c`. This is a feature of the borrow checker
  called "non-lexical lifetimes".
- The exclusive reference constraint is quite strong. Rust uses it to ensure
  that data races do not occur. Rust also _relies_ on this constraint to
  optimize code. For example, a value behind a shared reference can be safely
  cached in a register for the lifetime of that reference.
- The borrow checker is designed to accommodate many common patterns, such as
  taking exclusive references to different fields in a struct at the same time.
  But, there are some situations where it doesn't quite "get it" and this often
  results in "fighting with the borrow checker."

</details>
