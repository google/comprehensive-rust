---
minutes: 30
---

# Example: String interning library

C++ Header: interner.hpp

```cpp
#ifndef INTERNER_HPP
#define INTERNER_HPP

#include <string>
#include <unordered_set>

class StringInterner {
    std::unordered_set<std::string> strings;

public:
    // Returns pointer to interned string (valid for lifetime of interner)
    const char* intern(const char* s) {
        auto [it, _] = strings.emplace(s);
        return it->c_str();
    }

    size_t count() const {
        return strings.size();
    }
};

#endif
```

C header file: interner.h

```c
// interner.h (C API for FFI)
#ifndef INTERNER_H
#define INTERNER_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct StringInterner StringInterner;

StringInterner* interner_new(void);
void interner_free(StringInterner* interner);
const char* interner_intern(StringInterner* interner, const char* s);
size_t interner_count(const StringInterner* interner);

#ifdef __cplusplus
}
#endif
```

C++ implementation (interner.cpp)

```cpp
#include "interner.hpp"
#include "interner.h"

extern "C" {

StringInterner* interner_new(void) {
    return new StringInterner();
}

void interner_free(StringInterner* interner) {
    delete interner;
}

const char* interner_intern(StringInterner* interner, const char* s) {
    return interner->intern(s);
}

size_t interner_count(const StringInterner* interner) {
    return interner->count();
}

}
```

<details>

This is a larger example. Write a wrapper for the string interner. You will need
to guide learners on how to create an opaque pointer, either directly by
explaining the code below or asking learners to do further research.

_Suggested Solution_

```rust
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::c_char;

#[repr(C)]
pub struct StringInternerRaw {
    _opaque: [u8; 0],
    _pin: PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

unsafe extern "C" {
    fn interner_new() -> *mut StringInternerRaw;

    fn interner_free(interner: *mut StringInternerRaw);

    fn interner_intern(
        interner: *mut StringInternerRaw,
        s: *const c_char,
    ) -> *const c_char;

    fn interner_count(interner: *const StringInternerRaw) -> usize;
}
```

Once the raw wrapper is written, ask learners to create a safe wrapper.

</details>
