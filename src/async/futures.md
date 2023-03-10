# Futures

[Future](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#37)
is a trait, implemented by objects that represent an operation that may not be
complete yet. A future can be polled, and `poll` returns either
`Poll::Ready(result)` or `Poll::Pending`.

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

An async function returns an `impl Future`, and an async block evaluates to an
`impl Future`. It's also possible (but uncommon) to implement `Future` for your
own types. For example, the `JoinHandle` returns from `tokio::spawn` implements
`Future` to allow joining to it.

```rust,editable
async fn fetch_token() -> String {
	todo!()
}

async fn api_request() {
	let _token_fut: () = fetch_token();
	todo!()
}
```

The `.await` keyword, applied to a Future, causes the current async function or
block to pause until that Future is ready, and then evaluates to its output.

<details>

* These types are conceptually quite simple, and implemented as such in
  `std::task`.

* We will get to `Pin` and `Context` shortly, so don't get into any depth on
  those items yet.

* The second code sample shows a technique for finding the type of a value,
  from the error message.  Try adding `.await` after `fetch_token()` to see
  the error message change.

</details>
