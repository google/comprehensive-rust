# Important Traits

We will now look at some of the most common traits of the Rust standard library:

* [`Iterator`][1] and [`IntoIterator`][2] used in `for` loops,
* [`From`][3] and [`Into`][4] used to convert values,
* [`Read`][5] and [`Write`][6] used for IO,
* [`Add`][7], [`Mul`][8], ... used for operator overloading, and
* [`Drop`][9] used for defining destructors.
* [`Default`][10] used to construct a default instance of a type.

[1]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[2]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[3]: https://doc.rust-lang.org/std/convert/trait.From.html
[4]: https://doc.rust-lang.org/std/convert/trait.Into.html
[5]: https://doc.rust-lang.org/std/io/trait.Read.html
[6]: https://doc.rust-lang.org/std/io/trait.Write.html
[7]: https://doc.rust-lang.org/std/ops/trait.Add.html
[8]: https://doc.rust-lang.org/std/ops/trait.Mul.html
[9]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[10]: https://doc.rust-lang.org/std/default/trait.Default.html
