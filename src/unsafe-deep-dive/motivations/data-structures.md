# Data Structures

Some families of data structures are impossible to create in safe Rust.

- graphs
- bit twiddling
- self-referential types
- intrusive data structures

<details>

Graphs: General-purpose graphs cannot be created as they may need to represent
cycles. Cycles are impossible for the type system to reason about.

Bit twiddling: Overloading bits with multiple meanings. Examples include using
the NaN bits in `f64` for some other purpose or the higher-order bits of
pointers on `x86_64` platforms. This is somewhat common when writing language
interpreters to keep representations within the word size the target platform.

Self-referential types are too hard for the borrow checker to verify. (note to
self: citation needed)

</details>
