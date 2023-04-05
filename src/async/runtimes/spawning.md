# Spawning

Similar to closures, a snippet of async code can be included inline in another
function with an async block:

```rust,editable,compile_fail
use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(count_to(10));

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}
```

<details>

* The `spawn` function creates a new, concurrent "task", just like spawning a thread.

* An async block is similar to a closure, but does not take any arguments. Its return
  value is a Future, similar to `async fn`.

* With the `tokio::main` macro we can now make `main` async.

**Further exploration:**

* Why does `count_to` not (usually) get to 10? This is an example of async
  cancellation. `tokio::spawn` returns a handle which can be awaited to wait
  until it finishes.

* Try `count_to(10).await` instead of spawning.

* Try importing `tokio::join` and using it to join multiple handles.

</details>
