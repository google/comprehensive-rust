---
course: Concurrency
session: Morning
---

# Welcome to Concurrency in Rust

Rust has full support for concurrency using OS threads with mutexes and
channels.

The Rust type system plays an important role in making many concurrency bugs
compile time bugs. This is often referred to as _fearless concurrency_ since you
can rely on the compiler to ensure correctness at runtime.

<details>

- Rust lets us access OS concurrency toolkit: threads, sync. primitives, etc.
- The type system gives us safety for concurrency without any special features.
- The same tools that help with "concurrent" access in a single thread (e.g., a
  called function that might mutate an argument or save references to it to read
  later) save us from multi-threading issues.

</details>
