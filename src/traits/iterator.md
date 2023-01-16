# Iterators

You can implement the `Iterator` trait on your own types:

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

<details>

`IntoIterator` is the trait that makes for loops work. It is implemented by collection types such as
`Vec<T>` and references to them such as `&Vec<T>` and `&[T]`. Ranges also implement it.

</details>
