# Checking Crates into Chromium Source Code

`git status` should reveal:

- Crate code in `//third_party/rust/chromium_crates_io`
- Metadata (`BUILD.gn` and `README.chromium`) in
  `//third_party/rust/<crate>/<version>`

Please also add an `OWNERS` file in the latter location.

You should land all this, along with your `Cargo.toml` and `gnrt_config.toml`
changes, into the Chromium repo.

**Important**: you need to use `git add -f` because otherwise `.gitignore` files
may result in some files being skipped.

As you do so, you might find presubmit checks fail because of non-inclusive
language. This is because Rust crate data tends to include names of git
branches, and many projects still use non-inclusive terminology there. So you
may need to run:

```shell
infra/update_inclusive_language_presubmit_exempt_dirs.sh > infra/inclusive_language_presubmit_exempt_dirs.txt
git add -p infra/inclusive_language_presubmit_exempt_dirs.txt # add whatever changes are yours
```
