# Setup

For the microcontroller part of the course we will use the
[BBC micro:bit](https://microbit.org/) v2 as an example. It's a
[development board](https://tech.microbit.org/hardware/) based on the Nordic
nRF52833 microcontroller with some LEDs and buttons, an I2C-connected
accelerometer and compass, and an on-board SWD debugger.

To get started, install some tools we'll need later.

## Linux

Please run the following on gLinux or Debian:

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

## MacOS

Please run the following on MacOS:

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
