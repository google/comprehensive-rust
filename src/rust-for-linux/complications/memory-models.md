---
minutes: 10
---

# Memory Models: LKMM vs. Rust (C11) Memory Model

Memory models are their own complex topic which we will not cover in depth,
but it's important to understand how they relate to the Rust for Linux project.

The Linux Kernel and the Rust language itself use different memory models,
which specify what guarantees are made when different threads interact through
shared memory and low-level synchronization primitives.

- The kernel has its own memory model (Linux Kernel Memory Model or "LKMM").
  
  This is because it predates standardized formal memory models for concurrency
  and needs high performance for synchronization as used in RCU and elsewhere.
- Rust inherits the semantics promised by LLVM - from the C++11 specification
  (and adopted by the C11 spec).
  So Rust essentially uses the C11 MM.
- LKMM relies on orderings provided by address, data, and control dependencies.
- The C11 MM does not provide all of these, so it isn't simple to express the
  LKMM in terms of the C11 MM.
  - LKMM relies on semantics not guaranteed by the C spec but merely by
    compiler behavior.
    
    This means that conforming to the C standard is not sufficient for an
    arbitrary compiler to compile a working kernel.
    In practice, the kernel is only compiled with GCC or Clang, which both
    implement the desired semantics, so this is fine.

Because Rust atomics and Linux kernel atomics do not necessarily provide
the same guarantees, using them together could have very surprising results.

Instead, Kernel Rust should probably re-implement corresponding atomics the
same way the kernel does in C[^1].

  - This should allow Rust for Linux to interoperate with the rest of the
    kernel in an understandable way,
    but could subtly alter the behavior of other crates that use atomics if
    used in the kernel atop kernel atomics.

<details>

See these links for more background:

- <https://rust-for-linux.zulipchat.com/#narrow/channel/288089-General/topic/Status.20of.20the.20Linux-kernel.20memory.20model.20support.20in.20Rust>
- <https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/.E2.9C.94.20Rust.20and.20the.20Linux.20Kernel.20Memory.20Model>
- <https://lwn.net/Articles/967049/>
- <https://lwn.net/Articles/993785/>

[^1]: <https://github.com/rust-lang/unsafe-code-guidelines/issues/348#issuecomment-1221376388>

</details>
