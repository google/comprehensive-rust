# Async Blocks

Similar to closures, a snippet of async code can be included inline in another
function with an async block:

```rust,editable,compile_fail
use tokio::{time, task};

#[tokio::main]
async fn main() {
    let mut joinset = task::JoinSet::new();

    for i in 1..5 {
        joinset.spawn(async move {
            println!("task {i} starting");
            time::sleep(time::Duration::from_millis(i)).await;
            println!("task {i} done");
            format!("hello from task {i}")
        });
    }

    while let Some(res) = joinset.join_next().await {
        let greeting = res.unwrap();
        println!("task joined with result: {greeting}");
    }
}
```

<details>

An async block is similar to a closure, but does not take any arguments.

Its return value is a Future, which is described on the next slide.

</details>
