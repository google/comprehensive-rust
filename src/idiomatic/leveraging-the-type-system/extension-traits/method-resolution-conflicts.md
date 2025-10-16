---
minutes: 15
---

# Method Resolution Conflicts

What happens when you have a name conflict between an inherent method and an
extension method?

```rust,editable
mod ext {
    pub trait CountOnesExt {
        fn count_ones(&self) -> u32;
    }

    impl CountOnesExt for i32 {
        fn count_ones(&self) -> u32 {
            let value = *self;
            (0..32).filter(|i| ((value >> i) & 1i32) == 1).count() as u32
        }
    }
}
fn main() {
    pub use ext::CountOnesExt;
    // Which `count_ones` method is invoked?
    // The one from `CountOnesExt`? Or the inherent one from `i32`?
    assert_eq!((-1i32).count_ones(), 32);
}
```

<details>

- A foreign type may, in a newer version, add a new inherent method with the
  same name as our extension method.

  Ask: What will happen in the example above? Will there be a compiler error?
  Will one of the two methods be given higher priority? Which one?

  Add a `panic!("Extension trait");` in the body of `CountOnesExt::count_ones`
  to clarify which method is being invoked.

- To prevent users of the Rust language from having to manually specify which
  method to use in all cases, there is a priority ordering system for how
  methods get "picked" first:
  - Immutable (`&self`) first
    - Inherent (method defined in the type's `impl` block) before Trait (method
      added by a trait impl).
  - Mutable (`&mut self`) Second
    - Inherent before Trait.

  If every method with the same name has different mutability and was either
  defined in as an inherent method or trait method, with no overlap, this makes
  the job easy for the compiler.

  This does introduce some ambiguity for the user, who may be confused as to why
  a method they're relying on is not producing expected behavior. Avoid name
  conflicts instead of relying on this mechanism if you can.

  Demonstrate: Change the signature and implementation of
  `CountOnesExt::count_ones` to `fn count_ones(&mut self) -> u32` and modify the
  invocation accordingly:

  ```rust
  assert_eq!((&mut -1i32).count_ones(), 32);
  ```

  `CountOnesExt::count_ones` is invoked, rather than the inherent method, since
  `&mut self` has a higher priority than `&self`, the one used by the inherent
  method.

  If an immutable inherent method and a mutable trait method exist for the same
  type, we can specify which one to use at the call site by using
  `(&<value>).count_ones()` to get the immutable (higher priority) method or
  `(&mut <value>).count_ones()`

  Point the students to the Rust reference for more information on
  [method resolution][2].

- Avoid naming conflicts between extension trait methods and inherent methods.
  Rust's method resolution algorithm is complex and may surprise users of your
  code.

## More to explore

- The interaction between the priority search used by Rust's method resolution
  algorithm and automatic `Deref`ing can be used to emulate [specialization][4]
  on the stable toolchain, primarily in the context of macro-generated code.
  Check out ["Autoref Specialization"][5] for the specific details.

</details>

[1]: https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html#r-expr.method.candidate-search
[2]: https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html
[3]: https://github.com/rust-lang/reference/pull/1725
[4]: https://github.com/rust-lang/rust/issues/31844
[5]: https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
