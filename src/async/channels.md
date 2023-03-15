# Async Channels

Multiple Channels crates have support for `async`/`await` read and write. For instance `tokio` channels:


```rust,editable
use tokio::sync::mpsc::{self, Receiver};

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;
    
    while let Some(_) = input.recv().await {
        count += 1;
        println!("Received {count} pings so far.");
    }
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(32);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for _ in 0..10 {
        sender.send(()).await.expect("Failed to send ping.");
    }
    
    std::mem::drop(sender);
    ping_handler_task.await.expect("Something went wrong in ping handler task.");
}
```
