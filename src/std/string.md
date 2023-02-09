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

* `new` heap allocated buffer. `StringL::with capacity` is used when you know how much you want to push on
* `len` returns the size of the `String` in bytes, not its length in characters.
* `chars` returns an iterator over the actual characters.
*  When people refer to strings they could either be talking about `&str` or `String`. 
* Implementing Deref (trait), gives the compiler ability to take a value of any type, call the deref method, and know how to dereference it  
    * `String` implements `Deref<Target = str>` which transparently gives it access to `str`'s methods.
    * Write and compare `let s3 = s1.deref();` and  `let s3 = &*s1`;.
* `String` is implemented as a wrapper around a vector of bytes, many of the operations you see supported on vectors are also supported on `String`, but with some extra guarantees.
* Show the danger of indexing Strings by (1) adding a unicode character to an above string (Ex. `Hβello`) and slicing it at various locations and (2) unwraping the characters `.chars.nth(i).unwrap()` where `i` is in-bound and out-of-bounds given the number of chars. 

</details>
