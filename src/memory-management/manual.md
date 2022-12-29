# Manual Memory Management

사용자가 직접 메모리를 할당, 해제 합니다.
> You allocate and deallocate heap memory yourself.

## C Example

`malloc`로 할당하는 포인터마다 `free`를 호출해야 합니다: 
> You must call `free` on every pointer you allocate with `malloc`:

```c
void foo(size_t n) {
    int* int_array = (int*)malloc(n * sizeof(int));
    //
    // ... lots of code
    //
    free(int_array);
}
```

만약`malloc` 과 `free` 사이에서 함수가 먼저 반환되면 메모리 누수가 일어납니다.

: 포인터가 손실되어 메모리 할당을 해제할 수 없게 됩니다.
> Memory is leaked if the function returns early between `malloc` and `free`: the
> pointer is lost and we cannot deallocate the memory.
