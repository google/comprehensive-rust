# Testing in Android

Building on [Testing](../testing.md), we will now look at how unit tests work in
AOSP. Use the `rust_test` module for your unit tests:

_testing/Android.bp_:

```javascript
{{#include testing/Android.bp}}
```

_testing/src/lib.rs_:

```rust
{{#include testing/src/lib.rs:leftpad}}
```

You can now run the test with

```shell
{{#include build_all.sh:libleftpad_test}}
```

The output looks like this:

```text
INFO: Elapsed time: 2.666s, Critical Path: 2.40s
INFO: 3 processes: 2 internal, 1 linux-sandbox.
INFO: Build completed successfully, 3 total actions
//comprehensive-rust-android/testing:libleftpad_test_host            PASSED in 2.3s
    PASSED  libleftpad_test.tests::long_string (0.0s)
    PASSED  libleftpad_test.tests::short_string (0.0s)
Test cases: finished with 2 passing and 0 failing out of 2 test cases
```

Notice how you only mention the root of the library crate. Tests are found
recursively in nested modules.
