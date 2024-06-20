---
minutes: 20
---

# `Pin`

Async blocks and functions return types implementing the `Future` trait. The
type returned is the result of a compiler transformation which turns local
variables into data stored inside the future.

Some of those variables can hold pointers to other local variables. Because of
that, the future should never be moved to a different memory location, as it
would invalidate those pointers.

To prevent moving the future type in memory, it can only be polled through a
pinned pointer. `Pin` is a wrapper around a reference that disallows all
operations that would move the instance it points to into a different memory
location.

```rust,editable,compile_fail
{{#include pin.rs}}
```

<details>

- You may recognize this as an example of the actor pattern. Actors typically
  call `select!` in a loop.

- This serves as a summation of a few of the previous lessons, so take your time
  with it.

  - Naively add a `_ = sleep(Duration::from_millis(100)) => { println!(..) }` to
    the `select!`. This will never execute. Why?

  - Instead, add a `timeout_fut` containing that future outside of the `loop`:

    ```rust,compile_fail
    let timeout_fut = sleep(Duration::from_millis(100));
    loop {
        select! {
            ..,
            _ = timeout_fut => { println!(..); },
        }
    }
    ```
  - This still doesn't work. Follow the compiler errors, adding `&mut` to the
    `timeout_fut` in the `select!` to work around the move, then using
    `Box::pin`:

    ```rust,compile_fail
    let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
    loop {
        select! {
            ..,
            _ = &mut timeout_fut => { println!(..); },
        }
    }
    ```

  - This compiles, but once the timeout expires it is `Poll::Ready` on every
    iteration (a fused future would help with this). Update to reset
    `timeout_fut` every time it expires:
    ```rust,compile_fail
    let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
    loop {
        select! {
            _ = &mut timeout_fut => {
                println!(..);
                timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
            },
        }
    }
    ```

- Box allocates on the heap. In some cases, `std::pin::pin!` (only recently
  stabilized, with older code often using `tokio::pin!`) is also an option, but
  that is difficult to use for a future that is reassigned.

- Another alternative is to not use `pin` at all but spawn another task that
  will send to a `oneshot` channel every 100ms.

- Data that contains pointers to itself is called self-referential. Normally,
  the Rust borrow checker would prevent self-referential data from being moved,
  as the references cannot outlive the data they point to. However, the code
  transformation for async blocks and functions is not verified by the borrow
  checker.

- `Pin` is a wrapper around a reference. An object cannot be moved from its
  place using a pinned pointer. However, it can still be moved through an
  unpinned pointer.

- The `poll` method of the `Future` trait uses `Pin<&mut Self>` instead of
  `&mut Self` to refer to the instance. That's why it can only be called on a
  pinned pointer.

</details>
