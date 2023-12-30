---
minutes: 10
---

<!-- NOTES:
Including `&str` as a way of representing a slice of valid utf-8
-->

# String References

We can now understand the two string types in Rust: `&str` is almost like
`&[char]`, but with its data stored in a variable-length encoding (UTF-8).

```rust,editable
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}
```

Rust terminology:

- `&str` an immutable reference to a string slice.
- `String` a mutable string buffer.

<details>

- `&str` introduces a string slice, which is an immutable reference to UTF-8
  encoded string data stored in a block of memory. String literals (`”Hello”`),
  are stored in the program’s binary.

- Rust’s `String` type is a wrapper around a vector of bytes. As with a
  `Vec<T>`, it is owned.

- As with many other types `String::from()` creates a string from a string
  literal; `String::new()` creates a new empty string, to which string data can
  be added using the `push()` and `push_str()` methods.

- The `format!()` macro is a convenient way to generate an owned string from
  dynamic values. It accepts the same format specification as `println!()`.

- You can borrow `&str` slices from `String` via `&` and optionally range
  selection. If you select a byte range that is not aligned to character
  boundaries, the expression will panic. The `chars` iterator iterates over
  characters and is preferred over trying to get character boundaries right.

- For C++ programmers: think of `&str` as `std::string_view` from C++, but the
  one that always points to a valid string in memory. Rust `String` is a rough
  equivalent of `std::string` from C++ (main difference: it can only contain
  UTF-8 encoded bytes and will never use a small-string optimization).

- Byte strings literals allow you to create a `&[u8]` value directly:

  <!-- mdbook-xgettext: skip -->
  ```rust,editable
  fn main() {
      println!("{:?}", b"abc");
      println!("{:?}", &[97, 98, 99]);
  }
  ```

</details>
