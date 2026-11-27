# Async Motivation

- Programs that involve heavy network communication **spend a lot of their time
  waiting.**
- We want to handle **many ongoing tasks concurrently** while being efficient about
  the resources we allocate to each task.
- **Threads are relatively expensive** tools for doing concurrency.
- Async is a paradigm that allows us to run **many concurrent tasks on fewer
  threads** by swapping out the active task when it is waiting on network data.
