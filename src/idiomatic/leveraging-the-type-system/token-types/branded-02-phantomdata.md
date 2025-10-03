---
minutes: 5
---

# `PhantomData` and Lifetimes (Branding 2/4)

```rust,editable
use std::marker::PhantomData;

#[derive(Default)]
struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);
```

<details>

<!-- TODO: Link back to PhantomData in the borrowck invariants chapter.
- We saw `PhantomData` back in the Borrow Checker Invariants chapter.
-->

- Note: There are two parts to this, the first part is `PhantomData` and
  `for<'a>` quantification of lifetimes.

  How type parameter of `InvariantLifetime`'s internal `PhantomData` relates to
  [Subtyping](https://doc.rust-lang.org/stable/reference/subtyping.html) is what
  forces this "branding" between lifetimes to apply.

  Without it, the compiler would see the lifetimes on the types we're handling
  as "similar enough" (able to be subtyped) and users would be able to use the
  token for one value with a different value.

</details>
