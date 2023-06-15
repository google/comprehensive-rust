# Exceptions

AArch64 defines an exception vector table with 16 entries, for 4 types of exceptions (synchronous,
IRQ, FIQ, SError) from 4 states (current EL with SP0, current EL with SPx, lower EL using AArch64,
lower EL using AArch32). We implement this in assembly to save volatile registers to the stack
before calling into Rust code:

```rust,editable,compile_fail
{{#include examples/src/exceptions.rs:exceptions}}
```

<details>

* EL is exception level; all our examples this afternoon run in EL1.
* For simplicity we aren't distinguishing between SP0 and SPx for the current EL exceptions, or
  between AArch32 and AArch64 for the lower EL exceptions.
* For this example we just log the exception and power down, as we don't expect any of them to
  actually happen.
* We can think of exception handlers and our main execution context more or less like different
  threads. [`Send` and `Sync`][1] will control what we can share between them, just like with threads.
  For example, if we want to share some value between exception handlers and the rest of the
  program, and it's `Send` but not `Sync`, then we'll need to wrap it in something like a `Mutex`
  and put it in a static.

</details>

[1]: ../../concurrency/send-sync.md
