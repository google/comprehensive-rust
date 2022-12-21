# Fearless Concurrency

Rust has full support for concurrency using OS threads with mutexes and
channels.

The Rust type system plays an important role in making many concurrency bugs
compile time bugs. This is often referred to a _fearless concurrency_ since you
can rely on the compiler to ensure correctness at runtime.
