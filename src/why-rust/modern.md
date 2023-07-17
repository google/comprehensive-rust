# Modern Features

Rust is built with all the experience gained in the last decades.

## Language Features

* Enums and pattern matching.
* Generics.
* No overhead FFI.
* Zero-cost abstractions.

## Tooling

* Great compiler errors.
* Built-in dependency manager.
* Built-in support for testing.
* Excellent Language Server Protocol support.

<details>

Key points:

* Zero-cost abstractions, similar to C++, means that you don't have to 'pay'
  for higher-level programming constructs with memory or CPU. For example,
  writing a loop using `for` should result in roughly the same low level
  instructions as using the `.iter().fold()` construct.

* It may be worth mentioning that Rust enums are 'Algebraic Data Types', also
  known as 'sum types', which allow the type system to express things like
  `Option<T>` and `Result<T, E>`.

* Remind people to read the errors --- many developers have gotten used to
  ignore lengthy compiler output. The Rust compiler is significantly more
  talkative than other compilers. It will often provide you with _actionable_
  feedback, ready to copy-paste into your code.

* The Rust standard library is small compared to languages like Java, Python,
  and Go. Rust does not come with several things you might consider standard and
  essential:

  * a random number generator, but see [rand].
  * support for SSL or TLS, but see [rusttls].
  * support for JSON, but see [serde_json].

  The reasoning behind this is that functionality in the standard library cannot
  go away, so it has to be very stable. For the examples above, the Rust
  community is still working on finding the best solution --- and perhaps there
  isn't a single "best solution" for some of these things.

  Rust comes with a built-in package manager in the form of Cargo and this makes
  it trivial to download and compile third-party crates. A consequence of this
  is that the standard library can be smaller.

  Discovering good third-party crates can be a problem. Sites like
  <https://lib.rs/> help with this by letting you compare health metrics for
  crates to find a good and trusted one.
  
* [rust-analyzer] is a well supported LSP implementation used in major
  IDEs and text editors.

[rand]: https://docs.rs/rand/
[rusttls]: https://docs.rs/rustls/
[serde_json]: https://docs.rs/serde_json/
[rust-analyzer]: https://rust-analyzer.github.io/

</details>
