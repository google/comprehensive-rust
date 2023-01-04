# Examples

## `Send + Sync`

대부분의 타입은 `Send + Sync`입니다:
* `i8`, `f32`, `bool`, `char`, `&str`, ...
* `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, ...
* `String`, `Option<T>`, `Vec<T>`, `Box<T>`, ...
* `Arc<T>`: atomic 참조 카운트로 명시적 스레드-세이프.
* `Mutex<T>`: 내부 잠금을 통해 명시적 스레드-세이프.
* `AtomicBool`, `AtomicU8`, ...: 특별한 atomic 명령을 사용합니다.

> Most types you come across are `Send + Sync`:
> 
> * `i8`, `f32`, `bool`, `char`, `&str`, ...
> * `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, ...
> * `String`, `Option<T>`, `Vec<T>`, `Box<T>`, ...
> * `Arc<T>`: Explicitly thread-safe via atomic reference count.
> * `Mutex<T>`: Explicitly thread-safe via internal locking.
> * `AtomicBool`, `AtomicU8`, ...: Uses special atomic instructions.

제너릭 타입은 일반적으로 타입파라메터가 `Send + Sync`이면 `Send + Sync` 입니다.
> The generic types are typically `Send + Sync` when the type parameters are
> `Send + Sync`.

## `Send + !Sync`

아래 타입들은 일반적으로 내부 가변성으로 인해 다른 스레드로 이동될 수 있지만 스레드-세이프 하지 않습니다.
> These types can be moved to other threads, but they're not thread-safe.
> Typically because of interior mutability:

* `mpsc::Sender<T>`
* `mpsc::Receiver<T>`
* `Cell<T>`
* `RefCell<T>`

## `!Send + Sync`

아래 타입들은 스레드-세이프 하지만 다른 스레드로 이동될 수 없습니다:
* `MutexGuard<T>`: OS레벨에서 생성한 스레드에서 할당해제해야합니다
> These types are thread-safe, but they cannot be moved to another thread:
> 
> * `MutexGuard<T>`: Uses OS level primitives which must be deallocated on the
>   thread which created them.

## `!Send + !Sync`
아래 타입들은 스레드-세이프 하지도 않고 다른 스레드로 이동될 수도 없습니다:
* `Rc<T>`: 각 `Rc<T>` 는 비 atomic 참조 카운트를 포함하는 `RcBox<T>`를 참조합니다.
* `*const T`, `*mut T`: 러스트는 raw 포인터가 특별한 동시성 고려사항을 가질 수 있다고 가정합니다.
> These types are not thread-safe and cannot be moved to other threads:
> 
> * `Rc<T>`: each `Rc<T>` has a reference to an `RcBox<T>`, which contains a
>   non-atomic reference count.
> * `*const T`, `*mut T`: Rust assumes raw pointers may have special
>   concurrency considerations.
