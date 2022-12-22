# Compile Time Guarantees

Static memory management at compile time:

* No uninitialized variables.
* No memory leaks[^leaks].
* No double-frees.
* No use-after-free.
* No `NULL` pointers.
* No forgotten locked mutexes.
* No data races between threads.
* No iterator invalidation.

[^leaks]: It is technically possible to produce a memory leak in (safe) Rust. The [`Box::leak`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak) method allows getting a raw reference out of a [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) and dropping the [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) afterwards, without running the destructor. A use of this could be to get runtime-initialized and runtime-sized static variables. It is also possible to leak memory by creating a reference cycle, for example by using [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) and a self-referential type (see [Chapter 15.6 in the Rust Book](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)). Or simply, the [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) function, which makes the compiler "forget" about a value meaning the destructor is never run. There are many other ways to create leaks in safe Rust, but for the purpose of this course, "No memory leaks" should be understood as "Pretty much no *accidental* memory leaks".
