---
minutes: 10
---

# Borrow Checking

Rust's _borrow checker_ puts constraints on the ways you can borrow values.
We've already seen that a reference cannot _outlive_ the value it borrows:

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    dbg!(x_ref);
}
```

There's also a second main rule that the borrow checker enforces: The _aliasing_
rule. For a given value, at any time:

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

    dbg!(a);
    dbg!(b);
}
```

<details>

- The "outlives" rule was demonstrated previously when we first looked at
  references. We review it here to show students that the borrow checking is
  following a few different rules to validate borrowing.
- The above code does not compile because `a` is borrowed as mutable (through
  `c`) and as immutable (through `b`) at the same time.
  - Note that the requirement is that conflicting references not _exist_ at the
    same point. It does not matter where the reference is dereferenced. Try
    commenting out `*c = 20` and show that the compiler error still occurs even
    if we never use `c`.
  - Note that the intermediate reference `c` isn't necessary to trigger a borrow
    conflict. Replace `c` with a direct mutation of `a` and demonstrate that
    this produces a similar error. This is because direct mutation of a value
    effectively creates a temporary mutable reference.
- Move the `dbg!` statement for `b` before the scope that introduces `c` to make
  the code compile.
  - After that change, the compiler realizes that `b` is only ever used before
    the new mutable borrow of `a` through `c`. This is a feature of the borrow
    checker called "non-lexical lifetimes".

## More to Explore

- Technically multiple mutable references to a piece of data can exist at the
  same time via re-borrowing. This is what allows you to pass a mutable
  reference into a function without invaliding the original reference.
  [This playground example][1] demonstrates that behavior.
- Rust uses the exclusive reference constraint to ensure that data races do not
  occur in multi-threaded code, since only one thread can have mutable access to
  a piece of data at a time.
- Rust also uses this constraint to optimize code. For example, a value behind a
  shared reference can be safely cached in a register for the lifetime of that
  reference.
- Fields of a struct can be borrowed independently of each other, but calling a
  method on a struct will borrow the whole struct, potentially invalidating
  references to individual fields. See [this playground snippet][2] for an
  example of this.

</details>

[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=8f5896878611566845fe3b0f4dc5af68
[2]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=f293a31f2d4d0d31770486247c2e8437
