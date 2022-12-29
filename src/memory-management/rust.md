# Memory Management in Rust

러스트의 메모리 관리는 이전 방식들을 혼합해서 사용합니다:
* 자바처럼 안전하고 정확합니다. 하지만 GC는 없습니다. 
* C++처럼 범위(스코프)기반입니다. 하지만 컴파일러가 엄격하게 적용합니다.
* C/C++처럼 런타임 오버헤드[^역주1]가 없습니다. 

이는 러스트의 명시적인 `소유권` 설계를 통해 이뤄집니다.

> Memory management in Rust is a mix:
> 
> * Safe and correct like Java, but without a garbage collector.
> * Scope-based like C++, but the compiler enforces full adherence.
> * Has no runtime overhead like in C and C++.
> 
> It achieves this by modeling _ownership_ explicitly.

---
역주

[^역주1]: 런타임에서 GC가 동작시 부하가 발생합니다. JAVA의 경우 종종 프로그램이 프리즈가 되는 것 처럼 보이는 
~~[더 월드](https://namu.wiki/w/%EB%8D%94%20%EC%9B%94%EB%93%9C(%EC%A3%A0%EC%A3%A0%EC%9D%98%20%EA%B8%B0%EB%AC%98%ED%95%9C%20%EB%AA%A8%ED%97%98)#s-3.2)~~ 현상이 있습니다.