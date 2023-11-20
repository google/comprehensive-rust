# Generating gn build rules

A tool called `gnrt` knows how to generate Chromium build rules for the
crate you just added.

To start, download the crate you want like this:

```
cd chromium/src
vpython3 tools/crates/run_gnrt.py -- vendor
```

> Although the `gnrt` tool is part of the Chromium source code,
> by running this command you will be downloading and running its dependencies
> from `crates.io`. See [the earlier section][1] discussing this security
> decision.

This `vendor` command may download:
* Your crate
* Direct and transitive dependencies
* Updates or new versions of other crates

If a crate in `//third_party/rust/chromium_crates_io/patches` was updated as
part of vendoring, then reapply patches to it by running
`cd //third_party/rust/chromium_crates_io; ./apply_patches.sh`.

Once you've downloaded the crate, generate the `BUILD.gn` files like this:

```
vpython3 tools/crates/run_gnrt.py -- gen
```

Now run `git status`. You should find:

* At least one new crate source code in `third_party/rust/chromium_crates_io/vendor`
* At least one new `BUILD.gn` in `third_party/rust/<crate name>`
* An appropriate `README.chromium`

Take a close look, especially at the things generated in `third_party/rust`.


[0]: https://chromium.googlesource.com/chromium/src/+/main/docs/adding_to_third_party.md#add-a-readme_chromium
[1]: ../cargo.md