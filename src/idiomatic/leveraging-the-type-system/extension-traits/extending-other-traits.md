---
minutes: 15
---

# Extending Other Traits

As with types, it may be desirable to **extend foreign traits**. In particular,
to attach new methods to _all_ implementors of a given trait.

```rust
mod ext {
    use std::fmt::Display;

    pub trait DisplayExt {
        fn quoted(&self) -> String;
    }

    impl<T: Display> DisplayExt for T {
        fn quoted(&self) -> String {
            format!("'{}'", self)
        }
    }
}

pub use ext::DisplayExt as _;

assert_eq!("dad".quoted(), "'dad'");
assert_eq!(4.quoted(), "'4'");
assert_eq!(true.quoted(), "'true'");
```

<details>

- Highlight how we added new behavior to _multiple_ types at once. `.quoted()`
  can be called on string slices, numbers, and booleans since they all implement
  the `Display` trait.

  This flavor of the extension trait pattern uses
  [_blanket implementations_][1].

  A blanket implementation implements a trait for all types `T` that satisfy the
  trait bounds specified in the `impl` block. In this case, the only requirement
  is that `T` implements the `Display` trait.

- Draw the students' attention to the implementation of `DisplayExt::quoted`: we
  can't make any assumptions about `T` other than that it implements `Display`.
  All our logic must either use methods from `Display` or functions/macros that
  don't require other traits.

  For example, we can call `format!` with `T`, but can't call `.to_uppercase()`
  because it is not necessarily a `String`.

  We could introduce additional trait bounds on `T`, but it would restrict the
  set of types that can leverage the extension trait.

- Conventionally, the extension trait is named after the trait it extends,
  followed by the `Ext` suffix. In the example above, `DisplayExt`.

- There are entire crates that extend standard library traits with new
  functionality.

  - `itertools` crate provides the `Itertools` trait that extends `Iterator`. It
    adds many iterator adapters, such as `interleave` and `unique`. It provides
    new algorithmic building blocks for iterator pipelines built with method
    chaining.

  - `futures` crate provides the `FutureExt` trait, which extends the `Future`
    trait with new combinators and helper methods.

## More To Explore

- Extension traits can be used by libraries to distinguish between stable and
  experimental methods.

  Stable methods are part of the trait definition.

  Experimental methods are provided via an extension trait defined in a
  different library, with a less restrictive stability policy. Some utility
  methods are then "promoted" to the core trait definition once they have been
  proven useful and their design has been refined.

- Extension traits can be used to split a [dyn-incompatible trait][2] in two:

  - A **dyn-compatible core**, restricted to the methods that satisfy
    dyn-compatibility requirements.
  - An **extension trait**, containing the remaining methods that are not
    dyn-compatible (e.g., methods with a generic parameter).

- Concrete types that implement the core trait will be able to invoke all
  methods, thanks to the blanket impl for the extension trait. Trait objects
  (`dyn CoreTrait`) will be able to invoke all methods on the core trait as well
  as those on the extension trait that don't require `Self: Sized`.

</details>

[1]: https://doc.rust-lang.org/stable/reference/glossary.html#blanket-implementation
[`itertools`]: https://docs.rs/itertools/latest/itertools/
[`Itertools`]: https://docs.rs/itertools/latest/itertools/trait.Itertools.html
[`futures`]: https://docs.rs/futures/latest/futures/
[`FutureExt`]: https://docs.rs/futures/latest/futures/future/trait.FutureExt.html
[`Future`]: https://docs.rs/futures/latest/futures/future/trait.Future.html
[2]: https://doc.rust-lang.org/reference/items/traits.html#r-items.traits.dyn-compatible
