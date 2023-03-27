# Pin

When you await a future, you effectively move the whole stack frame from which you called `.await` to an internal data structure of your executor. If your future has pointers to data on the stack, the addresses might get invalidated. This is extremely unsafe. Therefore, you want to guarantee that the addresses your future point to don't change. That is why we need to `pin` futures. In most cases, you won't have to think about it when using futures from common libraries unless you use `select` in a loop (which is a pretty common use case). If, you implement your own future, you will likely run into this issue.

```rust,editable,compile_fail
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
struct Runner {
    name: String,
}

async fn race_finish_line(mut rcv: Receiver<String>, timeout: Duration) -> Option<Vec<Runner>> {
    let mut performances: Vec<Runner> = Vec::new();
    let timeout_sleep = sleep(timeout);
    // Pinning here allows us to await `timeout_sleep` multiple times.
    tokio::pin!(timeout_sleep);

    loop {
        tokio::select! {
            // Rcv.recv() returns a new future every time, hence it does not need to be pinned.
            name = rcv.recv() => performances.push(Runner { name: name? }),
            _ = timeout_sleep.as_mut() => break
        }
    }
    Some(performances)
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);

    let names_and_time = [
        ("Leo", 9),("Milo", 3),("Luna", 13),("Oliver", 5),("Charlie", 11),
    ];

    let finish_line_future = race_finish_line(receiver, Duration::from_secs(6));

    for (name, duration_secs) in names_and_time {
        let sender = sender.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(duration_secs)).await;
            sender.send(String::from(name)).await.expect("Failed to send runner");
        });
    }

    println!("{:?}", finish_line_future.await.expect("Failed to collect finish line"));
    // [Runner { name: "Milo" }, Runner { name: "Oliver" }]
}
```


<details>

* `tokio::pin!` only works on futures that implement `Unpin`. Other futures need to use `box::pin`.
* Another alternative is to not use `tokio::pin!` at all but spawn another task that will send to a `oneshot` channel after the end of the `sleep` call. 

```rust,editable,compile_fail
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};
use tokio::sync::oneshot;

#[derive(Debug, PartialEq)]
struct Runner {
    name: String,
}

async fn race_finish_line(mut rcv: Receiver<String>, mut timeout: oneshot::Receiver<()>) -> Option<Vec<Runner>> {
    let mut performances: Vec<Runner> = Vec::new();
    loop {
        tokio::select! {
            name = rcv.recv() => performances.push(Runner { name: name? }),
            _ = &mut timeout => break
        }
    }
    Some(performances)
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);
    let (os_sender, os_receiver) = oneshot::channel();

    let names_and_time = [
        ("Leo", 9),("Milo", 3),("Luna", 13),("Oliver", 5),("Charlie", 11),
    ];

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;
        os_sender.send(()).expect("Failed to send oneshot.");
    });
    let finish_line_future = race_finish_line(receiver, os_receiver);

    for (name, duration_secs) in names_and_time {
        let sender = sender.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(duration_secs)).await;
            sender.send(String::from(name)).await.expect("Failed to send runner");
        });
    }

    println!("{:?}", finish_line_future.await.expect("Failed to collect finish line"));
    // [Runner { name: "Milo" }, Runner { name: "Oliver" }]
}
```

</details>
