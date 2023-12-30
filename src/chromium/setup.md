# Setup

Make sure you can build and run Chromium. Any platform and set of build flags is
OK, so long as your code is relatively recent (commit position 1223636 onwards,
corresponding to November 2023):

```shell
gn gen out/Debug
autoninja -C out/Debug chrome
out/Debug/chrome # or on Mac, out/Debug/Chromium.app/Contents/MacOS/Chromium
```

(A component, debug build is recommended for quickest iteration time. This is
the default!)

See
[How to build Chromium](https://www.chromium.org/developers/how-tos/get-the-code/)
if you aren't already at that point. Be warned: setting up to build Chromium
takes time.

It's also recommended that you have Visual Studio code installed.

# About the exercises

This part of the course has a series of exercises which build on each other.
We'll be doing them spread throughout the course instead of just at the end. If
you don't have time to complete a certain part, don't worry: you can catch up in
the next slot.
