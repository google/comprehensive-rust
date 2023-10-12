# Board support crates

Board support crates provide a further level of wrapping for a specific board for convenience.
<!-- mdbook-xgettext: skip -->
```rust,editable,compile_fail
{{#include examples/src/bin/board_support.rs:Example}}
```

<details>

 * In this case the board support crate is just providing more useful names, and a bit of
   initialisation.
 * The crate may also include drivers for some on-board devices outside of the microcontroller
   itself.
   * `microbit-v2` includes a simple driver for the LED matrix.

Run the example with:

```sh
cargo embed --bin board_support
```

</details>
