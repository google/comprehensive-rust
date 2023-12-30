# Resolving Problems

If your build fails, it may be because of a `build.rs`: programs which do
arbitrary things at build time. This is fundamentally at odds with the design of
`gn` and `ninja` which aim for static, deterministic, build rules to maximize
parallelism and repeatability of builds.

Some `build.rs` actions are automatically supported; others require action:

| build script effect                                       | Supported by our gn templates | Work required by you                |
| --------------------------------------------------------- | ----------------------------- | ----------------------------------- |
| Checking rustc version to configure features on and off   | Yes                           | None                                |
| Checking platform or CPU to configure features on and off | Yes                           | None                                |
| Generating code                                           | Yes                           | Yes - specify in `gnrt_config.toml` |
| Building C/C++                                            | No                            | Patch around it                     |
| Arbitrary other actions                                   | No                            | Patch around it                     |

Fortunately, most crates don't contain a build script, and fortunately, most
build scripts only do the top two actions.

[0]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
