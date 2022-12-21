# Scoped Threads

Normal threads cannot borrow from their environment:

```rust,editable,compile_fail
use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::spawn(|| {
        println!("Length: {}", s.len());
    });
}
```

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
