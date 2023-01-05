# Comparison

메모리 관리 기술의 대략적인 비교입니다.
> Here is a rough comparison of the memory management techniques.

## Pros of Different Memory Management Techniques

* C와 같은 수동 관리: 
  * 런타임 오버헤드가 없음. 
* JAVA와 같은 자동화 관리: 
  * 완전한 자동화.
  * 안전하고 정확함.
* C++과 같은 범위 기반 관리: 
  * 부분 자동화
  * 런타임 오버헤드가 없음.
* 러스트와 같은 컴파일러 수행 범위 기반 관리: 
  * 컴파일러에 의해 수행됩니다.
  * 런타임 오버헤드가 없습니다. 
  * 안전하고 정확합니다. 

> * Manual like C:
>   * No runtime overhead.
> * Automatic like Java:
>   * Fully automatic.
>   * Safe and correct.
> * Scope-based like C++:
>   * Partially automatic.
>   * No runtime overhead.
> * Compiler-enforced scope-based like Rust:
>   * Enforced by compiler.
>   * No runtime overhead.
>   * Safe and correct.

## Cons of Different Memory Management Techniques

* Manual like C:
  * Use-after-free.
  * Double-frees.
  * Memory leaks.
* Automatic like Java:
  * Garbage collection pauses.
  * Destructor delays.
* Scope-based like C++:
  * Complex, opt-in by programmer.
  * Potential for use-after-free.
* Compiler-enforced and scope-based like Rust:
  * Some upfront complexity.
  * Can reject valid programs.
