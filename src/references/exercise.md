---
minutes: 15
existing course material:
- exercises/day-3/points-polygons.md
---

<!-- NOTES:
A few utility functions like dot product, magnitude, normalize
-->
# Exercise: Geometry

# Polygon Struct

We will create a `Polygon` struct which contain some points. Copy the code below
to <https://play.rust-lang.org/> and fill in the missing methods to make the
tests pass:

```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include exercise.rs:Point}}
    // add fields
}

{{#include exercise.rs:Point-impl}}
    // add methods
}

{{#include exercise.rs:Polygon}}
    // add fields
}

{{#include exercise.rs:Polygon-impl}}
    // add methods
}

{{#include exercise.rs:Circle}}
    // add fields
}

{{#include exercise.rs:Circle-impl}}
    // add methods
}

{{#include exercise.rs:Shape}}

{{#include exercise.rs:unit-tests}}

#[allow(dead_code)]
fn main() {}
```

<details>

Since the method signatures are missing from the problem statements, the key part
of the exercise is to specify those correctly. You don't have to modify the tests.

Other interesting parts of the exercise:

* Derive a `Copy` trait for some structs, as in tests the methods sometimes don't borrow their arguments.
* Discover that `Add` trait must be implemented for two objects to be addable via "+". Note that we do not discuss generics until Day 3.

</details>
