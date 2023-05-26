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
