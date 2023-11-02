# Resolving problems

In the Cargo ecosystem, build instructions for crates are spread across:

* The declarative `Cargo.toml`
* The (optional) imperative [`build.rs` script][0]

Our tooling is pretty good at converting `Cargo.toml` files to appropriate
`gn` rules.

But, `build.rs` is more difficult. This is a program which is itself run
_as part of the build of a crate_. That program can do arbitrary things.
This is fundamentally at odds with the design of `gn` and `ninja` which aim
for static, deterministic, build rules to maximize parallelism and
repeatability of builds.

In Chromium, we emulate Cargo to build and run `build.rs` scripts, but we
can't possibly simulate all the possible things that arbitrary code can
do.

If your crate - or one of its transitive dependencies - does unusual things
in its `build.rs`, you're going to have to resolve how to make that work
with `gn`.

Specifically:

| build script effect | Supported by our gn templates | Work required by you |
|-----|-----|-----|
| Checking rustc version to configure features on and off | Yes | None |
| Checking platform or CPU to configure features on and off | Yes | None |
| Generating code | Yes | Yes - specify in `third_party.toml` |
| Reading files | Yes | Yes - specify in `third_party.toml` |
| Building C/C++ | No | Patch around it |
| Arbitrary other actions | No | Patch around it |

Fortunately, most crates don't contain a build script, and fortunately, most
build scripts only do the top two actions.

## Build scripts which generate code

Some `build.rs` files generate files of Rust code.

You'll find out about this problem when you first come to actually build
the code in Chromium. Most likely, `ninja` will complain about a missing
source code file.

You should then check the `build.rs` file for the script to see if it reads
or writes source code files.

If so, modify [`third_party.toml`][1] to add `build-script-outputs` to the
crate. If this is a transitive dependency, that is, one on which Chromium
code should not directly depend, also add `allow-first-party-usage=false`.
There are several examples already in that file:

```toml
[dependencies.unicode-linebreak]
version = "0.1"
allow-first-party-usage = false
build-script-outputs = [ "tables.rs" ]
```

Now rerun [`gnrt.py gen`][2] to regenerate `BUILD.gn` files to inform ninja
that this particular output file is input to subsequent build steps.

## Build scripts which read files

If the `build.rs` depends on reading code from the crate or the rest of the
Chromium source code tree, you can similarly specify `build-script-inputs`.
There are no examples of this right now.

## Building C/C++ or arbitrary other actions - patching a crate

Some crates use the [`cc`][2] crate to build and link C/C++ libraries.
Other crates parse C/C++ using [`bindgen`][3] within their build scripts.
These actions can't be supported in a Chromium context - our gn, ninja
and LLVM build system is very specific in expressing relationships between
build actions.

So, your options are:

* Avoid these crates
* Apply a patch to the crate.

Patches should be kept in `third_party/rust/<crate>/<epoch>/patches` -
see for example the [patches against the cxx crate][4]. There is currently
no automation - [simply create and apply patches manually][5] to remove the
problematic actions from the build script.

If your patches modify the `Cargo.toml` file, rerun `gnrt gen`.

## Conflicting versions

The crate that you want to include might have transitive dependencies
already in the tree, and perhaps it needs a different version.

Multiple versions of crates are allowed in Chromium, but it's strongly advised
that you try to resolve this problem to reduce duplication, for example
by upgrading some other crate.

[0]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[1]: configuring-third-party-toml.md
[2]: https://crates.io/crates/cc
[3]: https://crates.io/crates/bindgen
[4]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/cxx/v1/patches/
[5]: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#patching-third_party-crates