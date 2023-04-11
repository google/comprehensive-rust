# Async Traits

Async methods in traits are not yet supported in the stable channel ([An experimental feature exists in nightly and should be stabilized in the mid term.](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html))

The crate [async_trait](https://docs.rs/async-trait/latest/async_trait/) provides a workaround through a macro:

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

async fn run_all_sleepers_multiple_times(sleepers: Vec<Box<dyn Sleeper>>, n_times: usize) {
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

* The difficulty with `async trait` is in that the resulting `Future` does not
  have a size known at compile time, because the size of the `Future` depends
  on the implementation. 

* `async_trait` is easy to use, but note that it's using heap allocations to
  achieve this, and solve the unknow size problem above. This heap allocation
  has performance overhead.

* Try creating a new sleeper struct that will sleep for a random amount of time
  and adding it to the Vec.

* Try making the `sleep` call take `&mut self`.

</details>
