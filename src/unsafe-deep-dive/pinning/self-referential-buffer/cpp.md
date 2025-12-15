# Motivating Example: C++

```cpp,editable,ignore
class SelfReferentialBuffer {
    char data[1024];
    char* cursor;
    
public:
    SelfReferentialBuffer() = default;
    
    SelfReferentialBuffer(SelfReferentialBuffer&& other) 
        : cursor(data + (other.cursor - other.data))
    {
        std::memcpy(data, other.data, 1024);
    }
};
```

Investigate on [Compiler Explorer](https://godbolt.org/z/ascME6aje)

<details>

The `SelfReferentialBuffer` contains two members, `data` is a kilobyte of memory
and `cursor` is a pointer into the former.

Its move constructor ensures that cursor is updated to the new memory address.

This type can't be expressed easily in Rust.

> Note: `char*` is dated, but exists in legacy codebases and is used here for
> simplicity.
>
> If your class includes experienced C++ developers, consider replacing `char*`
> with `std::byte*`:
>
> ```cpp
> #include <cstddef>
> #include <cstring>
>
> class SelfReferentialBuffer {
>     std::byte data[1024];
>     std::byte* cursor = data;
>     
> public:
>     SelfReferentialBuffer(SelfReferentialBuffer&& other) 
>         : cursor{data + (other.cursor - other.data)}
>     {
>         std::memcpy(data, other.data, 1024);
>     }
> };
> ```

</details>
