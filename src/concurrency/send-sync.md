# `Send` and `Sync`

러스트는 아래 두가지 트레이트를 이용하여 스레드 간 공유 접근을 금지된 것을 알 수 있습니다:

* [`Send`][1]: a type `T` is `Send` if it is safe to move a `T` across a thread
  boundary.
* [`Sync`][2]: a type `T` is `Sync` if it is safe to move a `&T` across a thread
  boundary.

> How does Rust know to forbid shared access across thread? The answer is in two traits:
> 
> * [`Send`][1]: a type `T` is `Send` if it is safe to move a `T` across a thread
>   boundary.
> * [`Sync`][2]: a type `T` is `Sync` if it is safe to move a `&T` across a thread
>   boundary.

[1]: https://doc.rust-lang.org/std/marker/trait.Send.html
[2]: https://doc.rust-lang.org/std/marker/trait.Sync.html
