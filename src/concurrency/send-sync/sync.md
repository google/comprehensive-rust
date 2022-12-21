# `Sync`

> A type `T` is [`Sync`][1] if it is safe to access a `T` value from multiple
> threads at the same time.

More precisely, the definition is:

> `T` is `Sync` if and only if `&T` is `Send`

[1]: https://doc.rust-lang.org/std/marker/trait.Sync.html
