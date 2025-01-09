---
minutes: 15
---

# The Kernel Rust Safety Model

## Soundness

Safety in normal, userspace Rust is already a subtle topic.
The verification boundary for `unsafe` code is not the unsafe block or even the containing function,
but the privacy boundary of the public interface of the containing module.
And the guarantees that unsafe code can rely on depend on a combination of the semantics of
regular Rust along with the behavior of the underlying compiler, operating system, and hardware.

In kernel Rust, things are even more complicated.
The golden standard for Rust code making use of `unsafe` is that it must be impossible
for any consumer of the code to trigger undefined behavior through safe interfaces.
But there are many parts of the Linux kernel in which we might want to use Rust that cannot be
fully compartmentalized from the rest of the kernel by a safe, water-tight API.

Many tasks performed by the kernel are only understandable outside the model of C or Rust language semantics:
for example, writing to CPU registers that control paging or DMA may alter the meaning of pointers,
but models of language semantics do not include notions of the underlying architecture's paging or memory-management system.
Tools like miri cannot analyze programs that perform low-level operations like these,
and static analysis tools similarly lack models of their effects.
So we're forced to live with a less thorough notion of safety than we might have in userspace Rust.

For now, some kernel components will be suitable for writing fully safe Rust interfaces
(perhaps those with limited interactions with the rest of the system, such as GPIOs),
while others can only offer limited safety.

This is an area where Rust for Linux is pushing the boundaries of
what Rust's paradigm of memory safety can achieve.

## Limitations of Type and Memory Safety

Rust's guarantees of memory safety provide a baseline that can raise our confidence in Rust code
head and shoulders above the status quo writing other low-level languages.
But some desirable properties are difficult or impossible to guarantee through Rust's type system.

For example, because variables can always be dropped, it's difficult to guarantee liveness properties.

Similarly, because Rust type- and borrow-checking are local analyses,
they cannot be used to ensure global properties like lock ordering.

Other tools can perform useful static analyses for Rust code
similar to those that might be performed with standalone C static analysis packages or gcc compiler plugins.
[`Clippy`](https://doc.rust-lang.org/clippy/usage.html) is the most common static analysis tool for Rust code,
but for kernel-specific analyses the [`klint`](https://github.com/Rust-for-Linux/klint) tool also exists.
