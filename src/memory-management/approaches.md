---
minutes: 10
---

# Approaches to Memory Management

Traditionally, languages have fallen into two broad categories:

- Full control via manual memory management: C, C++, Pascal, ...
  - Programmer decides when to allocate or free heap memory.
  - Programmer must determine whether a pointer still points to valid memory.
  - Studies show, programmers make mistakes.
- Full safety via automatic memory management at runtime: Java, Python, Go,
  Haskell, ...
  - A runtime system ensures that memory is not freed until it can no longer be
    referenced.
  - Typically implemented with reference counting or garbage collection.

Rust offers a new mix:

> Full control _and_ safety via compile time enforcement of correct memory
> management.

It does this with an explicit ownership concept.

<details>

This slide is intended to help students coming from other languages to put Rust
in context.

- C must manage heap manually with `malloc` and `free`. Common errors include
  forgetting to call `free`, calling it multiple times for the same pointer, or
  dereferencing a pointer after the memory it points to has been freed.

- C++ has tools like smart pointers (`unique_ptr`, `shared_ptr`) that take
  advantage of language guarantees about calling destructors to ensure memory is
  freed when a function returns. It is still quite easy to mis-use these tools
  and create similar bugs to C.

- Java, Go, and Python rely on the garbage collector to identify memory that is
  no longer reachable and discard it. This guarantees that any pointer can be
  dereferenced, eliminating use-after-free and other classes of bugs. But, GC
  has a runtime cost and is difficult to tune properly.

Rust's ownership and borrowing model can, in many cases, get the performance of
C, with alloc and free operations precisely where they are required -- zero
cost. It also provides tools similar to C++'s smart pointers. When required,
other options such as reference counting are available, and there are even
third-party crates available to support runtime garbage collection (not covered
in this class).

</details>
