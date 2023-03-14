# Volatile memory access for MMIO

 * Use `pointer::read_volatile` and `pointer::write_volatile`.
 * Never hold a reference.
 * `addr_of!` lets you get fields of structs without creating an intermediate reference.

<details>

 * Volatile access: read or write operations may have side-effects, so prevent the compiler or
   hardware from reordering, duplicating or eliding them.
   * Usually if you write and then read, e.g. via a mutable reference, the compiler may assume that
     the value read is the same as the value just written, and not bother actually reading memory.
 * Some existing crates for volatile access to hardware do hold references, but this is unsound.
   Whenever a reference exist, the compiler may choose to dereference it.
 * Use the `addr_of!` macro to get struct field pointers from a pointer to the struct.

</details>
