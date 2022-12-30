# Designing a Library

우리는 내일 구조체와 `Vec<T>`에 대해 더 많은 것을 배울 것입니다. 

일단 오늘은 API의 일부만 알면 됩니다:

> We will learn much more about structs and the `Vec<T>` type tomorrow. For now,
> you just need to know part of its API:

```rust,editable
fn main() {
    let mut vec = vec![10, 20];
    vec.push(30);
    println!("middle value: {}", vec[vec.len() / 2]);
    for item in vec.iter() {
        println!("item: {item}");
    }
}
```

도서관 프로그램을 만들기 위해 아래 코드를 <https://play.rust-lang.org/>에 복사해서 구현하시면 됩니다.
> Use this to create a library application. Copy the code below to
> <https://play.rust-lang.org/> and update the types to make it compile:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include book-library.rs:setup}}

{{#include book-library.rs:Library_new}}
        unimplemented!()
    }

{{#include book-library.rs:Library_len}}

{{#include book-library.rs:Library_is_empty}}

{{#include book-library.rs:Library_add_book}}

{{#include book-library.rs:Library_print_books}}

{{#include book-library.rs:Library_oldest_book}}
}

{{#include book-library.rs:main}}
```

---
역주
- 강의를 눈으로만 보고 있었다면 헬게이트가 될 수 있습니다. 
- 기본적으로 이 강의는 주어진 내용을 추가로 찾아보고 공식문서 보고 예제 이것저것 만져보고 해봐야 따라올 수 있습니다.
<details>
<summary>힌트</summary>

- 사실 대부분 컴파일러가 에러 내면서 이렇게 하세요 하니깐 난이도는 쉽습니다.
- 실제로 해보니 main에도 수정을 하도록 컴파일러가 워닝을 주네요... 
- 근데 공식문서에서 필요한 메서드들은 찾아서 풀어야 합니다. 물론 ide에서 띄워줄테지만 플레이그라운드에는 그런거 없음...
    - [벡터(Vec<T>)](https://doc.rust-lang.org/std/vec/struct.Vec.html) 보다는 [이터레이터(Iter())](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) 쪽을 좀 더 봐야할듯.
- 클로저(익명함수)를 써야 한다거나 해서 문법을 간략히 쓰면 `|param| param.item` 형식입니다. 
    - js에서는 `param => param.item` 에 해당합니다.

</details>
