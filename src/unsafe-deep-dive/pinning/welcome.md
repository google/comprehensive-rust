# Welcome

This segment of the course covers:

- What "pinning" is
- Why it is necessary
- How Rust implements it
- How it interacts with unsafe and FFI

<details>

"Pinning, or holding a value's memory address in a fixed location,is one of the
more challenging concepts in Rust."

"Normally only seen within async code, i.e. [`poll(self: Pin<&mut Self>)`],
pinning has wider applicability."

Some some data structures that are difficult or impossible to write without the
unsafe keyword, including self-referential structs and intrusive data
structures.

FFI with C++ is a prominent use case that's related to this. Rust must assume
that any C++ with a reference might be a self-referential data structure.

"To understand this conflict in more detail, we'll first need to make sure that
we have a strong understanding of Rust's move semantics."

<details>

[poll]: https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll
