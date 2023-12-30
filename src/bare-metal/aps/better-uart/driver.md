# Driver

Now let's use the new `Registers` struct in our driver.

```rust,editable,compile_fail
{{#include ../examples/src/pl011.rs:Uart}}
```

<details>

- Note the use of `addr_of!` / `addr_of_mut!` to get pointers to individual
  fields without creating an intermediate reference, which would be unsound.

</details>
