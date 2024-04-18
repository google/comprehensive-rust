# `async`/`await`

At a high level, async Rust code looks very much like "normal" sequential code:

```rust,editable,compile_fail
use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count is: {i}!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    block_on(async_main(10));
}
```

<details>

Key points:

- Note that this is a simplified example to show the syntax. There is no long
  running operation or any real concurrency in it!

- What is the return type of an async call?
  - Use `let future: () = async_main(10);` in `main` to see the type.

- The "async" keyword is syntactic sugar. The compiler replaces the return type
  with a future.

- You cannot make `main` async, without additional instructions to the compiler
  on how to use the returned future.

- You need an executor to run async code. `block_on` blocks the current thread
  until the provided future has run to completion.

- `.await` asynchronously waits for the completion of another operation. Unlike
  `block_on`, `.await` doesn't block the current thread.

- `.await` can only be used inside an `async` function (or block; these are
  introduced later).

</details>
