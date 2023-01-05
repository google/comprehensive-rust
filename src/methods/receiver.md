# Method Receiver

`&self`는 메서드가 객체를 불변하게 빌려옴을 나타냅니다. 그 외에 메서드가 받아올 수 있는 인자는 다음과 같습니다:
* `&self`: 호출자로부터 불변참조객체를 빌려옴을 나타냅니다. 객체는 나중에 다시 사용 될 수 있습니다.
* `&mut self`: 호출자로부터 가변참조객체를 빌려옴을 나타냅니다. 객체는 나중에 다시 사용 될 수 있습니다.
* `self`: 호출자의 객체가 이동되어 메서드가 객체의 소유권을 가져옵니다. 따라서 명시적으로 소유권을 반환하지 않으면 (메서드 종료 후)객체는 삭제(해제)됩니다.
* 인자 없음: 구조체의 정적 메서드가 됩니다. 관습적으로 `new`라고 지정되는 생성자를 만들때 사용합니다. 

`self`에 대한 variants ㅇ외에도 `Box<Self>`와 같이 수신자 유형으로 허용되는 [special wrapper types](https://doc.rust-lang.org/reference/special-types-and-traits.html)도 있습니다.

> The `&self` above indicates that the method borrows the object immutably. There
> are other possible receivers for a method:
> * `&self`: borrows the object from the caller using a shared and immutable
  reference. The object can be used again afterwards.
> * `&mut self`: borrows the object from the caller using a unique and mutable
  reference. The object can be used again afterwards.
* `self`: takes ownership of the object and moves it away from the caller. The
  method becomes the owner of the object. The object will be dropped (deallocated)
  when the method returns, unless its ownership is explicitly
  transmitted.
> * No receiver: this becomes a static method on the struct. Typically used to
  create constructors which are called `new` by convention.

Beyond variants on `self`, there are also
[special wrapper types](https://doc.rust-lang.org/reference/special-types-and-traits.html)
allowed to be receiver types, such as `Box<Self>`.
