---
course: Bare Metal
session: Morning
---

# Welcome dans Rust Bare Metal

Il s'agit d'un cours autonome d'une journée consacré à Rust en environnement bare-metal, destiné aux personnes
déjà familières avec les bases de Rust (par exemple après avoir suici le cours Comprehensive
Rust), et idéalement ayant une certaine expérience du développement bare-metal
dans un autre langage comme le C.

AUjourd'hui, nous allons explorer Rust en environnement bare-metal : exécuter du code Rust sans système d'exploitation. 
Le cours sera structuré en plusieurs parties :

- Qu'est-ce que Rust `no_std` ?
- Écrire du micrologiciel (firmware) pour microcontrôleurs.
- Écrire un bootloader / du code noyau pour des processeurs applicatifs.
- Quelques crates utiles pour le développement bare-metal en Rust.

Pour la partie sur les microcontrôleurs, nous utiliserons comme exemple le 
[BBC micro:bit](https://microbit.org/) v2. Il s’agit d’une 
[carte de développement](https://tech.microbit.org/hardware/) basée sur le microcontrôleur Nordic 
nRF52833, avec des LED, des boutons, un 
accéléromètre et une boussole connectés en I2C, ainsi qu’un débogueur SWD intégré.

Pour commencer, installez quelques outils dont nous aurons besoin plus tard. Sur gLinux ou Debian :

<!-- mdbook-xgettext: skip -->

```bash
sudo apt install gdb-multiarch libudev-dev picocom pkg-config qemu-system-arm build-essential
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

And give users in the `plugdev` group access to the micro:bit programmer:
Et donner aux utilisateurs du groupe `plugdev` l'accès au programmateur micro:bit : 

<!-- mdbook-xgettext: skip -->

```bash
echo 'SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0d28", MODE="0660", GROUP="logindev", TAG+="uaccess"' |\
  sudo tee /etc/udev/rules.d/50-microbit.rules
sudo udevadm control --reload-rules
```

Vous devriez voir "NXP ARM mbed" en sortie du `lsusb` si l'appareil est
disponible. Si vous utilisez un environnement Linux sur un Chromebook, vous devrez
partager le périphérique USB avec Linux, via
`chrome://os-settings/crostini/sharedUsbDevices`.

Sur MacOS:

<!-- mdbook-xgettext: skip -->

```bash
xcode-select --install
brew install gdb picocom qemu
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```
