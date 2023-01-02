# Trait Objects

트레이트를 구현할 때 인수를 취하는 방법:
> We've seen how a function can take arguments which implement a trait:

```rust,editable
use std::fmt::Display;

fn print<T: Display>(x: T) {
    println!("Your value: {}", x);
}

fn main() {
    print(123);
    print("Hello");
}
```
아래와 같이 여러가지 타입을 혼합하여 인수로 받려면 어떻게 해야 합니까?
> However, how can we store a collection of mixed types which implement `Display`?

```rust,editable,compile_fail
fn main() {
    let xs = vec![123, "Hello"];
}
```

이를 위해서 _트레이트 객체_가 필요합니다:
> For this, we need _trait objects_:

```rust,editable
use std::fmt::Display;

fn main() {
    let xs: Vec<Box<dyn Display>> = vec![Box::new(123), Box::new("Hello")];
    for x in xs {
        println!("x: {x}");
    }
}
```

`xs`가 할당될때 메모리 레이아웃:
> Memory layout after allocating `xs`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - -.
:                           :     :                                               :
:    xs                     :     :                                               :
:   +-----------+-------+   :     :   +-----+-----+                               :
:   | ptr       |   o---+---+-----+-->| o o | o o |                               :
:   | len       |     2 |   :     :   +-|-|-+-|-|-+                               :
:   | capacity  |     2 |   :     :     | |   | |   +----+----+----+----+----+    :
:   +-----------+-------+   :     :     | |   | '-->| H  | e  | l  | l  | o  |    :
:                           :     :     | |   |     +----+----+----+----+----+    :
`- - - - - - - - - - - - - -'     :     | |   |                                   :
                                  :     | |   |     +-------------------------+   :
                                  :     | |   '---->| "<str as Display>::fmt" |   :
                                  :     | |         +-------------------------+   :
                                  :     | |                                       :
                                  :     | |   +-------------------------+         :
                                  :     | '-->| "<i32 as Display>::fmt" |         :
                                  :     |     +-------------------------+         :
                                  :     |                                         :
                                  :     |     +----+----+----+----+               :
                                  :     '---->| 7b | 00 | 00 | 00 |               :
                                  :           +----+----+----+----+               :
                                  :                                               :
                                  :                                               :
                                  '- - - - - - - - - - - - - - - - - - - - - - - -'
```

마찬가지로 트레이트를 구현한 다른 값을 반환할 때도 트레이트 객체가 필요합니다:
> Similarly, you need a trait object if you want to return different values
> implementing a trait:

```rust,editable
fn numbers(n: i32) -> Box<dyn Iterator<Item=i32>> {
    if n > 0 {
        Box::new(0..n)
    } else {
        Box::new((n..0).rev())
    }
}

fn main() {
    println!("{:?}", numbers(-5).collect::<Vec<_>>());
    println!("{:?}", numbers(5).collect::<Vec<_>>());
}

```
