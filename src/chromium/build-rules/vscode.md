# Visual Studio code

Types are elided in Rust code, which makes a good IDE even more useful than
for C++. Visual Studio code works well for Rust in Chromium. To use it,

* Ensure your VSCode has the `rust-analyzer` extension, not earlier forms
  of Rust support
* `gn gen out/Debug --export-rust-project` (or equivalent for your output
  directory)
* `ln -s out/Debug/rust-project.json rust-project.json`

<img src="vscode.png" style="border: 1px solid black;" alt="Example screenshot from VSCode">

<details>
A demo of some of the code annotation and exploration features of rust-analyzer might be
beneficial if the audience are naturally skeptical of IDEs.
</details>