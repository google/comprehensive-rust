---
minutes: 3
---

# Recursion

An async function's future type _contains_ the futures for all functions it
calls. This means a recursive async functions are not allowed.

```rust,editable,compile_fail
use futures::executor::block_on;

async fn count_to(n: u32) {
    if n > 0 {
        count_to(n - 1).await;
        println!("{n}");
    }
}

fn main() {
    block_on(count_to(5));
}
```

<details>

This is a quick illustration of how understanding the state machine helps to
understand errors. Recursion would require `CountToFuture` to contain a field of
type `CountToFuture`, which is impossible.

Fix this with `Box::pin(count_to(n-1)).await;`, boxing the future returned from
`count_to`.

</details>
