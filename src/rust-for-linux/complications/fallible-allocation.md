---
minutes: 13
---

# Fallible Allocation

Allocation in Rust is assumed to be infallible:

```rust
let x = Box::new(5);
```

In the Linux kernel, memory allocation is much more complex.

```C
void * kmalloc(size_t size, int flags)
```

`flags` is one of `GFP_KERNEL`, `GFP_NOWAIT`, `GFP_ATOMIC`, etc.[^1]

The return value must be checked against `NULL` to see whether allocation
succeeded.

In Rust for Linux, rather than using the infallible allocation APIs provided by
`liballoc`, the kernel library has its own allocation interfaces:

## `KBox`

```rust
let b = KBox::new(24_u64, GFP_KERNEL)?;
assert_eq!(*b, 24_u64);
```

[`KBox::new`](https://rust.docs.kernel.org/kernel/alloc/kbox/struct.Box.html#tymethod.new)
returns a `Result<Self, AllocError>`. Here we propagate this error with the `?`
operator.

## `KVec`

Similarly,
[`KVec`](https://rust.docs.kernel.org/kernel/alloc/kvec/type.KVec.html) presents
a similar API to the standard `Vec`, but where operations that may allocate take
a flags parameter:

```rust
let mut v = KVec::new();
v.push(1, GFP_KERNEL)?;
assert_eq!(&v, &[1]);
```

## `FromIterator`

Because the standard
[`FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)
trait also involves making new collections often involving memory allocation,
the `.collect()` method on iterators is not available in Rust for Linux in its
original form. Work is ongoing to design an equivalent API[^2], but for now we
do without its convenience.

[^1]: <https://docs.kernel.org/core-api/memory-allocation.html>

[^2]: <https://rust-for-linux.zulipchat.com/#narrow/channel/288089-General/topic/flat_map.20collecting.20with.20Kvec>
