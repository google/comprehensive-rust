# Async Rust

"Async" is a concurrency model where multiple tasks are executed concurrently by
executing each task until it would block, then switching to another task that is
ready to make progress. The model scales to higher concurrency than threads
because the per-task overhead is typically very low and operating systems
provide means of efficiently selecting tasks that can make progress.

## Comparisons

 * Python has a similar model in its `asyncio`. However, its `Future` type is
   callback-based, and not polled. Async Python programs require a "loop",
   similar to an executor in Rust.

 * JavaScript's `Promise` is similar, but again callback-based. The language
   runtime implements the event loop, so many of the details of Promise
   resolution are hidden.
