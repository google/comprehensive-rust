# Scoped Threads

일반 스레드는 그들의 환경(주 스레드)에서 빌릴수 없습니다:
> Normal threads cannot borrow from their environment:

```rust,editable,compile_fail
use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::spawn(|| {
        println!("Length: {}", s.len());
    });
}
```

하지만, [scoped thread][1]에서는 가능합니다:
However, you can use a [scoped thread][1] for this:

```rust,editable
use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}
```

[1]: https://doc.rust-lang.org/std/thread/fn.scope.html
