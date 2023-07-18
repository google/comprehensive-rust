# Manual Memory Management

You allocate and deallocate heap memory yourself.

If not done with care, this can lead to crashes, bugs, security vulnerabilities, and memory leaks.

## C Example

You must call `free` on every pointer you allocate with `malloc`:

```c
void foo(size_t n) {
    int* int_array = malloc(n * sizeof(int));
    //
    // ... lots of code
    //
    free(int_array);
}
```

Memory is leaked if the function returns early between `malloc` and `free`: the
pointer is lost and we cannot deallocate the memory.
Worse, freeing the pointer twice, or accessing a freed pointer can lead to exploitable security vulnerabilities.
