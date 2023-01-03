# Example


`Arc`와 `Mutex`의 동작을 살펴봅니다:
> Let us see `Arc` and `Mutex` in action:

```rust,editable,compile_fail
use std::thread;
// use std::sync::{Arc, Mutex};

fn main() {
    let mut v = vec![10, 20, 30];
    let handle = thread::spawn(|| {
        v.push(10);
    });
    v.push(1000);

    handle.join().unwrap();
    println!("v: {v:?}");
}
```

---
역주
- 위 코드를 동작하게 수정해보세요.