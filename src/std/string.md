# String

[`String`][1] is the standard heap-allocated growable UTF-8 string buffer:

```rust,editable
fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}", s3.len(),
             s3.chars().count());
}
```

`String` implements [`Deref<Target = str>`][2], which means that you can call all
`str` methods on a `String`.

[1]: https://doc.rust-lang.org/std/string/struct.String.html
[2]: https://doc.rust-lang.org/std/string/struct.String.html#deref-methods-str

<details>

* `String::new` returns a new empty string, use `String::with capacity` when you know how much data you want to push to the string.
* `String::len` returns the size of the `String` in bytes (which can be different from its length in characters).
* `String::chars` returns an iterator over the actual characters. Note that a `char` can be different from what a human will consider a "character" due to [grapheme clusters](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/struct.Graphemes.html).
*  When people refer to strings they could either be talking about `&str` or `String`.  
* When a type implements `Deref<Target = T>`, the compiler will let you transparently call methods from `T`.
    * `String` implements `Deref<Target = str>` which transparently gives it access to `str`'s methods.
    * Write and compare `let s3 = s1.deref();` and  `let s3 = &*s1`;.
* `String` is implemented as a wrapper around a vector of bytes, many of the operations you see supported on vectors are also supported on `String`, but with some extra guarantees.
* Compare the different ways to index a `String` by using `s3[i]` and `s3.chars().nth(i).unwrap()` where `i` is in-bound, out-of-bounds, and "on" the flag Unicode character.

</details>
