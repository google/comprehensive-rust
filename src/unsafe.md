# Unsafe Rust

러스트의 문법은 크게 두가지 부분으로 나뉩니다: 

* **안전한 러스트:** 안전한 메모리, 정의되지 않은 동작 가능성 없음.
* **안전하지 않은 러스트:** 사전 조건을 위반하는 경우 정의되지 않은 동작을 수행할 수 있습니다.

We will be seeing mostly safe Rust in this course, but it's important to know
what unsafe Rust is.

안전하지 않은 러스트는 다음과 같은 5가지 새로운 기능을 제공합니다:
* 원시포인트 참조 해제
* 정적 가변변수 접근 및 수정
* `union` 필드 접근
* `extern`함수를 포함한 `unsafe` 함수 호출
* `unsafe` 트레이트 구현

* Dereference raw pointers.
* Access or modify mutable static variables.
* Access `union` fields.
* Call `unsafe` functions, including `extern` functions
* Implement `unsafe` traits.

