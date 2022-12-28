# Using Cargo

Rust를 시작하려고하면 당신은 곧 [Cargo](https://doc.rust-lang.org/cargo/)라는 Rust 생태계에서 사용하는 표준 빌드/실행 도구를 만날 것 입니다. 
여기서는 카고가 무엇인지, 그리고 카고가 더 넓은 생태계에 어떻게 적합한지, 그리고 이 교육에 어떻게 적합한지에 대한 간략한 개요를 제공하고자 합니다.

> When you start reading about Rust, you will soon meet [Cargo](https://doc.rust-lang.org/cargo/), the standard tool
> used in the Rust ecosystem to build and run Rust applications. Here we want to
> give a brief overview of what Cargo is and how it fits into the wider ecosystem
> and how it fits into this training.

Debian이나 Ubuntu에서 cargo와 Rust소스를 아래 커맨드로 설치합니다. 
> On Debian/Ubuntu, you can install Cargo and the Rust source with

```shell
$ sudo apt install cargo rust-src
```

[VS Code][2]에서 작업하는 걸 추천 드립니다. [rust-analyzer][1] 확장을 통해 정의 이동 등 개발에 도움 받을 수 있습니다.(또는 다른 IDE나 편집기를 사용해도 무방합니다.)
> This will allow [rust-analyzer][1] to jump to the definitions. We suggest using
> [VS Code][2] to edit the code (but any LSP compatible editor works).

참고로, 만약 가능하다면 [rustup](https://rustup.rs/)과 같은 Rust 생태계 툴을 통한 설치를 추천드립니다. 
> As a sidenote: if you have the access/capability to do so, it's recommended to
> install Rust's tooling via [rustup](https://rustup.rs/) since it's better integrated with the
> rest of the ecosystem.

[1]: https://rust-analyzer.github.io/
[2]: https://code.visualstudio.com/

