---
minutes: 5
existing course material:
- ownership.md
- memory-management/rust.md
---

<!-- NOTES:
Ownership and how it ties to destructors
-->
# Ownership

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

* At the end of the scope, the variable is _dropped_ and the data is freed.
* A destructor can run here to free up resources.
* We say that the variable _owns_ the value.
# Memory Management in Rust

Memory management in Rust is a mix:

* Safe and correct like Java, but without a garbage collector.
* Scope-based like C++, but the compiler enforces full adherence.
* A Rust user can choose the right abstraction for the situation, some even have no cost at runtime like C.

Rust achieves this by modeling _ownership_ explicitly.

<details>

* If asked how at this point, you can mention that in Rust this is usually handled by RAII wrapper types such as [Box], [Vec], [Rc], or [Arc]. These encapsulate ownership and memory allocation via various means, and prevent the potential errors in C.

* You may be asked about destructors here, the [Drop] trait is the Rust equivalent.

</details>

[Box]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
