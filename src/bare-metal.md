# Welcome to Bare Metal Rust

This is a standalone one-day course about bare-metal Rust, aimed at people who are familiar with the
basics of Rust (perhaps from completing the Comprehensive Rust course), and ideally also have some
experience with bare-metal programming in some other language such as C.

Today we will talk about 'bare-metal' Rust: running Rust code without an OS underneath us. This will
be divided into several parts:

- What is `no_std` Rust?
- Writing firmware for microcontrollers.
- Writing bootloader / kernel code for application processors.
- Some useful crates for bare-metal Rust development.

For the microcontroller part of the course we will use the [BBC micro:bit](https://microbit.org/) v2
as an example. It's a [development board](https://tech.microbit.org/hardware/) based on the Nordic
nRF51822 microcontroller with some LEDs and buttons, an I2C-connected accelerometer and compass, and
an on-board SWD debugger.

To get started, install some tools we'll need later. On gLinux or Debian:
<!-- mdbook-xgettext: skip -->
```bash
sudo apt install gcc-aarch64-linux-gnu gdb-multiarch libudev-dev picocom pkg-config qemu-system-arm
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils cargo-embed
```

And give users in the `plugdev` group access to the micro:bit programmer:
<!-- mdbook-xgettext: skip -->
```bash
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", MODE="0664", GROUP="plugdev"' |\
  sudo tee /etc/udev/rules.d/50-microbit.rules
sudo udevadm control --reload-rules
```

On MacOS:
<!-- mdbook-xgettext: skip -->
```bash
xcode-select --install
brew install gdb picocom qemu
brew install --cask gcc-aarch64-embedded
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils cargo-embed
```
