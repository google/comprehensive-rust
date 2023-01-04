# Multi-threaded Link Checker

새로 배운것들을 활용해서 멀티 스레드 링크 검사기를 만듭니다. 
검사기는 웹페이지가 유효한지 확인합니다. 
그리고 재귀적으로 동일 도메인의 다른 모든 페이지가 유효한지 확인합니다.

이를 위해서 [`reqwest`][1]와 같은 HTTP 클라이언트가 필요합니다. 새로운 로컬 프로젝트를 만들고 [`reqwest`][1]를 종속성에 추가하십시요
> Let us use our new knowledge to create a multi-threaded link checker. It should
> start at a webpage and check that links on the page are valid. It should
> recursively check other pages on the same domain and keep doing this until all
> pages have been validated.
> 
> For this, you will need an HTTP client such as [`reqwest`][1]. Create a new
> Cargo project and `reqwest` it as a dependency with:
```shell
$ cargo new link-checker
$ cd link-checker
$ cargo add --features blocking reqwest
```
> 만일 `cargo add` 커맨드가 `error: no such subcommand` 로 실패한다면 
> `Cargo.toml` 파일을 직접 수정해도 됩니다. 아래에 전체 종속성 내용이 있습니다.

링크를 찾기 위해서 [`scraper`][2]도 추가합니다:

> > If `cargo add` fails with `error: no such subcommand`, then please edit the
> > `Cargo.toml` file by hand. Add the dependencies listed below.
> 
> You will also need a way to find links. We can use [`scraper`][2] for that:

```shell
$ cargo add scraper
```
마지막으로 오류 처리하는 방법으로 [`thiserror`][3]도 추가합니다:
> Finally, we'll need some way of handling errors. We [`thiserror`][3] for that:

```shell
$ cargo add thiserror
```
모든 `cargo add`이 끝나면 `Cargo.toml`에 아래 내용이 추가 되있습니다:
> The `cargo add` calls will update the `Cargo.toml` file to look like this:

```toml
# 실습 시 버전은 다를 수 있습니다.
[dependencies]
reqwest = { version = "0.11.12", features = ["blocking"] }
scraper = "0.13.0"
thiserror = "1.0.37"
```

이제 `https://www.google.org/` 같은 웹 페이지를 탐색할 수 있습니다. 
> You can now download the start page. Try with a small site such as
> `https://www.google.org/`.

`rc/main.rs`파일은 아래와 같습니다:
> Your `src/main.rs` file should look something like this:

```rust,compile_fail
{{#include link-checker.rs:setup}}

{{#include link-checker.rs:extract_links}}

fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    let response = get(start_url).unwrap();
    match extract_links(response) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
```

아래 커맨드로 소스를 실행합니다: 
> Run the code in `src/main.rs` with

```shell
$ cargo run
```

> 만약 `cargo run`이 OpenSSL을 언급하며 오류가 발생한다면 `sudo apt install libssl-dev`커맨드를 실행해서 OpenSSL 헤더를 설치해봅니다.
> > If `cargo run` fails with an error mentioning OpenSSL, ensure that you have
> > the OpenSSL headers installed by running `sudo apt install libssl-dev`.

## Tasks

* 스레드를 사용하여 링크를 병렬로 확인합니다: URL을 채널로 보내서 몇 개의 스레드가 URL을 병렬로 체크하도록 합니다.
* `www.google.org`도메인의 모든 페이지를 재귀적으로 확인하기 위해 코드를 확장해서 작성합니다: 사이트에 의해 차단되지 않도록 100페이지 정도로 제한을 두시기 바랍니다.

> * Use threads to check the links in parallel: send the URLs to be checked to a
>   channel and let a few threads check the URLs in parallel.
> * Extend this to recursively extract links from all pages on the
>   `www.google.org` domain. Put an upper limit of 100 pages or so so that you
>   don't end up being blocked by the site.


---
역주 
- 기본 소스는 한번에 한 링크씩 확인하는 소스 입니다. 
- 이를 스레드를 사용하여 병렬화 하는 코드를 작성하시면 됩니다.



[1]: https://docs.rs/reqwest/
[2]: https://docs.rs/scraper/
[3]: https://docs.rs/thiserror/
