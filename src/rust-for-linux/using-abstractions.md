---
minutes: 12
---

# Using Abstractions

Now that we've seen a trivial driver, let's look at a real one for the
[`Asix ax88796b network PHY`](https://github.com/Rust-for-Linux/linux/blob/rust-next/drivers/net/phy/ax88796b_rust.rs).

Here we'll see what it looks like to register a driver for a particular
subsystem and implement the needed functionality using abstractions from a
subsystem.
