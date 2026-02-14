---
minutes: 15
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Modelled in C++

```cpp,editable,ignore
#include <cstddef>
#include <cstring>

class SelfReferentialBuffer {
    std::byte data[1024];
    std::byte* cursor = data;
    
public:
    SelfReferentialBuffer(SelfReferentialBuffer&& other) 
        : cursor{data + (other.cursor - other.data)}
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

</details>
