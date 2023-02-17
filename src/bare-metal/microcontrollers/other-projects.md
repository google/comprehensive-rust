# Other projects

 * [RTIC](https://rtic.rs/)
   * "Real-Time Interrupt-driven Concurrency"
   * Shared resource management, message passing, task scheduling, timer queue
 * [Embassy](https://embassy.dev/)
   * `async` executors with priorities, timers, networking, USB
 * [TockOS](https://www.tockos.org/documentation/getting-started)
   * Security-focused RTOS with preemptive scheduling and Memory Protection Unit support
 * [Hubris](https://hubris.oxide.computer/)
   * Microkernel RTOS from Oxide Computer Company with memory protection, unprivileged drivers, IPC
 * [Bindings for FreeRTOS](https://github.com/lobaro/FreeRTOS-rust)
 * Some platforms have `std` implementations, e.g.
   [esp-idf](https://esp-rs.github.io/book/overview/using-the-standard-library.html).

<details>

 * RTIC can be considered either an RTOS or a concurrency framework.
   * It doesn't include any HALs.
   * It uses the Cortex-M NVIC (Nested Virtual Interrupt Controller) for scheduling rather than a
     proper kernel.
   * Cortex-M only.

</details>
