# Checking crates into Chromium source code

Downloaded crates live in `//third_party/rust`.

Under each `//third_party/rust/<some crate name>`, you'll find:
* The crate source code in `v1/crate` (or whatever the version is)
* A `BUILD.gn` file
* A `README.chromium` file

Please also add an `OWNERS` file.

You should land all this, along with your `third_party.toml` change, into
the Chromium repo.

As you do so, you might find presubmit checks fail because of non-inclusive
language. This is because Rust crate data tends to include names of git branches,
and many projects still use non-inclusive terminology there. So you may need
to run:

```
infra/update_inclusive_language_presubmit_exempt_dirs.sh > infra/inclusive_language_presubmit_exempt_dirs.txt
git add -p infra/inclusive_language_presubmit_exempt_dirs.txt # add whatever changes are yours
```
