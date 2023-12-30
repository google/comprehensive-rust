# Debugging

_Embed.toml_:

<!-- mdbook-xgettext: skip -->

```toml
[default.general]
chip = "nrf52833_xxAA"

[debug.gdb]
enabled = true
```

In one terminal under `src/bare-metal/microcontrollers/examples/`:

<!-- mdbook-xgettext: skip -->

```sh
cargo embed --bin board_support debug
```

In another terminal in the same directory:

On gLinux or Debian:

<!-- mdbook-xgettext: skip -->

```sh
gdb-multiarch target/thumbv7em-none-eabihf/debug/board_support --eval-command="target remote :1337"
```

On MacOS:

<!-- mdbook-xgettext: skip -->

```sh
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/board_support --eval-command="target remote :1337"
```

<details>

In GDB, try running:

<!-- mdbook-xgettext: skip -->

```gdb
b src/bin/board_support.rs:29
b src/bin/board_support.rs:30
b src/bin/board_support.rs:32
c
c
c
```

</details>
