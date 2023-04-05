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
    let future = count_to(count);
    future.await;
}

fn main() {
    let future = async_main(10);
    block_on(future);
}
```

<details>

Key points:

* What is the return type of an async call?
  * Change `let future = async_main(10);` to `let future: () = async_main(10);`
    to see the type.

* The "async" keyword is syntactic sugar. The compiler replaces the return type. 

* You cannot make `main` async, without additional instructions to the compiler
  on how to use the returned future.

* You need an executor to run async code. `block_on` blocks the current thread
  until the provided future has run to completion. 

* `.await` asynchronously waits for the completion of another operation. Unlike
  `block_on`, `.await` doesn't block the current thread.

* `.await` can only be used inside an `async` block. 

</details>
