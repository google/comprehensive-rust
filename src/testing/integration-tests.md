# Integration Tests

라이브러리를 실제 테스트 하기위한 통합테스트는 `test/` 폴더 내에 `.rs`를 작성하여 수행합니다: 
> If you want to test your library as a client, use an integration test.
> Create a `.rs` file under `tests/`:

```rust,ignore
use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}
```

이 테스트는 크레이트의 공개 API에만 접근할 수 있습니다.
> These tests only have access to the public API of your crate.
