---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# A C Preprocessor Macro Example

C and C++ macros perform literal, raw text replacement.

This allows developers substantial freedom to write C or C++ programs as
sequences of tokens that look little like normal code before preprocessing:

```cpp
#include <iostream>
#define BEGIN () {
#define END }
#define EXPECT(check) if (idx check) {}
#define OR else
#define DIE PanicFormatter() <<

class PanicFormatter {
public:
    void operator<<(const char *message) {
        std::cerr << message << std::endl; exit(1);
    }
};

int main BEGIN
  for (int idx = 0; idx < 10; idx++) {
    EXPECT(< 10) OR DIE "Uh-Oh!";
  }
END
```

### Key takeaways from this example:

- **Custom Syntax:** Using `BEGIN` and `END` in place of braces is not permitted
  by C itself, and would surprise most readers, but as the preprocessing runs
  before compilation, the compiler just sees the braces to which these macros
  expand.
- **Unhygienic State Access:** The `EXPECT` macro implicitly accesses `idx`,
  which is defined in `main` and not passed into the macro. This makes the
  macro's effect depend on the context in which it is invoked (e.g., suppose
  `idx` was a pointer to a `struct BTreeIndex`).

<details>

- Explain the C preprocessor operates as a pure copy-paste system. It replaces
  tokens line by line without any syntactic structure check.
- Notice how `EXPECT(< 10)` expands to `if (idx < 10) {}`. The macro assumes
  `idx` exists and is accessible.
- If someone uses `EXPECT` inside a function without `idx`, they get a compiler
  error at the _expansion site_, not the macro definition site, which can lead
  to confusing and hard-to-debug compiler errors.

</details>
