---
minutes: 5
---

# Trait Method Conflicts

What happens when you have a name conflict between two different trait methods
implemented for the same type?

```rust,compile_fail
mod ext {
    pub trait Ext1 {
        fn is_palindrome(&self) -> bool;
    }

    pub trait Ext2 {
        fn is_palindrome(&self) -> bool;
    }

    impl Ext1 for &str {
        fn is_palindrome(&self) -> bool {
            self.chars().eq(self.chars().rev())
        }
    }

    impl Ext2 for &str {
        fn is_palindrome(&self) -> bool {
            self.chars().eq(self.chars().rev())
        }
    }
}

pub use ext::Ext1;
pub use ext::Ext2;

// Which method is invoked?
// The one from `Ext1`? Or the one from `Ext2`?
assert!("dad".is_palindrome());
```

<details>

- The trait you are extending may, in a newer version, add a new trait method
  with the same name as your extension method. Or another extension trait for
  the same type may define a method with a name that conflicts with your own
  extension method.

  Ask: what will happen in the example above? Will there be a compiler error?
  Will one of the two methods be given higher priority? Which one?

- The compiler rejects the code because it cannot determine which method to
  invoke. Neither `Ext1` nor `Ext2` has a higher priority than the other.

  To resolve this conflict, you must specify which trait you want to use. For
  example, you can call `Ext1::is_palindrome("dad")` or
  `Ext2::is_palindrome("dad")`. Demonstrate this syntax and that the updated
  code compiles.

  For methods with more complex signatures, you may need to use a more explicit
  [fully-qualified syntax][1].

</details>

[1]: https://doc.rust-lang.org/reference/expressions/call-expr.html#disambiguating-function-calls
