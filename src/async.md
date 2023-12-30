---
session: Afternoon
---

# Async Rust

"Async" is a concurrency model where multiple tasks are executed concurrently by
executing each task until it would block, then switching to another task that is
ready to make progress. The model allows running a larger number of tasks on a
limited number of threads. This is because the per-task overhead is typically
very low and operating systems provide primitives for efficiently identifying
I/O that is able to proceed.

Rust's asynchronous operation is based on "futures", which represent work that
may be completed in the future. Futures are "polled" until they signal that they
are complete.

Futures are polled by an async runtime, and several different runtimes are
available.

## Comparisons

- Python has a similar model in its `asyncio`. However, its `Future` type is
  callback-based, and not polled. Async Python programs require a "loop",
  similar to a runtime in Rust.

- JavaScript's `Promise` is similar, but again callback-based. The language
  runtime implements the event loop, so many of the details of Promise
  resolution are hidden.
