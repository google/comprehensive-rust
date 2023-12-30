---
course: Android
session: Android
---

# Welcome to Rust in Android

Rust is supported for system software on Android. This means that you can write
new services, libraries, drivers or even firmware in Rust (or improve existing
code as needed).

> We will attempt to call Rust from one of your own projects today. So try to
> find a little corner of your code base where we can move some lines of code to
> Rust. The fewer dependencies and "exotic" types the better. Something that
> parses some raw bytes would be ideal.

<details>

The speaker may mention any of the following given the increased use of Rust in
Android:

- Service example:
  [DNS over HTTP](https://security.googleblog.com/2022/07/dns-over-http3-in-android.html)

- Libraries:
  [Rutabaga Virtual Graphics Interface](https://crosvm.dev/book/appendix/rutabaga_gfx.html)

- Kernel Drivers:
  [Binder](https://lore.kernel.org/rust-for-linux/20231101-rust-binder-v1-0-08ba9197f637@google.com/)

- Firmware:
  [pKVM firmware](https://security.googleblog.com/2023/10/bare-metal-rust-in-android.html)

</details>
