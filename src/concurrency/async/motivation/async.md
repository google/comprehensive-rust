# The Async Model

The async programming model allows us to be more efficient in our concurrency by
allowing multiple tasks to run on a single thread by swapping out the active
task when it becomes blocked.

This involves setting up a lightweight runtime and scheduling system that can
manage multiple ongoing tasks without needing to go through the kernel.

The programming language then also provides us with primitives for writing code
that can suspend itself when it reaches a point where it would block.
