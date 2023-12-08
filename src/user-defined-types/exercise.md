---
minutes: 15
---

# Exercise: Elevator Events

We will create a data structure to represent an event in an elevator control
system. It is up to you to define the types and functions to construct various
events. Use `#[derive(Debug)]` to allow the types to be formatted with `{:?}`.

```rust,compile_fail
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

This exercise only requires creating data structures. The next part of the
course will cover getting data out of these structures.
