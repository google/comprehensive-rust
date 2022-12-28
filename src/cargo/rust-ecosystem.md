# The Rust Ecosystem

러스트의 생태계는 여러가지 도구들로 구성되어 있으며, 그 중 중요한 것들은 아래와 같습니다. 
> The Rust ecosystem consists of a number of tools, of which the main ones are:
* `rustc`: `.rs`확장자 파일을 바이너리 혹은 다른 중간자 형식으로 변환해주는 Rust 컴파일러입니다.
> * `rustc`: the Rust compiler which turns `.rs` files into binaries and other
>   intermediate formats.

* `cargo`: 러스트 종속성 관리자 및 빌드도구 입니다. <https://crates.io>에서 호스팅되는 여러 종속성을 다운로드하고 `rustc`가 당신의 프로젝트를 빌드할 때 이를 전달합니다. 
  또한 유닛 테스트를 실행하는 테스트 툴을 내장하고 있습니다.
> * `cargo`: the Rust dependency manager and build tool. Cargo knows how to
>   download dependencies hosted on <https://crates.io> and it will pass them to
>   `rustc` when building your project. Cargo also comes with a built-in test
>   runner which is used to execute unit tests.

* `rustup`: 러스트 툴체인 설치 프로그램 및 업데이트 프로그램. 
  이 도구는 새 버전의 러스트가 출시될 때 `rustc` 및 `cargo` 설치하고 업데이트하는 데 사용됩니다. 
  또한 `rustup`은 표준 라이브러리에 대한 문서를 다운로드할 수도 있습니다. 
  한 번에 여러 버전의 러스트를 설치할 수 있으며 `rustup`을 통해 필요에 따라 이들 버전을 전환할 수 있습니다
> * `rustup`: the Rust toolchain installer and updater. This tool is used to
>   install and update `rustc` and `cargo` when new versions of Rust is released.
>   In addition, `rustup` can also download documentation for the standard
>   library. You can have multiple versions of Rust installed at once and `rustup`
>   will let you switch between them as needed.

