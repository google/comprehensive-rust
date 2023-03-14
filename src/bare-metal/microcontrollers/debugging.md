# Debugging

Embed.toml:

```toml
[default.general]
chip = "nrf52833_xxAA"

[debug.gdb]
enabled = true
```

In one terminal:

```sh
cargo embed --bin board_support debug
```

In another terminal:

```sh
gdb-multiarch target/thumbv7em-none-eabihf/debug/board_support --eval-command="target remote :1337"
```

<details>

In GDB, try running:

```gdb
b src/bin/board_support.rs:28
b src/bin/board_support.rs:29
b src/bin/board_support.rs:31
c
c
c
```

</details>
