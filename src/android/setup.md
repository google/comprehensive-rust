# Setup

We will be using a Cuttlefish Android Virtual Device to test our code. Make sure
you have access to one or create a new one with:

```shell
source build/envsetup.sh
lunch aosp_cf_x86_64_phone-trunk_staging-userdebug
acloud create
```

Please see the
[Android Developer Codelab](https://source.android.com/docs/setup/start) for
details.

<details>

Key points:

- Cuttlefish is a reference Android device designed to work on generic Linux
  desktops. MacOS support is also planned.

- The Cuttlefish system image maintains high fidelity to real devices, and is
  the ideal emulator to run many Rust use cases.

</details>
