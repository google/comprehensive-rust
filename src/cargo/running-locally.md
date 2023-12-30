# Running Code Locally with Cargo

If you want to experiment with the code on your own system, then you will need
to first install Rust. Do this by following the
[instructions in the Rust Book][1]. This should give you a working `rustc` and
`cargo`. At the time of writing, the latest stable Rust release has these
version numbers:

```shell
% rustc --version
rustc 1.69.0 (84c898d65 2023-04-16)
% cargo --version
cargo 1.69.0 (6e9a83356 2023-04-12)
```

You can use any later version too since Rust maintains backwards compatibility.

With this in place, follow these steps to build a Rust binary from one of the
examples in this training:

1. Click the "Copy to clipboard" button on the example you want to copy.

2. Use `cargo new exercise` to create a new `exercise/` directory for your code:

   ```shell
   $ cargo new exercise
        Created binary (application) `exercise` package
   ```

3. Navigate into `exercise/` and use `cargo run` to build and run your binary:

   ```shell
   $ cd exercise
   $ cargo run
      Compiling exercise v0.1.0 (/home/mgeisler/tmp/exercise)
       Finished dev [unoptimized + debuginfo] target(s) in 0.75s
        Running `target/debug/exercise`
   Hello, world!
   ```

4. Replace the boiler-plate code in `src/main.rs` with your own code. For
   example, using the example on the previous page, make `src/main.rs` look like

   ```rust
   fn main() {
       println!("Edit me!");
   }
   ```

5. Use `cargo run` to build and run your updated binary:

   ```shell
   $ cargo run
      Compiling exercise v0.1.0 (/home/mgeisler/tmp/exercise)
       Finished dev [unoptimized + debuginfo] target(s) in 0.24s
        Running `target/debug/exercise`
   Edit me!
   ```

6. Use `cargo check` to quickly check your project for errors, use `cargo build`
   to compile it without running it. You will find the output in `target/debug/`
   for a normal debug build. Use `cargo build --release` to produce an optimized
   release build in `target/release/`.

7. You can add dependencies for your project by editing `Cargo.toml`. When you
   run `cargo` commands, it will automatically download and compile missing
   dependencies for you.

[1]: https://doc.rust-lang.org/book/ch01-01-installation.html

<details>

Try to encourage the class participants to install Cargo and use a local editor.
It will make their life easier since they will have a normal development
environment.

</details>
