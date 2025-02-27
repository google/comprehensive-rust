---
minutes: 15
---

# Exercise: Elevator Events

We will create a data structure to represent an event in an elevator control
system. It is up to you to define the types and functions to construct various
events. Use `#[derive(Debug)]` to allow the types to be formatted with `{:?}`.

This exercise only requires creating and populating data structures so that
`main` runs without errors. The next part of the course will cover getting data
out of these structures.

```rust,editable,should_panic
{{#include exercise.rs:event}}
    // TODO: add required variants
}

{{#include exercise.rs:direction}}

{{#include exercise.rs:car_arrived}}
    todo!()
}

{{#include exercise.rs:car_door_opened}}
    todo!()
}

{{#include exercise.rs:car_door_closed}}
    todo!()
}

{{#include exercise.rs:lobby_call_button_pressed}}
    todo!()
}

{{#include exercise.rs:car_floor_button_pressed}}
    todo!()
}

{{#include exercise.rs:main}}
```

<details>

- If students ask about `#![allow(dead_code)]` at the top of the exercise, it's
  necessary because the only thing we do with the `Event` type is print it out.
  Due to a nuance of how the compiler checks for dead code this causes it to
  think that the code is unused. They can ignore it for the purpose of this
  exercise.

</details>
