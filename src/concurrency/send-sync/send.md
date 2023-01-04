# `Send`

> `T`가 스레드 간 이동이 안전하다면, `T`의 타입은 `Send`입니다.
> > A type `T` is [`Send`][1] if it is safe to move a `T` value to another thread.

소유권을 다른 스레드로 이동하면 소멸자가 해당 스레드에서 실행됩니다. 

여기서 의문은 언제 한 스레드에서 값을 할당하고 다른 스레드에서 값을 할당 해제할 수 있는가 입니다. 
> The effect of moving ownership to another thread is that _destructors_ will run
> in that thread. So the question is when you can allocate a value in one thread
> and deallocate it in another.

[1]: https://doc.rust-lang.org/std/marker/trait.Send.html
