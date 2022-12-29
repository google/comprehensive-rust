# Memory Management

전통적으로, 프로그래밍 언어는 크게 두가지로 분류 됩니다:

* 소스레벨 메모리 관리(수동 제어): C, C++, Pascal, ...
* 런타임 시 자동 메모리 관리를 통한 안전성 제공[^역주1]: Java, Python, Go, Haskell, ... 
> Traditionally, languages have fallen into two broad categories:
> 
> * Full control via manual memory management: C, C++, Pascal, ...
> * Full safety via automatic memory management at runtime: Java, Python, Go, Haskell, ...

러스트는 새로운 형태를 제안합니다: 
> 컴파일 시 메모리 관리를 통한 안전성과 제어 제공

> Rust offers a new mix:
> 
>> Full control *and* safety via compile time enforcement of correct memory
>> management.

이것은 명시적인 소유권 컨셉을 통해 이뤄집니다. 

우선 메모리 관리가 이뤄지는 방식을 다시 살펴 보겠습니다.
> It does this with an explicit ownership concept.
> First, let's refresh how memory management works.

---
[^역주1]: GC같은거 