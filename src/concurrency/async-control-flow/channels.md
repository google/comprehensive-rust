---
minutes: 8
---

# Async Channels

Several crates have support for asynchronous channels. For instance `tokio`:

```rust,editable,compile_fail
{{#include channels.rs}}
```

<details>

- Change the channel size to `3` and see how it affects the execution.

- Overall, the interface is similar to the `sync` channels as seen in the
  [morning class](../channels.md).

- Try removing the `std::mem::drop` call. What happens? Why?

- The [Flume](https://docs.rs/flume/latest/flume/) crate has channels that
  implement both `sync` and `async` `send` and `recv`. This can be convenient
  for complex applications with both IO and heavy CPU processing tasks.

- What makes working with `async` channels preferable is the ability to combine
  them with other `future`s to combine them and create complex control flow.

</details>
