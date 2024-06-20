---
minutes: 4
---

# Join

A join operation waits until all of a set of futures are ready, and returns a
collection of their results. This is similar to `Promise.all` in JavaScript or
`asyncio.gather` in Python.

```rust,editable,compile_fail
{{#include join.rs}}
```

<details>

Copy this example into your prepared `src/main.rs` and run it from there.

- For multiple futures of disjoint types, you can use `std::future::join!` but
  you must know how many futures you will have at compile time. This is
  currently in the `futures` crate, soon to be stabilised in `std::future`.

- The risk of `join` is that one of the futures may never resolve, this would
  cause your program to stall.

- You can also combine `join_all` with `join!` for instance to join all requests
  to an http service as well as a database query. Try adding a
  `tokio::time::sleep` to the future, using `futures::join!`. This is not a
  timeout (that requires `select!`, explained in the next chapter), but
  demonstrates `join!`.

</details>
