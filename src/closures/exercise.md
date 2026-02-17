---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Log Filter

Building on the generic logger from this morning, implement a `Filter` that uses
a closure to filter log messages, sending those that pass the filtering
predicate to an inner logger.

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:setup}}

// TODO: Define and implement `Filter`.

{{#include exercise.rs:main}}
```
