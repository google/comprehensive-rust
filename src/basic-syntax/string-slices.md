# `String` vs `str`

We can now understand the two string types in Rust:

```rust,editable
fn main() {
    let s1: &str = "Hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}
```

Rust terminology:

* `&str` an immutable reference to a string slice.
* `String` a mutable string buffer.

<details>

* `&str` introduces a string slice, which is an immutable reference to UTF-8 encoded string data stored in a block of memory. String literals (`”Hello”`), are stored in the program’s binary.

* Rust’s `String` type is a wrapper around a vector of bytes. As with a `Vec<T>`, it is mutable and owned.

* `String::from` creates a string from a string literal; `String::new` creates a new empty string, to which string data can be added using the `to_string` method.

* The `push_str` method appends a string slice to the string.
    
</details>
