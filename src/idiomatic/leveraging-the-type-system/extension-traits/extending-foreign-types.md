---
minutes: 15
---

# Extending Foreign Types

An **extension trait** is a local trait definition whose primary purpose is to
attach new methods to foreign types.

```rust
mod ext {
    pub trait StrExt {
        fn is_palindrome(&self) -> bool;
    }

    impl StrExt for &str {
        fn is_palindrome(&self) -> bool {
            self.chars().eq(self.chars().rev())
        }
    }
}

// Bring the extension trait into scope...
pub use ext::StrExt as _;
// ...then invoke its methods as if they were inherent methods
assert!("dad".is_palindrome());
assert!(!"grandma".is_palindrome());
```

<details>

- The `Ext` suffix is conventionally attached to the name of extension traits.

  It communicates that the trait is primarily used for extension purposes, and
  it is therefore not intended to be implemented outside the crate that defines
  it.

  Refer to the ["Extension Trait" RFC][1] as the authoritative source for naming
  conventions.

- The trait implementation for the chosen foreign type must belong to the same
  crate where the trait is defined, otherwise you'll be blocked by Rust's
  [_orphan rule_][2].

- The extension trait must be in scope when its methods are invoked.

  Comment out the `use` statement in the example to show the compiler error
  that's emitted if you try to invoke an extension method without having the
  corresponding extension trait in scope.

- The example above uses an [_underscore import_][3] (`use ext::StrExt as _`) to
  minimize the likelihood of a naming conflict with other imported traits.

  With an underscore import, the trait is considered to be in scope and you're
  allowed to invoke its methods on types that implement the trait. Its _symbol_,
  instead, is not directly accessible. This prevents you, for example, from
  using that trait in a `where` clause.

  Since extension traits aren't meant to be used in `where` clauses, they are
  conventionally imported via an underscore import.

- Some students may be wondering: does the extension trait pattern provide
  enough value to justify the additional boilerplate? Wouldn't a free function
  be enough?

  Show how the same example could be implemented using an `is_palindrome` free
  function, with a single `&str` input parameter:

  ```rust
  fn is_palindrome(s: &str) -> bool {
      s.chars().eq(s.chars().rev())
  }
  ```

  A bespoke extension trait might be an overkill if you want to add a single
  method to a foreign type. Both a free function and an extension trait will
  require an additional import, and the familiarity of the method calling syntax
  may not be enough to justify the boilerplate of a trait definition.

  Nonetheless, extension methods can be **easier to discover** than free
  functions. In particular, language servers (e.g. `rust-analyzer`) will suggest
  extension methods if you type `.` after an instance of the foreign type.

</details>

[1]: https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html
[2]: https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md#what-is-coherence-and-why-do-we-care
[3]: https://doc.rust-lang.org/stable/reference/items/use-declarations.html#r-items.use.as-underscore
