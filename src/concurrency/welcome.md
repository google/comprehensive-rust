---
course: Concurrency
session: Morning
target_minutes: 180
---

# Welcome to Concurrency in Rust

Rust has full support for concurrency using OS threads with mutexes and
channels.

The Rust type system plays an important role in making many concurrency bugs
compile time errors. This idea is known as _fearless concurrency_ since you can
rely on the compiler to ensure correctness at runtime.

## Schedule

{{%session outline}}

<details>

- Rust lets us access OS concurrency toolkit: threads, sync. primitives, etc.
- The type system gives us safety for concurrency without any special features.
- The same tools that help with "concurrent" access in a single thread (e.g., a
  called function that might mutate an argument or save references to it to read
  later) save us from multi-threading issues.

</details>
