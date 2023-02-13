# The type state pattern

```rust,editable,compile_fail
{{#include examples/src/bin/typestate.rs:Example}}
```

<details>

 * Pins don't implement `Copy` or `Clone`, so only one instance of each can exist. Once a pin is
   moved out of the port struct nobody else can take it.
 * Changing the configuration of a pin consumes the old pin instance, so you can’t keep use the old
   instance afterwards.
 * The type of a value indicates the state that it is in: e.g. in this case, the configuration state
   of a GPIO pin. This encodes the state machine into the type system, and ensures that you don't
   try to use a pin in a certain way without properly configuring it first. Illegal state
   transitions are caught at compile time.
 * You can call `is_high` on an input pin and `set_high` on an output pin, but not vice-versa.
 * Many HAL crates follow this pattern.

</details>
