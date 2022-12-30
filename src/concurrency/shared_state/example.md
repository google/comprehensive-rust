# Example

Let us see `Arc` and `Mutex` in action:

```rust,editable
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Using an Arc to share among threads, data inside is protected by a Mutex
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));
    let vc = v.clone();
    let handle = thread::spawn(move || {
        vc.lock().unwrap().push(10);
    });
    v.lock().unwrap().push(1000);

    handle.join().unwrap();
    println!("v: {v:?}");
}
```
