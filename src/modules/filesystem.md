# Filesystem Hierarchy

모듈의 내용은 생략될 수 있습니다:
> The module content can be omitted:

```rust,editable,compile_fail
mod garden;
```
`garden` 모듈의 내용은 아래 위치에서 확인 할 수 있습니다.
> The `garden` module content is found at:

* `src/garden.rs` (modern Rust 2018 style)
* `src/garden/mod.rs` (older Rust 2015 style)

유사하게 `garden::vegetables` 모듈은 아래 위치에서 확인할 수 있습니다.
> Similarly, a `garden::vegetables` module can be found at:

* `src/garden/vegetables.rs` (modern Rust 2018 style)
* `src/garden/vegetables/mod.rs` (older Rust 2015 style)

`crate`(크레이트)의 루트는 아래 경로 입니다:
> The `crate` root is in:

* `src/lib.rs` (for a library crate)
* `src/main.rs` (for a binary crate)
