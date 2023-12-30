---
minutes: 2
---

# Clone

Sometimes you _want_ to make a copy of a value. The `Clone` trait accomplishes
this.

```rust,editable
#[derive(Default)]
struct Backends {
    hostnames: Vec<String>,
    weights: Vec<f64>,
}

impl Backends {
    fn set_hostnames(&mut self, hostnames: &Vec<String>) {
        self.hostnames = hostnames.clone();
        self.weights = hostnames.iter().map(|_| 1.0).collect();
    }
}
```

<details>

The idea of `Clone` is to make it easy to spot where heap allocations are
occurring. Look for `.clone()` and a few others like `Vec::new` or `Box::new`.

It's common to "clone your way out" of problems with the borrow checker, and
return later to try to optimize those clones away.

</details>
