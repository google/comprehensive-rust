# With C++

The [CXX crate][1] makes it possible to do safe interoperability between Rust
and C++.

The overall approach looks like this:

<img src="cpp/overview.svg">

See the [CXX tutorial][2] for an full example of using this.

<details>

- At this point, the instructor should switch to the [CXX tutorial][2].

- Walk the students through the tutorial step by step.

- Highlight how CXX presents a clean interface without unsafe code in _both languages_.

- Show the correspondence between [Rust and C++ types](https://cxx.rs/bindings.html):

    - Explain how a Rust `String` cannot map to a C++ `std::string`
      (the latter does not uphold the UTF-8 invariant). Show that
      despite being different types, `rust::String` in C++ can be
      easily constructed from a C++ `std::string`, making it very
      ergonomic to use.

    - Explain that a Rust function returning `Result<T, E>` becomes a
      function which throws a `E` exception in C++ (and vice versa).

</details>

[1]: https://cxx.rs/
[2]: https://cxx.rs/tutorial.html
