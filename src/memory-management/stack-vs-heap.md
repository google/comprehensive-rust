# The Stack vs The Heap

* 스택: 지역변수를 위한 메모리 상 연속적인 영역
  * 값은 사전 정의된 고정 크기를 갖습니다. 
  * 극도로 빠름: 단지 스택 포인트만 이동됩니다. 
  * 관리가 쉬움: 함수 호출을 따릅니다.
  * 메모리 인접성

* 힙: 함수 호출 외부의 값이 저장되는 곳
  * 값은 런타임 시 결정되는 동적 크기를 갖습니다. 
  * 스택에 비해서는 약간 느림: 약간의 추가 기록(부기)[^역주1]이 필요함.
  * 메모리 인접성을 보장하지 않음.

> * Stack: Continuous area of memory for local variables.
>   * Values have fixed sizes known at compile time.
>   * Extremely fast: just move a stack pointer.
>   * Easy to manage: follows function calls.
>   * Great memory locality.
> 
> * Heap: Storage of values outside of function calls.
>   * Values have dynamic sizes determined at runtime.
>   * Slightly slower than the stack: some book-keeping needed.
>   * No guarantee of memory locality.

---
역주

[^역주1]: book-keeping(부기)는 회계쪽 용어이고 어떠한 사건에 대해 요약, 정리해서 별도의 장부에 기록하는 것입니다. 
여기서는 데이터를 힙에 저장하고 해당 힙 주소를 스택에 저장하는 형태에 대한 설명입니다.