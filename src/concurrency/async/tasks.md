---
minutes: 6
---

# Tasks

Rust has a task system, which is a form of lightweight threading.

A task has a single top-level future which the executor polls to make progress.
That future may have one or more nested futures that its `poll` method polls,
corresponding loosely to a call stack. Concurrency within a task is possible by
polling multiple child futures, such as racing a timer and an I/O operation.

```rust,compile_fail
{{#include tasks.rs}}
```

<details>

Copy this example into your prepared `src/main.rs` and run it from there.

Try connecting to it with a TCP connection tool like
[nc](https://www.unix.com/man-page/linux/1/nc/) or
[telnet](https://www.unix.com/man-page/linux/1/telnet/).

- Ask students to visualize what the state of the example server would be with a
  few connected clients. What tasks exist? What are their Futures?

- This is the first time we've seen an `async` block. This is similar to a
  closure, but does not take any arguments. Its return value is a Future,
  similar to an `async fn`.

- Refactor the async block into a function, and improve the error handling using
  `?`.

</details>
