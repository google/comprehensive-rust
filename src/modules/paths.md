# Paths

경로는 아래와 같이 구분합니다:

1. 상대경로
  *  `foo` 또는 `self::foo`는 `foo`모듈 안에서 현재 모듈을 가리킵니다.
  * `super::foo`는 `foo`모듈의 부모 모듈을 가리킵니다.

2. 절대 경로
  * `crate::foo`는 현재 크레이트 루트의 있는 `foo`를 가리킵니다.
  * `bar::foo`는 `bar`크레이트 안에 있는 `foo`를 가리킵니다.
> Paths are resolved as follows:
> 
> 1. As a relative path:
>    * `foo` or `self::foo` refers to `foo` in the current module,
>    * `super::foo` refers to `foo` in the parent module.
> 
> 2. As an absolute path:
>    * `crate::foo` refers to `foo` in the root of the current crate,
>    * `bar::foo` refers to `foo` in the `bar` crate.

---
역주
- crate: 러스트의 기본 패키지 혹은 라이브러리의 단위입니다. std를 기본 라이브러리 / 기본 크레이트 라고 합니다. 