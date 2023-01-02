# Standard Library

러스트는 러스트 라이브러리와 프로그램에서 사용되는 공통 타입을 설정하는 데 도움을 주는 표준 라이브러리와 함께 제공 됩니다. 이렇게 하면 두 라이브러리 모두 같은 `String` 타입을 사용하기 때문에 원활하게 함께 작동할 수 있습니다.

> Rust comes with a standard library which helps establish a set of common types
> used by Rust library and programs. This way, two libraries can work together
> smoothly because they both use the same `String` type.

일반적인 타입은 아래와 같습니다:
> The common vocabulary types include:

* [`Option` and `Result`](std/option-result.md) 타입: 선택적 옵션 값과 [error handling](error-handling.md)에 사용됩니다.
* [`String`](std/string.md): 소유 데이터에서 사용되는 기본적인 문자열 타입입니다.
* [`Vec`](std/vec.md): 확장가능한 표준 벡터 타입입니다.
* [`HashMap`](std/hashmap.md): 구성 가능한 해시 알고리즘을 가지는 해쉬 맵 타입입니다
* [`Box`](std/box.md): 힙에 할당된 데이터에 대한 소유 포인터입니다.
* [`Rc`](std/rc.md): 힙에 할당된 데이터에 대한 참조 카운팅 공유 포인트입니다.

> * [`Option` and `Result`](std/option-result.md) types: used for optional values
  and [error handling](error-handling.md).
> * [`String`](std/string.md): the default string type used for owned data.
> * [`Vec`](std/vec.md): a standard extensible vector.
> * [`HashMap`](std/hashmap.md): a hash map type with a configurable hashing
>   algorithm.
> * [`Box`](std/box.md): an owned pointer for heap-allocated data.
> * [`Rc`](std/rc.md): a shared reference-counted pointer for heap-allocated data.
