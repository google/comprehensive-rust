# Rust Binaries
간단한 응용 프로그램으로 시작해 보겠습니다. AOSP 체크아웃의 루트에서 다음 파일을 생성합니다:
> Let us start with a simple application. At the root of an AOSP checkout, create
> the following files:

_hello_rust/Android.bp_:

```javascript
{{#include binary/Android.bp}}
```

_hello_rust/src/main.rs_:

```rust
{{#include binary/src/main.rs:main}}
```
이제 바이너리를 빌드, 푸시, 실행할 수 있습니다:
> You can now build, push, and run the binary:

```shell
{{#include ../build_all.sh:hello_rust}}
Hello from Rust!
```
