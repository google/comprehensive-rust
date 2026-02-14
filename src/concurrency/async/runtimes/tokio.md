<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Tokio

Tokio provides:

- A multi-threaded runtime for executing asynchronous code.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

```rust,editable,compile_fail
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use tokio::time;

async fn count_to(count: i32) {
    for i in 0..count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(count_to(10));

    for i in 0..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}
```

<details>

- With the `tokio::main` macro we can now make `main` async.

- The `spawn` function creates a new, concurrent "task".

- Note: `spawn` takes a `Future`, you don't call `.await` on `count_to`.

**Further exploration:**

- Why does `count_to` not get to 10? This is an example of async cancellation.
  `tokio::spawn` returns a handle which can be awaited to wait until it
  finishes.

- Try `count_to(10).await` instead of spawning.

- Try awaiting the task returned from `tokio::spawn`.

</details>
