---
minutes: 30
---

# `PhantomData` and Lifetime Subtyping (Branding 2/4)

Idea:

- Use a lifetime as a unique brand for each token.
- Make lifetimes sufficiently distinct so that they don't implicitly convert
  into each other.

<!-- dprint-ignore-start -->
```rust,editable
use std::marker::PhantomData;

#[derive(Default)]
struct InvariantLifetime<'id>(PhantomData<&'id ()>); // The main focus

struct Wrapper<'a> { value: u8, invariant: InvariantLifetime<'a> }

fn lifetime_separator<T>(value: u8, f: impl for<'a> FnOnce(Wrapper<'a>) -> T) -> T {
    f(Wrapper { value, invariant: InvariantLifetime::default() })
}

fn try_coerce_lifetimes<'a>(left: Wrapper<'a>, right: Wrapper<'a>) {}

fn main() {
    lifetime_separator(1, |wrapped_1| {
        lifetime_separator(2, |wrapped_2| {
            // We want this to NOT compile
            try_coerce_lifetimes(wrapped_1, wrapped_2);
        });
    });
}
```
<!-- dprint-ignore-end -->

<details>

<!-- TODO: Link back to PhantomData in the borrowck invariants chapter.
- We saw `PhantomData` back in the Borrow Checker Invariants chapter.
-->

- In Rust, lifetimes can have subtyping relations between one another.

  This kind of relation allows the compiler to determine if one lifetime
  outlives another.

  Determining if a lifetime outlives another also allows us to say _the shortest
  common lifetime is the one that ends first_.

  This is useful in many cases, as it means two different lifetimes can be
  treated as if they were the same in the regions they do overlap.

  This is typically what we want. But here we want to use lifetimes as a way to
  distinguish values so we say that a token only applies to a single variable
  without having to create a newtype for every single variable we declare.

- **Goal**: We want two lifetimes that the Rust compiler cannot determine if one
  outlives the other.

  We are using `try_coerce_lifetimes` as a compile-time check to see if the
  lifetimes have a common shorter lifetime (AKA being subtyped).

- Note: This slide compiles, by the end of this slide it should only compile
  when `subtyped_lifetimes` is commented out.

- There are two important parts of this code:
  - The `impl for<'a>` bound on the closure passed to `lifetime_separator`.
  - The way lifetimes are used in the parameter for `PhantomData`.

## `for<'a>` bound on a Closure

- We are using `for<'a>` as a way of introducing a lifetime generic parameter to
  a function type and asking that the body of the function to work for all
  possible lifetimes.

  What this also does is remove some ability of the compiler to make assumptions
  about that specific lifetime for the function argument, as it must meet Rust's
  borrow checking rules regardless of the "real" lifetime its arguments are
  going to have. The caller is substituting in actual lifetime, the function
  itself cannot.

  This is analogous to a forall (Ɐ) quantifier in mathematics, or the way we
  introduce `<T>` as type variables, but only for lifetimes in trait bounds.

  When we write a function generic over a type `T`, we can't determine that type
  from within the function itself. Even if we call a function
  `fn foo<T, U>(first: T, second: U)` with two arguments of the same type, the
  body of this function cannot determine if `T` and `U` are the same type.

  This also prevents _the API consumer_ from defining a lifetime themselves,
  which would allow them to circumvent the restrictions we want to impose.

## PhantomData and Lifetime Variance

- We already know `PhantomData`, which can introduce a formal no-op usage of an
  otherwise unused type or a lifetime parameter.

- Ask: What can we do with `PhantomData`?

  Expect mentions of the Typestate pattern, tying together the lifetimes of
  owned values.

- Ask: In other languages, what is subtyping?

  Expect mentions of inheritance, being able to use a value of type `B` when a
  asked for a value of type `A` because `B` is a "subtype" of `A`.

- Rust does have Subtyping! But only for lifetimes.

  Ask: If one lifetime is a subtype of another lifetime, what might that mean?

  A lifetime is a "subtype" of another lifetime when it _outlives_ that other
  lifetime.

- The way that lifetimes used by `PhantomData` behave depends not only on where
  the lifetime "comes from" but on how the reference is defined too.

  The reason this compiles is that the
  [**Variance**](https://doc.rust-lang.org/stable/reference/subtyping.html#r-subtyping.variance)
  of the lifetime inside of `InvariantLifetime` is too lenient.

  Note: Do not expect to get students to understand variance entirely here, just
  treat it as a kind of ladder of restrictiveness on the ability of lifetimes to
  establish subtyping relations.

  <!-- Note: We've been using "invariants" in this module in a specific way, but subtyping introduces _invariant_, _covariant_, and _contravariant_ as specific terms. -->

- Ask: How can we make it more restrictive? How do we make a reference type more
  restrictive in Rust?

  Expect or demonstrate: Making it `&'id mut ()` instead. This will not be
  enough!

  We need to use a
  [**Variance**](https://doc.rust-lang.org/stable/reference/subtyping.html#r-subtyping.variance)
  on lifetimes where subtyping cannot be inferred except on _identical
  lifetimes_. That is, the only subtype of `'a` the compiler can know is `'a`
  itself.

  Note: Again, do not try to get the whole class to understand variance. Treat
  it as a ladder of restrictiveness for now.

  Demonstrate: Move from `&'id ()` (covariant in lifetime and type),
  `&'id mut ()` (covariant in lifetime, invariant in type), `*mut &'id mut ()`
  (invariant in lifetime and type), and finally `*mut &'id ()` (invariant in
  lifetime but not type).

  Those last two should not compile, which means we've finally found candidates
  for how to bind lifetimes to `PhantomData` so they can't be compared to one
  another in this context.

  Reason: `*mut` means
  [mutable raw pointer](https://doc.rust-lang.org/reference/types/pointer.html#r-type.pointer.raw).
  Rust has mutable pointers! But you cannot reason about them in safe Rust.
  Making this a mutable raw pointer to a reference that has a lifetime
  complicates the compiler's ability subtype because it cannot reason about
  mutable raw pointers within the borrow checker.

- Wrap up: We've introduced ways to stop the compiler from deciding that
  lifetimes are "similar enough" by choosing a Variance for a lifetime in
  `PhantomData` that is restrictive enough to prevent this slide from compiling.

  That is, we can now create variables that can exist in the same scope as each
  other, but whose types are automatically made different from one another
  per-variable without much boilerplate.

## More to Explore

- The `for<'a>` quantifier is not just for function types. It is a
  [**Higher-ranked trait bound**](https://doc.rust-lang.org/reference/subtyping.html?search=Hiher#r-subtype.higher-ranked).

</details>
