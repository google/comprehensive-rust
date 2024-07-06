# Course Structure

> This page is for the course instructor.

## Rust Fundamentals

The first four days make up [Rust Fundamentals](../welcome-day-1.md). The days
are fast paced and we cover a lot of ground!

{{%course outline Fundamentals}}

## Deep Dives

In addition to the 4-day class on Rust Fundamentals, we cover some more
specialized topics:

### Rust in Android

The [Rust in Android](../android.md) deep dive is a half-day course on using
Rust for Android platform development. This includes interoperability with C,
C++, and Java.

You will need an [AOSP checkout][1]. Make a checkout of the
[course repository][2] on the same machine and move the `src/android/` directory
into the root of your AOSP checkout. This will ensure that the Android build
system sees the `Android.bp` files in `src/android/`.

Ensure that `adb sync` works with your emulator or real device and pre-build all
Android examples using `src/android/build_all.sh`. Read the script to see the
commands it runs and make sure they work when you run them by hand.

[1]: https://source.android.com/docs/setup/download/downloading
[2]: https://github.com/google/comprehensive-rust

### Rust in Chromium

The [Rust in Chromium](../chromium.md) deep dive is a half-day course on using
Rust as part of the Chromium browser. It includes using Rust in Chromium's `gn`
build system, bringing in third-party libraries ("crates") and C++
interoperability.

You will need to be able to build Chromium --- a debug, component build is
[recommended](../chromium/setup.md) for speed but any build will work. Ensure
that you can run the Chromium browser that you've built.

### Bare-Metal Rust

The [Bare-Metal Rust](../bare-metal.md) deep dive is a full day class on using
Rust for bare-metal (embedded) development. Both microcontrollers and
application processors are covered.

For the microcontroller part, you will need to buy the
[BBC micro:bit](https://microbit.org/) v2 development board ahead of time.
Everybody will need to install a number of packages as described on the
[welcome page](../bare-metal.md).

### Concurrency in Rust

The [Concurrency in Rust](../concurrency/welcome.md) deep dive is a full day
class on classical as well as `async`/`await` concurrency.

You will need a fresh crate set up and the dependencies downloaded and ready to
go. You can then copy/paste the examples into `src/main.rs` to experiment with
them:

```shell
cargo init concurrency
cd concurrency
cargo add tokio --features full
cargo run
```

{{%course outline Concurrency}}

## Format

The course is meant to be very interactive and we recommend letting the
questions drive the exploration of Rust!
