# Compile Time Guarantees

Static memory management at compile time:

* No uninitialized variables.
* No memory leaks.
* No double-frees.
* No use-after-free.
* No `NULL` pointers.
* No forgotten locked mutexes.
* No data races between threads.
* No iterator invalidation.
