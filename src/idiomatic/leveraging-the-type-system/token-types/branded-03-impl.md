---
minutes: 10
---

# Implementing Branded Types (Branding 3/4)

Can't use a type if we can't construct it.

```rust
# use std::marker::PhantomData;
# 
# #[derive(Default)]
# struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);
struct BrandedToken<'id>(InvariantLifetime<'id>);

struct MyStructure<'id>(Vec<u8>, InvariantLifetime<'id>);

impl<'id> MyStructure<'id> {
    fn new<T>(
        // The data we want to modify in this context.
        data: Vec<u8>,
        // We want a function whose lifetime is specific to each call to
        // `new`, not tied to any one data structure other than
        // the function. This hides enough information from the borrow
        // checker that it can no longer "subtype"
        f: impl for<'a> FnOnce(MyStructure<'a>, BrandedToken<'a>) -> T,
    ) -> T {
        f(
            MyStructure(data, InvariantLifetime::default()),
            BrandedToken(InvariantLifetime::default()),
        )
    }
    fn use_token(&mut self, token: &BrandedToken<'id>) {}
}
```

<details>

- The underlying Branded Data Structure we're going to use here is just a
  `Vec<u8>` (the data) and an `InvariantLifetime`.

- The constructor for this type will take **data for the `Vec<u8>`** plus a
  function to manipulate the data constructed by `MyStructure::new`.

- Ask: Does anyone know what the `for <'a>` is for?

  Expect not much, it's "for" in the sense of "forall" from mathematics.

- The `for<'a> [trait bound that uses 'a]` binding of `'a` means the lifetime is
  "self contained."

  That is, the borrow checker's view of the function passed to
  `MyStructure::new` is limited in the sub-typing it can do.

  This limit in the borrow checker's ability to sub-type lifetimes is what lets
  us force a token to only apply to a specific variable-bound value.

  The keyword for those interested is "higher-ranked trait bounds."

- Ask: But how can we then use "new" to return a `MyStructure`?

  Follow this up with the next slide.

</details>
