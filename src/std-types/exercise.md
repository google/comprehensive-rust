---
minutes: 5
---

# Exercise: Counter

In this exercise you will build a very simple data structure that counts the
number of times values are seen.

Use a
[`std::collections::HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html)
to implement your `Counter` type.

If you finish early, try using the
[`entry`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry)
method to halve the number of hash lookups required to implement the `count`
method.

```rust,compile_fail
{{#include exercise.rs:Counter}}
    // ...
}

impl<T> Counter<T> {
    {{#include exercise.rs:new}}
        todo!()
    }

    {{#include exercise.rs:count}}
        todo!()
    }

    {{#include exercise.rs:times_seen}}
        todo!()
    }
}

{{#include exercise.rs:main}}
```
