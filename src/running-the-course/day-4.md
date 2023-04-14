# Day 4

> This page is for the course instructor.

The afternoon of the fourth day should cover a topic of your choice. Include
the topic in the announcement of the course, so that participants know what to
expect.

This phase of the course is a chance for participants to see Rust in action on a
codebase they might be familiar with. You can choose from the topics already
defined here, or plan your own.

Some topics need additional preparation:

## Android

If you chose Android for Day 4 afternoon, you will need an [AOSP checkout][1].
Make a checkout of the [course repository][2] on the same machine and move the
`src/android/` directory into the root of your AOSP checkout. This will ensure
that the Android build system sees the `Android.bp` files in `src/android/`.

Ensure that `adb sync` works with your emulator or real device and pre-build
all Android examples using `src/android/build_all.sh`. Read the script to see
the commands it runs and make sure they work when you run them by hand.

## Async

If you chose Async for Day 4 afternoon, you will need a fresh crate set up and
the dependencies downloaded and ready to go. You can then copy/paste the
examples into `src/main.rs` to experiment with them.

```shell
cargo init day4
cd day4
cargo add tokio --features full
cargo run
```

[1]: https://source.android.com/docs/setup/download/downloading
[2]: https://github.com/google/comprehensive-rust
