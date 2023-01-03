# Shared State

러스트는 주로 아래 두 가지 타입 시스템을 이용해서 공유 데이터 동기화를 수행합니다: 
> Rust uses the type system to enforce synchronization of shared data. This is
> primarily done via two types:

* [`Arc<T>`][1], 최소단위 참조 카운트 `T`: 스레드 사이의 공유를 담당하고, 마지막 스레드 종료시 T를 해제합니다.
* [`Mutex<T>`][2]: `T`값에 대한 상호배제 엑세스를 보장합니다.
> * [`Arc<T>`][1], atomic reference counted `T`: handled sharing between threads and
>   takes care to deallocate `T` when the last thread exits,
> * [`Mutex<T>`][2]: ensures mutual exclusion access to the `T` value.

[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[2]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
