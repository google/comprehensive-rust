# Comprehensive Rust 🦀


이 저장소에는 안드로이드 팀에 의해 개발된 4일간의 Comprehensive Rust에 대한 소스 코드가 있습니다.
이 과정은 러스트의 모든 측면을 다룹니다.
기본 구문부터 제네릭 및 오류 처리에 이르기까지 다양한 러스트의 모든 측면을 다룹니다. 
또한 마지막 날에는 안드로이드 관련 콘텐츠까지 다룹니다.
> This repository has the source code for Comprehensive Rust 🦀, a four day Rust
course developed by the Android team. The course covers all aspects of Rust,
from basic syntax to generics and error handling. It also includes
Android-specific content on the last day.

아래 코스를 방문해 보세요 
**https://google.github.io/comprehensive-rust/**.
> Read the course at **https://google.github.io/comprehensive-rust/**.

## Building


강좌는 [mdBook](https://github.com/rust-lang/mdBook)과 [Svgbob plugin](https://github.com/boozook/mdbook-svgbob)를 사용해서 만들었습니다. 
아래 쉘로 라이브러리를 설치 합니다.
> The course is built using [mdBook](https://github.com/rust-lang/mdBook) and its [Svgbob plugin](https://github.com/boozook/mdbook-svgbob). Install both tools with

```shell
$ cargo install mdbook
$ cargo install mdbook-svgbob
```

실행은 아래와 같이 합니다.
> Then run

```shell
$ mdbook test
```

모든 강의 내용에 대한 테스트는 아래와 같이 실행하세요
> to test all included Rust snippets. Run

```shell
$ mdbook serve
```

<http://localhost:3000>에서 실행된 모든 컨텐츠를 확인할 수 있습니다. 
`mdbook build`을 실행하면 `book/`폴더에서 static 버전이 생성됩니다. 

> to start a web server with the course. You'll find the content on
> <http://localhost:3000>. You can use `mdbook build` to create a static version
> of the course in the `book/` directory.


## Contact

질문이나 의견이 있다면 [Martin Geisler](mailto:mgeisler@google.com)에게 연락을 주시거나 
[discussion on GitHub](https://github.com/google/comprehensive-rust/discussions)에 남겨주세요.

> For questions or comments, please contact [Martin Geisler](mailto:mgeisler@google.com) or start a [discussion on GitHub](https://github.com/google/comprehensive-rust/discussions). We would love to hear from you.

---
## 역자주
스터디 겸 해서 저장소 fork후 번역을 시작했습니다.

src 폴더 내에 md들을 구글 블로그 순서대로 번역 예정인데... 양이.... 

순서는 [Welcome to Comprehensive Rust 🦀](src/welcome.md)부터 시작합니다. 

## history
2022-12-28: 시작
- readme.md 1차 완료
- Welcome to Comprehensive Rust 🦀 1차 완료


