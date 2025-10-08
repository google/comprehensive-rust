---
minutes: 20
---

# `PhantomData` and Lifetime Subtyping (Branding 2/4)

We need a mechanism to make lifetimes distinct enough from each other to not
compile when we try to compare them.

<!-- dprint-ignore-start -->
```rust,editable
use std::marker::PhantomData;

#[derive(Default)]
struct InvariantLifetime<'id>(PhantomData<&'id ()>); // The main focus

struct Wrapper<'a> { value: u8, invariant: InvariantLifetime<'a> }

fn lifetime_separator<T>(value: u8, f: impl for<'a> FnOnce(Wrapper<'a>) -> T) -> T {
    f(Wrapper { value, invariant: InvariantLifetime::default() })
}

fn compare_lifetimes<'a>(left: Wrapper<'a>, right: Wrapper<'a>) {}

fn main() {
    lifetime_separator(1, |wrapped_1| {
        lifetime_separator(2, |wrapped_2| {
            // We want this to NOT compile
            compare_lifetimes(wrapped_1, wrapped_2);
        });
    });
}
```
<!-- dprint-ignore-end -->

<details>

<!-- TODO: Link back to PhantomData in the borrowck invariants chapter.
- We saw `PhantomData` back in the Borrow Checker Invariants chapter.
-->

- **Goal**: We want two lifetimes that the rust compiler cannot determine if one
  outlives the other.

  We are using `compare_lifetimes` as a compile-time check to see if the
  lifetimes are being subtyped.

- Note: This slide compiles, by the end of this slide it should only compile
  when `subtyped_lifetimes` is commented out.

- There are two important parts of this code:
  - The `impl for<'a>` bound on the closure passed to `lifetime_separator`.
  - The way lifetimes are used in the parameter for `PhantomData`.

- `for<'a> [trait bound]` is a way of introducing a new lifetime variable to a
  trait bound and asking that the trait bound be true for all instances of that
  new lifetime variable.

  This is analogous to a forall (Ɐ) quantifier in mathematics, or the way we
  introduce `<T>` as type variables, but only for lifetimes in trait bounds.

  What it also does is remove some ability of the compiler to make assumptions
  about that specific lifetime, as this `for<'a>` trait bound asks that the
  bound hold true for all possible lifetimes. This makes comparing that bound
  lifetime to other lifetimes slightly more difficult.

  This is a
  [**Higher-ranked trait bound**](https://doc.rust-lang.org/reference/subtyping.html?search=Hiher#r-subtype.higher-ranked).

- We already know `PhantomData`, which we can use to capture unused type or
  lifetime parameters to make them "used."

- Ask: What can we do with `PhantomData`?

  Expect mentions of the Typestate pattern, tying together the lifetimes of
  owned values.

- Ask: In other languages, what is subtyping?

  Expect mentions of Inheritance, being able to use a value of type `B` when a
  asked for a value of type `A` because `B` is a "subtype" of `A`.

- Rust does have Subtyping! But only for lifetimes.

  Ask: If one lifetime is a subtype of another lifetime, what might that mean?

  A lifetime is a "subtype" of another lifetime when it _outlives_ that other
  lifetime.

- The way that lifetimes captured by `PhantomData` behave depends not only on
  where the lifetime "comes from" but on how the reference is defined too.

  The reason this compiles is that the
  [**Variance**](https://doc.rust-lang.org/stable/reference/subtyping.html#r-subtyping.variance)
  of the lifetime captured by `InvariantLifetime` is too lenient.

  <!-- Note: We've been using "invariants" in this module in a specific way, but subtyping introduces _invariant_, _covariant_, and _contravariant_ as specific terms. -->

- Ask: How can we make it more restrictive?

  Expect or demonstrate: Making it `&'id mut ()` instead. This will not be
  enough!

  We need to use a
  [**Variance**](https://doc.rust-lang.org/stable/reference/subtyping.html#r-subtyping.variance)
  on lifetimes where subtyping cannot be inferred except on _identical
  lifetimes_. That is, the only subtype of `'a` the compiler can know is `'a`
  itself.

  Demonstrate: Move from `&'id ()` (covariant in lifetime and type),
  `&'id mut ()` (covariant in lifetime, invariant in type), `*mut &'id mut ()`
  (invariant in lifetime and type), and finally `*mut &'id ()` (invariant in
  lifetime but not type).

  Those last two should not compile, which means we've finally found candidates
  for how to bind lifetimes to `PhantomData` so they can't be compared to one
  another in this context.

- Wrap up: We've introduced ways to stop the compiler from deciding that
  lifetimes are "similar enough" by choosing a Variance for a lifetime captured
  in `PhantomData` that is restrictive enough to prevent this slide from
  compiling.

</details>
