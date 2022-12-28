# Running Code Locally with Cargo

만약 당신이 로컬 시스템에서 코드를 실행해보려면 먼저 Rust를 설치해야 합니다. 
[Rust Book][1]의 지침에 따라 `rustc`와 `cargo`를 함께 설치 하세요
설치 후 아래 커맨드를 통해 각 생태계 툴의 버전을 확인 할 수 있습니다. 
> If you want to experiment with the code on your own system, then you will need
>   to first install Rust. Do this by following the [instructions in the Rust
>   Book][1]. This should give you a working `rustc` and `cargo`. At the time of
>   writing, the latest stable Rust release has these version numbers:

```shell
% rustc --version
rustc 1.61.0 (fe5b13d68 2022-05-18)
% cargo --version
cargo 1.61.0 (a028ae4 2022-04-29)
```

정상적으로 설치가 되었으면 강의의 코드 블록중 하나를 아래 단계를 따라 로컬에서 실행할 수 있습니다.
> With this is in place, then follow these steps to build a Rust binary from one
of the examples in this training:

1. 예시 블록에 있는 "Copy to clipboard"버튼을 클릭해서 복사합니다.
> 1. Click the "Copy to clipboard" button on the example you want to copy.

2. shell에서 `cargo new exercise`를 입력해서 새로운 `exercise/` 폴더를 만듭니다.
> 2. Use `cargo new exercise` to create a new `exercise/` directory for your code:

    ```shell
    $ cargo new exercise
         Created binary (application) `exercise` package
    ```

3. `exercise/` 폴더로 이동한 후, `cargo run` 커맨드로 코드를 실행합니다.
> 3. Navigate into `exercise/` and use `cargo run` to build and run your binary:

    ```shell
    $ cd exercise
    $ cargo run
       Compiling exercise v0.1.0 (/home/mgeisler/tmp/exercise)
        Finished dev [unoptimized + debuginfo] target(s) in 0.75s
         Running `target/debug/exercise`
    Hello, world!
    ```

4. 보일러플레이트 코드는 `src/main.rs`에 작성합니다. 예를 들어 이전 페이지의 소스를 아래와 같이 `src/main.rs`에 작성합니다.
> 4. Replace the boiler-plate code in `src/main.rs` with your own code. For
   example, using the example on the previous page, make `src/main.rs` look like

    ```rust
    fn main() {
        println!("Edit me!");
    }
    ```

5. `cargo run`커맨드로 소스를 빌드하고 실행합니다.
> 5. Use `cargo run` to build and run your updated binary:

    ```shell
    $ cargo run
       Compiling exercise v0.1.0 (/home/mgeisler/tmp/exercise)
        Finished dev [unoptimized + debuginfo] target(s) in 0.24s
         Running `target/debug/exercise`
    Edit me!
    ```

6. `cargo check`커맨드는 빠르게 당신의 프로젝트에서 에러를 확인할 수 있습니다. 
    - `cargo build`는 실행없이 소스를 컴파일 합니다. 이 경우에 `target/debug/`폴더에서 output을 확인 할 수 있습니다. 
    - `cargo build --release`커맨드는 릴리즈 버전의 최적화된 output으로 컴파일하며 `target/release/`폴더에서 확인 할 수 있습니다.
> 6. Use `cargo check` to quickly check your project for errors, use `cargo build`
   to compile it without running it. You will find the output in `target/debug/`
   for a normal debug build. Use `cargo build --release` to produce an optimized
   release build in `target/release/`.

7. `Cargo.toml`파일에는 당신의 프로젝트에 사용하는 의존성 패키지를 추가할 수 있습니다. 
    - `cargo`커맨드를 실행하면 자동으로 프로젝트의 의존성 패키지를 다운로드, 컴파일 해줍니다.
> 7. You can add dependencies for your project by editing `Cargo.toml`. When you
   run `cargo` commands, it will automatically download and compile missing
   dependencies for you.

[1]: https://doc.rust-lang.org/book/ch01-01-installation.html


---
역주: 
- (혹시몰라 기록하자면)학습용 폴더를 하나 만들고 거기서 지시사항을 실행합니다.

