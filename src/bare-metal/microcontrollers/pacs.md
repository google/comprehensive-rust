# PACs

```rust,editable,compile_fail
{{#include examples/src/bin/pac.rs:Example}}
```

<details>

* `cortex-m-rt` provides the vector table, among other things.
* If you `cargo install cargo-binutils` then you can run
  `cargo objdump --bin pac -- -d --no-show-raw-insn` to see the resulting binary.

</details>
