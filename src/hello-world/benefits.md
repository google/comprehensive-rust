---
minutes: 3
---

# Benefits of Rust

Some unique selling points of Rust:

- _Compile time memory safety_ - whole classes of memory bugs are prevented at
  compile time
  - No uninitialized variables.
  - No double-frees.
  - No use-after-free.
  - No `NULL` pointers.
  - No forgotten locked mutexes.
  - No data races between threads.
  - No iterator invalidation.

- _No undefined runtime behavior_ - what a Rust statement does is never left
  unspecified
  - Array access is bounds checked.
  - Integer overflow is defined (panic or wrap-around).

- _Modern language features_ - as expressive and ergonomic as higher-level
  languages
  - Enums and pattern matching.
  - Generics.
  - No overhead FFI.
  - Zero-cost abstractions.
  - Great compiler errors.
  - Built-in dependency manager.
  - Built-in support for testing.
  - Excellent Language Server Protocol support.

<details>

Do not spend much time here. All of these points will be covered in more depth
later.

Make sure to ask the class which languages they have experience with. Depending
on the answer you can highlight different features of Rust:

- Experience with C or C++: Rust eliminates a whole class of _runtime errors_
  via the borrow checker. You get performance like in C and C++, but you don't
  have the memory unsafety issues. In addition, you get a modern language with
  constructs like pattern matching and built-in dependency management.

- Experience with Java, Go, Python, JavaScript...: You get the same memory
  safety as in those languages, plus a similar high-level language feeling. In
  addition you get fast and predictable performance like C and C++ (no garbage
  collector) as well as access to low-level hardware (should you need it)

</details>
