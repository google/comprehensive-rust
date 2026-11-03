# Thread Overhead

Now let's say we want to be able to handle many such tasks concurrently, maybe
1,000+ tasks at a time. We can do this by spawning a thread for each task,
but doing so has some unavoidable overhead:

- **Memory usage** - Every thread needs enough stack space to handle the worst
  case scenario for stack usage, even if in practice only a small portion of
  that stack space is used at any given time. Also, within the kernel the OS
  needs to allocate resources for tracking and managing active threads.
- **Context switching** - Switching between threads isn't free and requires a
  roundtrip through the kernel, plus work in the kernel to suspend and resume
  threads.
- **Startup latency** - Spawning a thread is somewhat slow, as the kernel needs
  to allocate various resources for managing the thread.
- **Bandwidth** - A thread pool allows avoiding paying the startup cost for each
  task, but we still need at least as many threads as tasks we want to perform
  concurrently.

<details>

- While threads give us a powerful tool for doing concurrency, they're not free
  and impose some unavoidable overhead.

- We'd like to be able to achieve more efficient concurrency:

  - Allocate only as much space as is needed by the task.
  - Faster context switching, i.e. switching between tasks is as cheap as just
    calling a different function.
  - Less startup latency, i.e. just call a new function on an existing thread.
    Thread pools help mitigate the latency problem, but we still need as many
    threads as tasks we want to be able to execute concurrently.
  - Fewer threads, to minimize the overhead we pay for the threads we do use.

</details>
