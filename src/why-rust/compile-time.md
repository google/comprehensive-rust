# Compile Time Guarantees

컴파일 시 정적 메모리 관리:
> Static memory management at compile time:

* 초기화되지 않는 변수가 없습니다.
* *거의* 없는 메모리 누수[^leaks].
* 메모리 이중 해제는 안됩니다. 
* 메모리 해제 후 사용 안됩니다.
* `NULL`포인터는 없습니다.
* 잠긴 뮤텍스를 잊을 수 없습니다.
* 스레드간 데이터레이스가 없습니다. 
* 반복자(iterator) 무효화가 없습니다.
> * No uninitialized variables.
> * *Mostly* no memory leaks[^leaks].
> * No double-frees.
> * No use-after-free.
> * No `NULL` pointers.
> * No forgotten locked mutexes.
> * No data races between threads.
> * No iterator invalidation.

[^leaks]: 기술적으로는 (안전한) 러스트에서 메모리 누수를 만드는 것이 가능합니다. [`Box::leak`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak)메서드를 사용하면 destructor를 실행하지 않고 [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)의 원시 참조를 가져온 후 삭제할 수 있습니다. 이를 사용하면 런타임 초기화 및 런타임 크기의 정적 변수를 얻을 수 있습니다. 또는 단순히 'std:mem::forget' 함수를 사용하면 소멸자가 실행되지 않음을 의미하는 값에 대해 컴파일러가 "잊게"됩니다. 안전한 러스트에서 누출을 생성하는 다른 방법은 여러 가지가 있지만, 본 코스의 목적상 "메모리 누수 없음"은 "매우 *우발적인* 메모리 누수 없음"으로 이해해야 합니다.
> [^leaks]: It is technically possible to produce a memory leak in (safe) Rust. The [`Box::leak`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak) method allows getting a raw reference out of a [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) and dropping the [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) afterwards, without running the destructor. A use of this could be to get runtime-initialized and runtime-sized static variables. Or simply, the [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) function, which makes the compiler "forget" about a value meaning the destructor is never run. There are many other ways to create leaks in safe Rust, but for the purpose of this course, "No memory leaks" should be understood as "Pretty much no *accidental* memory leaks".

---
* mutexes : 멀티스레딩에서 자원을 선점한 작업자가 잠그면(lock) 다른 작업자들은 lock이 해제될때까지 자원에 대해 접근을 할 수 없도록 막는 방식. 
    * cf. semaphore: 자원에 접근 할 수 있는 작업자(스레드,프로세스)의 수를 나타내는 값을 둬서 상호 배제하는 방식.
