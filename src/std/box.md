# `Box`

[`Box`][1]는 힙 데이터에 대한 소유 포인터입니다:
> [`Box`][1] is an owned pointer to data on the heap:

```rust,editable
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
```


```bob
     Stack               Heap
.- - - - - - -.     .- - - - - - -.
:             :     :             :
:    five     :     :             :
:   +-----+   :     :   +-----+   :
:   | o---|---+-----+-->|  5  |   :
:   +-----+   :     :   +-----+   :
:             :     :             :
:             :     :             :
`- - - - - - -'     `- - - - - - -'
```

`Box<T>`은 [`Deref<Target = T>`][2]를 구현합니다. 
이는 [`Box<T>`에서 T 관련 메서드를 직접 호출][2] 할 수 있다는 의미입니다.
> `Box<T>` implements `Deref<Target = T>`, which means that you can [call methods
> from `T` directly on a `Box<T>`][2].

[1]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[2]: https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion

---
역주
- 일종의 래핑 객체라서 타입을 담는 '상자' 라는 의미인듯 합니다.