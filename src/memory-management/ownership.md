---
minutes: 5
---

# Ownership

All variable bindings have a _scope_ where they are valid and it is an error to
use a variable outside its scope:

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    println!("y: {}", p.1);
}
```

We say that the variable _owns_ the value. Every Rust value has precisely one
owner at all times.

At the end of the scope, the variable is _dropped_ and the data is freed. A
destructor can run here to free up resources.

<details>

Students familiar with garbage-collection implementations will know that a
garbage collector starts with a set of "roots" to find all reachable memory.
Rust's "single owner" principle is a similar idea.

</details>
