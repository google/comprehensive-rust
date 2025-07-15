---
minutes: 15
---

# Method Resolution Conflicts

What happens when you have a name conflict between an inherent method and an
extension method?

```rust
mod ext {
    pub trait StrExt {
        fn trim_ascii(&self) -> &str;
    }

    impl StrExt for &str {
        fn trim_ascii(&self) -> &str {
            self.trim_start_matches(|c: char| c.is_ascii_whitespace())
        }
    }
}

pub use ext::StrExt;
// Which `trim_ascii` method is invoked?
// The one from `StrExt`? Or the inherent one from `str`?
assert_eq!(" dad ".trim_ascii(), "dad");
```

<details>

- The foreign type may, in a newer version, add a new inherent method with the
  same name of our extension method.

  Survey the class: what do the students think will happen in the example above?
  Will there be a compiler error? Will one of the two methods be given higher
  priority? Which one?

  Add a `panic!("Extension trait")` in the body of `StrExt::trim_ascii` to
  clarify which method is being invoked.

- [Inherent methods have higher priority than trait methods][1], _if_ they have
  the same name and the **same receiver**, e.g. they both expect `&self` as
  input. The situation becomes more nuanced if the use a **different receiver**,
  e.g., `&mut self` vs `&self`.

  Change the signature of `StrExt::trim_ascii` to
  `fn trim_ascii(&mut self) -> &str` and modify the invocation accordingly:

  ```rust
  assert_eq!((&mut " dad ").trim_ascii(), "dad");
  ```

  Now `StrExt::trim_ascii` is invoked, rather than the inherent method, since
  `&mut self` has a higher priority than `&self`, the one used by the inherent
  method.

  Point the students to the Rust reference for more information on
  [method resolution][2]. An explanation with more extensive examples can be
  found in [an open PR to the Rust reference][3].

- Avoid naming conflicts between extension trait methods and inherent methods.
  Rust's method resolution algorithm is complex and may surprise users of your
  code.

## More to explore

- The interaction between the priority search used by Rust's method resolution
  algorithm and automatic `Deref`ing can be used to emulate
  [specialization][4] on the stable toolchain, primarily in the context of
  macro-generated code. Check out ["Autoref Specialization"][5] for the specific
  details.

</details>

[1]: https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html#r-expr.method.candidate-search
[2]: https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html
[3]: https://github.com/rust-lang/reference/pull/1725
[4]: https://github.com/rust-lang/rust/issues/31844
[5]: https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
