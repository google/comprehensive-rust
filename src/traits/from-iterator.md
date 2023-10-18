# FromIterator

[`FromIterator`][1] lets you build a collection from an [`Iterator`][2].

```rust,editable
fn main() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}
```

<details>

`Iterator` implements
`fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>,
    Self: Sized`

There are also implementations which let you do cool things like convert an
`Iterator<Item = Result<V, E>>` into a `Result<Vec<V>, E>`.

</details>

[1]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html
[2]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
