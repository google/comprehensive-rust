---
minutes: 15
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Method Resolution Conflicts

What happens when you have a name conflict between an inherent method and an
extension method?

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
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

- When an inherent method and a trait method have the same name and receiver
  type, Rust's [method resolution][2] rules will choose one automatically.
  Generally it will prioritize the inherent method over the trait method, but
  the method resolution rules can be subtle and surprising.

  Demonstrate: Change the call to `(&-1i32).count_ones()` and demonstrate that
  the trait method is now called instead. Then change the trait method to take
  `self` (instead of `&self`) and show that the inherent method is once again
  called.

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
