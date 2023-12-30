# Build rules exercise

In your Chromium build, add a new Rust target to `//ui/base/BUILD.gn`
containing:

```rust
#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!")
}
```

**Important**: note that `no_mangle` here is considered a type of unsafety by
the Rust compiler, so you'll need to to allow unsafe code in your `gn` target.

Add this new Rust target as a dependency of `//ui/base:base`. Declare this
function at the top of `ui/base/resource/resource_bundle.cc` (later, we'll see
how this can be automated by bindings generation tools):

```cpp
extern "C" void hello_from_rust();
```

Call this function from somewhere in `ui/base/resource/resource_bundle.cc` - we
suggest the top of `ResourceBundle::MaybeMangleLocalizedString`. Build and run
Chromium, and ensure that "Hello from Rust!" is printed lots of times.

If you use VSCode, now set up Rust to work well in VSCode. It will be useful in
subsequent exercises. If you've succeeded, you will be able to use right-click
"Go to definition" on `println!`.

## Where to find help

- The options available to the [`rust_static_library` gn template][0]
- Information about [`#[no_mangle]`][1]
- Information about [`extern "C"`][2]
- Information about gn's [`--export-rust-project`][3] switch
- [How to install rust-analyzer in VSCode][4]

<details>
It's really important that students get this running, because future exercises
will build on it.

This example is unusual because it boils down to the lowest-common-denominator
interop language, C. Both C++ and Rust can natively declare and call C ABI
functions. Later in the course, we'll connect C++ directly to Rust.

`allow_unsafe = true` is required here because `#[no_mangle]` might allow Rust
to generate two functions with the same name, and Rust can no longer guarantee
that the right one is called.

If you need a pure Rust executable, you can also do that using the
`rust_executable` gn template.

</details>

[0]: https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_static_library.gni;l=16
[1]: https://doc.rust-lang.org/beta/reference/abi.html#the-no_mangle-attribute
[2]: https://doc.rust-lang.org/std/keyword.extern.html
[3]: https://gn.googlesource.com/gn/+/main/docs/reference.md#compilation-database
[4]: https://code.visualstudio.com/docs/languages/rust
