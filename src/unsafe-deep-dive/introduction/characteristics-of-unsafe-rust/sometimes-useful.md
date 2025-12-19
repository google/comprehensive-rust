---
minutes: 5
---

# Unsafe is sometimes useful

Your code can go faster!

```rust,editable
fn iter_sum(xs: &[u64]) -> u64 {
    xs.iter().sum()
}

fn fast_sum(xs: &[u64]) -> u64 {
    let mut acc = 0;
    let mut i = 0;
    unsafe {
        while i < xs.len() {
            acc += *xs.get_unchecked(i);
            i += 1;
        }
    }
    acc
}

fn main() {
    let data: Vec<_> = (0..1_000_000).collect();

    let baseline = iter_sum(&data);
    let unchecked = fast_sum(&data);

    assert_eq!(baseline, unchecked);
}
```

<details>

Code using `unsafe` _might_ be faster.

`fast_sum()` skips skips bounds checks. However, benchmarking is necessary to
validate performance claims. For cases like this, Rust's iterators can usually
elide bounds checks anyway.

Optional: [show identical generated assembly][godbolt] for the two functions.

[godbolt]: https://rust.godbolt.org/z/d48v1Y5aj

</details>
