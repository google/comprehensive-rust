# Example

Let's see `Arc` and `Mutex` in action:

```rust,editable
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));

    thread::scope(|scope| {
        for i in 4..=10 {
            // v and v_clone reference the same Mutex. v_clone will be Sent to
            // the thread spawned below thereby giving all threads access to the
            // same Mutex<Vec<i32>> without introducing any ownership ambiguity.
            let v_clone = v.clone();
            scope.spawn(move || {
                let mut v = v_clone.lock().unwrap();
                v.push(i);
            });
        }
    });

    println!("v: {v:?}");
}

```
