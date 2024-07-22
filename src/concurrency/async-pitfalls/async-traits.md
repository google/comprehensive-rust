---
minutes: 5
---

# Async Traits

Async methods in traits are were stabilized in the 1.75 release (December 2023).
This required support for using return-position `impl Trait` in traits, as the
desugaring for `async fn` includes `-> impl Future<Output = ...>`.

However, even with the native support, there are some pitfalls around
`async fn`:

- Return-position `impl Trait` captures all in-scope lifetimes (so some patterns
  of borrowing cannot be expressed).

- Async traits cannot be used with [trait objects] (`dyn Trait` support).

The [async_trait] crate provides a workaround for `dyn` support through a macro,
with some caveats:

```rust,editable,compile_fail
{{#include async-traits.rs}}
```

<details>

- `async_trait` is easy to use, but note that it's using heap allocations to
  achieve this. This heap allocation has performance overhead.

- The challenges in language support for `async trait` are too deep to describe
  in-depth in this class. See [this blog post] by Niko Matsakis if you are
  interested in digging deeper. See also these keywords:

  - [RPIT]: short for
    [return-position `impl Trait`](../../generics/impl-trait.md).
  - [RPITIT]: short for return-position `impl Trait` in trait (RPIT in trait).

- Try creating a new sleeper struct that will sleep for a random amount of time
  and adding it to the `Vec`.

</details>

[async_trait]: https://docs.rs/async-trait/
[trait objects]: ../../smart-pointers/trait-objects.md
[this blog post]: https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
[RPIT]: https://doc.rust-lang.org/reference/types/impl-trait.html#abstract-return-types
[RPITIT]: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
