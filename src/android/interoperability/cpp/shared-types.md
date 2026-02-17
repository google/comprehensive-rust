<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Shared Types

```rust,ignore
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../../../../third_party/cxx/book/snippets.rs:shared_types}}
```

<details>

- Only C-like (unit) enums are supported.
- A limited number of traits are supported for `#[derive()]` on shared types.
  Corresponding functionality is also generated for the C++ code, e.g. if you
  derive `Hash` also generates an implementation of `std::hash` for the
  corresponding C++ type.

</details>
