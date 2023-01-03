# `Arc`

[`Arc<T>`][1]는 `clone` 메서드를 통해 읽기전용 접근을 허용합니다:
> [`Arc<T>`][1] allows shared read-only access via its `clone` method:

```rust,editable
use std::thread;
use std::sync::Arc;

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = v.clone();
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}
```


[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html
