# Data Structures

Some families of data structures, are impossible to create in safe Rust.

- graphs
- bit stuffing
- self-referential types
- intrusive data structures

<details>

Graphs: General-purpose graphs cannot be created as they may need to represent
cycles. Cycles are impossible for the type system to reason about.

Bit stuffing: Overloading bits with multiple meanings, such as the NaN bits in
`f64` for some other purpose or higher-order bits on `x86_64` platforms,

Self-referential types are too hard for the borrow checker to verify. (note to
self: citation needed)

</details>
