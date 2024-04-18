# Async Traits

Async methods in traits are were stabilized only recently, in the 1.75 release.
This required support for using return-position `impl Trait` (RPIT) in traits,
as the desugaring for `async fn` includes `-> impl Future<Output = ...>`.

However, even with the native support today there are some pitfalls around
`async fn` and RPIT in traits:

- Return-position impl Trait captures all in-scope lifetimes (so some patterns
  of borrowing cannot be expressed)

- Traits whose methods use return-position `impl trait` or `async` are not `dyn`
  compatible.

If we do need `dyn` support, the crate
[async_trait](https://docs.rs/async-trait/latest/async_trait/) provides a
workaround through a macro, with some caveats:

```rust,editable,compile_fail
use async_trait::async_trait;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[async_trait]
trait Sleeper {
    async fn sleep(&self);
}

struct FixedSleeper {
    sleep_ms: u64,
}

#[async_trait]
impl Sleeper for FixedSleeper {
    async fn sleep(&self) {
        sleep(Duration::from_millis(self.sleep_ms)).await;
    }
}

async fn run_all_sleepers_multiple_times(
    sleepers: Vec<Box<dyn Sleeper>>,
    n_times: usize,
) {
    for _ in 0..n_times {
        println!("running all sleepers..");
        for sleeper in &sleepers {
            let start = Instant::now();
            sleeper.sleep().await;
            println!("slept for {}ms", start.elapsed().as_millis());
        }
    }
}

#[tokio::main]
async fn main() {
    let sleepers: Vec<Box<dyn Sleeper>> = vec![
        Box::new(FixedSleeper { sleep_ms: 50 }),
        Box::new(FixedSleeper { sleep_ms: 100 }),
    ];
    run_all_sleepers_multiple_times(sleepers, 5).await;
}
```

<details>

- `async_trait` is easy to use, but note that it's using heap allocations to
  achieve this. This heap allocation has performance overhead.

- The challenges in language support for `async trait` are deep Rust and
  probably not worth describing in-depth. Niko Matsakis did a good job of
  explaining them in
  [this post](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)
  if you are interested in digging deeper.

- Try creating a new sleeper struct that will sleep for a random amount of time
  and adding it to the Vec.

</details>
