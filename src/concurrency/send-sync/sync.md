# `Sync`

> `&T`가 여러 스레드에서 접근이 안전하다면, `&T`의 타입은 `Sync`입니다.
>> A type `T` is [`Sync`][1] if it is safe to access a `T` value from multiple
>> threads at the same time.

좀 더 정확한 정의는 다음과 같습니다:
> More precisely, the definition is:

`&T`만 `Send`인 경우, `T`의 타입은 `Sync`입니다.
> > `T` is `Sync` if and only if `&T` is `Send`

[1]: https://doc.rust-lang.org/std/marker/trait.Sync.html

