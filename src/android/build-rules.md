# Build Rules
안드로이드 빌드 시스템(Soong)은 다음과 같은 여러 모듈을 통해 러스트를 지원한다:

| Module Type       | Description                                                                                        |
|-------------------|----------------------------------------------------------------------------------------------------|
| `rust_binary`     | 러스트 바이너리(Rust binary)를 생성합니다.                                                         |
| `rust_library`    | 러스트 라이브러리를 생성하고, `rlib`와 `dylib` variants를 제공합니다.                              |
| `rust_ffi`        | `cc`모듈에서 사용할 수 있는 Rust C library를 생성하고, static and shared variants를 제공합니다.    |
| `rust_proc_macro` | `proc-macro` Rust library를 생성합니다. 컴파일러 플러그인과 유사합니다.                            |
| `rust_test`       | standard Rust test harness를 사용하는 러스트 테스트 바이너리를 생성합니다.                         |
| `rust_fuzz`       | `libfuzzer`를 사용하여 러스트 fuzz 바이너리를 생성합니다.                                          |
| `rust_protobuf`   | 소스를 생성하고 특정 protobuf에 대한 인터페이스를 제공하는 러스트 라이브러리를 생성합니다.         |
| `rust_bindgen`    | 소스를 생성하고 C 라이브러리에 대한 러스트 바인딩을 포함하는 러스트 라이브러리를 생성합니다        |

다음은 `rust_binary`와 `rust_library`를 살펴봅니다.

> The Android build system (Soong) supports Rust via a number of modules:
> 
> | Module Type       | Description                                                                                        |
> |-------------------|----------------------------------------------------------------------------------------------------|
> | `rust_binary`     | Produces a Rust binary.                                                                            |
> | `rust_library`    | Produces a Rust library, and provides both `rlib` and `dylib` variants.                            |
> | `rust_ffi`        | Produces a Rust C library usable by `cc` modules, and provides both static and shared variants.    |
> | `rust_proc_macro` | Produces a `proc-macro` Rust library. These are analogous to compiler plugins.                     |
> | `rust_test`       | Produces a Rust test binary that uses the standard Rust test harness.                              |
> | `rust_fuzz`       | Produces a Rust fuzz binary leveraging `libfuzzer`.                                                |
> | `rust_protobuf`   | Generates source and produces a Rust library that provides an interface for a particular protobuf. |
> | `rust_bindgen`    | Generates source and produces a Rust library containing Rust bindings to C libraries.              |
> 
> We will look at `rust_binary` and `rust_library` next.
