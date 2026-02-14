---
minutes: 20
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Builder Type

In this example, we will implement a complex data type that owns all of its
data. We will use the "builder pattern" to support building a new value
piece-by-piece, using convenience functions.

Fill in the missing pieces.

```rust,should_panic,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:Package}}
{{#include exercise.rs:as_dependency}}
        todo!("1")
    }
}

{{#include exercise.rs:PackageBuilder}}
{{#include exercise.rs:new}}
        todo!("2")
    }

{{#include exercise.rs:version}}

{{#include exercise.rs:authors}}
        todo!("3")
    }

{{#include exercise.rs:dependency}}
        todo!("4")
    }

{{#include exercise.rs:language}}
        todo!("5")
    }

{{#include exercise.rs:build}}

{{#include exercise.rs:main}}
```
