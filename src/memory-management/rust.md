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
