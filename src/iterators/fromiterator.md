---
minutes: 5
---

# FromIterator

[`FromIterator`][1] lets you build a collection from an [`Iterator`][2].

```rust,editable
fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}
```

<details>

`Iterator` implements

```rust,ignore
fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized
```

There are two ways to specify `B` for this method:

- With the "turbofish": `some_iterator.collect::<COLLECTION_TYPE>()`, as shown.
  The `_` shorthand used here lets Rust infer the type of the `Vec` elements.
- With type inference: `let prime_squares: Vec<_> = some_iterator.collect()`.
  Rewrite the example to use this form.

There are basic implementations of `FromIterator` for `Vec`, `HashMap`, etc.
There are also more specialized implementations which let you do cool things
like convert an `Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`.

</details>

[1]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html
[2]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
