---
minutes: 3
existing course material:
- why-rust.md
- why-rust/compile-time.md
- why-rust/runtime.md
- why-rust/modern.md
---

<!-- NOTES:
This section aims to give an overview of features in Rust that set it apart from other languages (e.g safety, modern language features like pattern matching, package ecosystem etc.). We will also mention the borrow checker briefly since it is crucial for safety.
-->
# Benefits of Rust

# Why Rust?

Some unique selling points of Rust:

* Compile time memory safety.
* Lack of undefined runtime behavior.
* Modern language features.

<details>

Make sure to ask the class which languages they have experience with. Depending
on the answer you can highlight different features of Rust:

* Experience with C or C++: Rust eliminates a whole class of _runtime errors_
  via the borrow checker. You get performance like in C and C++, but you don't
  have the memory unsafety issues. In addition, you get a modern language with
  constructs like pattern matching and built-in dependency management.

* Experience with Java, Go, Python, JavaScript...: You get the same memory safety
  as in those languages, plus a similar high-level language feeling. In addition
  you get fast and predictable performance like C and C++ (no garbage collector)
  as well as access to low-level hardware (should you need it)

</details>
# Compile Time Guarantees

Static memory management at compile time:

* No uninitialized variables.
* No memory leaks (_mostly_, see notes).
* No double-frees.
* No use-after-free.
* No `NULL` pointers.
* No forgotten locked mutexes.
* No data races between threads.
* No iterator invalidation.

<details>

It is possible to produce memory leaks in (safe) Rust. Some examples
are:

* You can use [`Box::leak`] to leak a pointer. A use of this could
  be to get runtime-initialized and runtime-sized static variables
* You can use [`std::mem::forget`] to make the compiler "forget" about
  a value (meaning the destructor is never run).
* You can also accidentally create a [reference cycle] with `Rc` or
  `Arc`.
* In fact, some will consider infinitely populating a collection a memory
  leak and Rust does not protect from those.

For the purpose of this course, "No memory leaks" should be understood
as "Pretty much no *accidental* memory leaks".

[`Box::leak`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
[`std::mem::forget`]: https://doc.rust-lang.org/std/mem/fn.forget.html
[reference cycle]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html

</details>
# Runtime Guarantees

No undefined behavior at runtime:

* Array access is bounds checked.
* Integer overflow is defined (panic or wrap-around).

<details>

Key points:

* Integer overflow is defined via the [`overflow-checks`](https://doc.rust-lang.org/rustc/codegen-options/index.html#overflow-checks)
  compile-time flag. If enabled, the program will panic (a controlled
  crash of the program), otherwise you get wrap-around
  semantics. By default, you get panics in debug mode (`cargo build`)
  and wrap-around in release mode (`cargo build --release`).

* Bounds checking cannot be disabled with a compiler flag. It can also
  not be disabled directly with the `unsafe` keyword. However,
  `unsafe` allows you to call functions such as `slice::get_unchecked`
  which does not do bounds checking.

</details>
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
