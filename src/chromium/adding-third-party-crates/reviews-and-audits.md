# Auditing Third Party Crates

Adding new libraries is subject to Chromium's standard [policies][0], but of
course also subject to security review. As you may be bringing in not just a
single crate but also transitive dependencies, there may be a lot of code to
review. On the other hand, safe Rust code can have limited negative side
effects. How should you review it?

Over time Chromium aims to move to a process based around [cargo vet][1].

Meanwhile, for each new crate addition, we are checking for the following:

- Understand why each crate is used. What's the relationship between crates? If
  the build system for each crate contains a `build.rs` or procedural macros,
  work out what they're for. Are they compatible with the way Chromium is
  normally built?
- Check each crate seems to be reasonably well maintained
- Use `cd third-party/rust/chromium_crates_io; cargo audit` to check for known
  vulnerabilities (first you'll need to `cargo install cargo-audit`, which
  ironically involves downloading lots of dependencies from the internet[2])
- Ensure any `unsafe` code is good enough for the [Rule of Two][3]
- Check for any use of `fs` or `net` APIs
- Read all the code at a sufficient level to look for anything out of place that
  might have been maliciously inserted. (You can't realistically aim for 100%
  perfection here: there's often just too much code.)

These are just guidelines --- work with reviewers from `security@chromium.org`
to work out the right way to become confident of the crate.

[0]: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#Third_party-review
[1]: https://mozilla.github.io/cargo-vet/
[2]: ../cargo.md
[3]: https://chromium.googlesource.com/chromium/src/+/main/docs/security/rule-of-2.md#unsafe-code-in-safe-languages
