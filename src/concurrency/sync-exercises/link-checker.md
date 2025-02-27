---
minutes: 20
---

# Multi-threaded Link Checker

Let us use our new knowledge to create a multi-threaded link checker. It should
start at a webpage and check that links on the page are valid. It should
recursively check other pages on the same domain and keep doing this until all
pages have been validated.

For this, you will need an HTTP client such as [`reqwest`][1]. You will also
need a way to find links, we can use [`scraper`][2]. Finally, we'll need some
way of handling errors, we will use [`thiserror`][3].

Create a new Cargo project and `reqwest` it as a dependency with:

```shell
cargo new link-checker
cd link-checker
cargo add --features blocking,rustls-tls reqwest
cargo add scraper
cargo add thiserror
```

> If `cargo add` fails with `error: no such subcommand`, then please edit the
> `Cargo.toml` file by hand. Add the dependencies listed below.

The `cargo add` calls will update the `Cargo.toml` file to look like this:

<!-- File Cargo.toml -->

```toml
[package]
name = "link-checker"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
reqwest = { version = "0.11.12", features = ["blocking", "rustls-tls"] }
scraper = "0.13.0"
thiserror = "1.0.37"
```

You can now download the start page. Try with a small site such as
`https://www.google.org/`.

Your `src/main.rs` file should look something like this:

<!-- File src/main.rs -->

```rust,compile_fail
{{#include link-checker.rs:setup}}

{{#include link-checker.rs:visit_page}}

fn main() {
    let client = Client::new();
    let start_url = Url::parse("https://www.google.org").unwrap();
    let crawl_command = CrawlCommand{ url: start_url, extract_links: true };
    match visit_page(&client, &crawl_command) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
```

Run the code in `src/main.rs` with

```shell
cargo run
```

## Tasks

- Use threads to check the links in parallel: send the URLs to be checked to a
  channel and let a few threads check the URLs in parallel.
- Extend this to recursively extract links from all pages on the
  `www.google.org` domain. Put an upper limit of 100 pages or so so that you
  don't end up being blocked by the site.

<details>

- This is a complex exercise and intended to give students an opportunity to
  work on a larger project than others. A success condition for this exercise is
  to get stuck on some "real" issue and work through it with the support of
  other students or the instructor.

</details>

[1]: https://docs.rs/reqwest/
[2]: https://docs.rs/scraper/
[3]: https://docs.rs/thiserror/
