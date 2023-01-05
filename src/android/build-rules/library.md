# Rust Libraries
`rust_library`를 사용하여 Android용 새 Rust 라이브러리를 만듭니다.

여기서 2개의 라이브러리 의존성을 설언합니다.

- 아래에 정의한 `libgreeting` 
- [`external/rust/crates/`][crates]에 존재하는 `libtextwrap`
> You use `rust_library` to create a new Rust library for Android.
> 
> Here we declare a dependency on two libraries:

> * `libgreeting`, which we define below,
> * `libtextwrap`, which is a crate already vendored in
  [`external/rust/crates/`][crates].

[crates]: https://cs.android.com/android/platform/superproject/+/master:external/rust/crates/

_hello_rust/Android.bp_:

```javascript
{{#include library/Android.bp}}
```

_hello_rust/src/main.rs_:

```rust,ignore
{{#include library/src/main.rs:main}}
```

_hello_rust/src/lib.rs_:

```rust,ignore
{{#include library/src/lib.rs:greeting}}
```

You build, push, and run the binary like before:

```shell
{{#include ../build_all.sh:hello_rust_with_dep}}
Hello Bob, it is very
nice to meet you!
```
