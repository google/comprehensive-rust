# Lifetimes

빌려온 값은 _수명_ 을 갖습니다:

* 수명는 생략할 수 있습니다: `add(p1: &Point, p2: &Point) -> Point`.
* 물론 명시할 수도 있습니다: `&'a Point`, `&'document str`.
* `&'a Point` 는 as `a`가 유효한 동안 빌려온 `Point` 입니다.
* 수명는 항상 컴파일러에 의해 추론됩니다.: 직접 수명을 설정할 수는 없습니다.
  * 수명 표기(`'`)은 제약조건을 생성합니다.
  * 컴파일러는 유효한 솔루션이 있는지 검증합니다.

> A borrowed value has a _lifetime_:
> 
> * The lifetime can be elided: `add(p1: &Point, p2: &Point) -> Point`.
> * Lifetimes can also be explicit: `&'a Point`, `&'document str`.
> * Read `&'a Point` as "a borrowed `Point` which is valid for at least the
>   lifetime `a`".
> * Lifetimes are always inferred by the compiler: you cannot assign a lifetime
>   yourself.
>  * Lifetime annotations create constraints; the compiler verifies that there is
>     a valid solution.

---
역주
- 유효한 솔루션이 어떤의미인지 모호한데 일단 매개변수와 리턴값은 모두 같은 수명이어야 한다고 합니다.