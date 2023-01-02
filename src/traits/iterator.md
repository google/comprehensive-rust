# Iterators
`Iterator`를 커스텀 타입에 구현할 수 있습니다: 
> You can implement the `Iterator` trait on your own types:

```rust,editable
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }
}
```
역주
- 이터레이터 트레이트의 next()를 오버로딩한 예제