# Comprehensive Rust 🦀


이 저장소에는 안드로이드 팀에 의해 개발된 4일간의 Comprehensive Rust에 대한 소스 코드가 있습니다.
이 과정은 러스트의 모든 측면을 다룹니다.
기본 구문부터 제네릭 및 오류 처리에 이르기까지 다양한 러스트의 모든 측면을 다룹니다. 
또한 마지막 날에는 안드로이드 관련 콘텐츠까지 다룹니다.
> This repository has the source code for Comprehensive Rust 🦀, a four day Rust
course developed by the Android team. The course covers all aspects of Rust,
from basic syntax to generics and error handling. It also includes
Android-specific content on the last day.

(원본)사이트를 방문해 보세요 **https://google.github.io/comprehensive-rust/**.  
* 원문병기 번역본은 **https://keispace.github.io/comprehensive-rust-kr/**
> Read the course at **https://google.github.io/comprehensive-rust/**.

## Course Format and Target Audience

The course is used internally at Google when teaching Rust to experienced
software engineers. They typically have a background in C++ or Java.

The course is taught in a classroom setting and we hope it will be useful for
others who want to teach Rust to their team. The course will be less useful for
self-study since you miss out on the discussions happening in the classroom. You
don't see the questions and answers and you don't see the compiler errors we
trigger when going through the code samples. We hope to improve on this via
[speaker notes](https://github.com/google/comprehensive-rust/issues/53) and by
[publishing videos](https://github.com/google/comprehensive-rust/issues/52).

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
mdbook search 기능이 한국어를 지원하지 않아서 찾아봤는데 아직 러스트 실력이 미천해서 mdbook에서 한국어 지원하도록 PR 보낼 수가 없네요(...) 그래서 검색용 영어와 병기해서 번역 합니다.

4일차 오후 강의는 안드로이드 OS에 대한 부분입니다. 아직 여기까지 닿을 실력이 안되서 4일차 오후 챕터(33,34 챕터)는 번역 스킵합니다(여기 볼 정도면 굳이 번역이...)

순서는 [Welcome to Comprehensive Rust 🦀](src/welcome.md)부터 시작하는데 해당 문서의 mdbook을 serve해서 보시거나 본 페이지 [인터넷북](https://keispace.github.io/comprehensive-rust-kr)에서 보시길 추천합니다.(next prev 이동 편의)

## history
- 1차 완료 파트
    - 2022-12-28: 시작 ~ 5.1
    - 2022-12-28: ~ 6.4
    - 2022-12-29: ~ 10.5
    - 2022-12-30: ~ 18.1
    - 2023-01-02: ~ 25(day3 morning)
    - 2023-01-03: ~ 31.4
    - 2023-01-04: 4일차 오후(안드로이드) 제외 전체 완료. 
    - 2023-01-05: 33.1~33.2 작업