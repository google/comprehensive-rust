---
minutes: 10
---

# Atomic/Task Contexts and Sleep

One of the safety conditions that the Rust type system does not help us establish is freedom from deadlocks.
In the Linux kernel, a related concern is only sleeping in contexts where doing so is allowed.
In particular, code executing in a task context may sleep, but code executing in an atomic context
(for example, within an interrupt handlers, while holding a spinlock, or in an RCU critical section)
may not.

Sleeping in the wrong place may lead to kernel hangs, but in the context of RCU,
it can even threaten memory safety:
if a CPU sleeps in an RCU read-side critical section, it will be mistakenly considered to have exited that critical section,
potentially leading to use-after-free[^1].

Existing C code in the kernel relies on [`might_sleep`](https://elixir.bootlin.com/linux/v6.12.6/source/include/linux/kernel.h#L93)
and similar annotations which facilitate debugging via runtime tracking when `CONFIG_DEBUG_ATOMIC_SLEEP` is enabled.

Because of the need to forbid sleep, it is not sufficient to simply use RAII to model RCU in Rust
as we might intuitively want to do.
We need some additional checking to ensure that while RCU guards exist, no sleeps or context switches
are performed.

# `klint`

The [`klint`](https://rust-for-linux.com/klint) tool performs static analysis on kernel Rust code
and addresses this problem by tracking preemption count across all functions at compile-time.

It does so based on annotations added to our functions that specify:
- the expected range of preemption counts when calling the function
- the adjustment performed to the preemption count after the function returns

If a function is called from a context where preemption count may be outside the function's expectation,
`klint` will emit an error message.

Recursive functions, generics, and function pointers complicate this analysis, so it is not foolproof,
and conditional control flow around also means `klint`'s analysis is approximate.
But this still catches obvious mistakes in straightforward code,
and `klint` is only likely to improve its analyses.

[^1] <https://www.memorysafety.org/blog/gary-guo-klint-rust-tools/>
