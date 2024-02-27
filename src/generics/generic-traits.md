---
Minutes: 5
---

# Generic Traits

Traits can also be generic, just like types and functions. A trait's parameters
get concrete types when it is used.

```rust,editable
trait Sink<T> {
    fn push(&mut self, v: T);
}

struct ConsoleSink;
impl<T: std::fmt::Display> Sink<T> for ConsoleSink {
    fn push(&mut self, v: T) {
        println!("{}", v);
    }
}

fn generate_numbers<S: Sink<u32>>(n: u32, sink: &mut S) {
    for i in 0..n {
        sink.push(i);
    }
}

fn main() {
    generate_numbers(5, &mut ConsoleSink);
}
```

<details>

- Generic traits take types as "input", while associated traits are a kind of
  "output trait.

- Implementations of the trait do not need to cover all possible type
  parameters. Here, `ConsoleSink` only covers types `T` that implement
  `Display`. Add a simple `SumSink` that only takes `u32`'s and sums them:

  ```rust,compile_fail
  struct SumSink(u32);
  impl Sink<u32> for SumSink {
      fn push(&mut self, v: u32) {
          self.0 += v;
      }
  }
  ```

</details>
