# Iterators and Ownership

러스트의 소유권 모델은 많은 API에 영향을 줍니다. 

예를들어 [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) 와 [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) 같은 트레이트가 있습니다.
> The ownership model of Rust affects many APIs. An example of this is the
> [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and
> [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) traits.

## `Iterator`

트레이트는 타입에 대한 행동(메서드)를 설명한다는 점에서 인터페이스와 유사합니다.

`Iterator`는 단순히 `None`가 나올때까지 `next`를 호출하는 트레이트입니다.

> Traits are like interfaces: they describe behavior (methods) for a type. The
> `Iterator` trait simply says that you can call `next` until you get `None` back:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

`Iterator`트레이트는 이렇게 사용합니다:
> You use this trait like this:

```rust,editable
fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}
```

질문) 반복자(이터레이터)가 반환하는 타입은 무엇입니까? 아래 코드를 수정해보세요:
> What is the type returned by the iterator? Test your answer here:

```rust,editable,compile_fail
fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    let v0: Option<..> = iter.next(); 
    println!("v0: {v0:?}");
}
```

왜 이런 것을 사용해야 합니까?
> Why is this type used?

## `IntoIterator`

`Iterator` 트레이트는 생성된 반복자를 "어떻게 _반복_ 하는지" 알려줍니다.

유사한 `IntoIterator` 트레이트는 "반복자를 어떻게 만드는지" 알려줍니다.
> The `Iterator` trait tells you how to _iterate_ once you have created an
> iterator. The related trait `IntoIterator` tells you how to create the iterator:

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

`IntoIterator`의 모든 구현은 반드시 두 타입을 선언해야합니다: 

* `Item`: `i8`과 같이 반복되는 요소의 유형
* `IntoIter`: `into_iter` 메서드에서 반환되는 `Iterator`타입

`IntoIter`와 `Item`는 링크되어 있습니다: 반복자와 `Item` 타입은 동일해야합니다. 즉, `Option<Item>`를 반환해야 합니다.

> The syntax here means that every implementation of `IntoIterator` must
> declare two types:
> * `Item`: the type we iterate over, such as `i8`,
> * `IntoIter`: the `Iterator` type returned by the `into_iter` method.
> Note that `IntoIter` and `Item` are linked: the iterator must have the same
> `Item` type, which means that it returns `Option<Item>`

질문) (이전과 마찬가지로) 반복자(이터레이터)가 반환하는 타입은 무엇입니까? 아래 코드를 수정해보세요:
Like before, what is the type returned by the iterator?

```rust,editable,compile_fail
fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<..> = iter.next();
    println!("v0: {v0:?}");
}
```

## `for` Loops

자, 이제 우리는 `Iterator`와 `IntoIterator`를 알았으므로 `for` 루프를 만들 수 있습니다. 
`for` 루프는 `into_iter()`를 호출하여 결과를 반환하는 것을 반복합니다:
> Now that we know both `Iterator` and `IntoIterator`, we can build `for` loops.
> They call `into_iter()` on an expression and iterates over the resulting
> iterator:

```rust,editable
fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }
}
```
질문) 매 루프에서 `word`의 타입은 무엇입니까? 
> What is the type of `word` in each loop?

위 코드에서 실험 해 본 후 다음 문서를 참조해서 답변을 확인하시기 바랍니다.
  * [`impl IntoIterator for &Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-%26%27a%20Vec%3CT%2C%20A%3E)
  * [`impl IntoIterator for Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-Vec%3CT%2C%20A%3E)
> Experiment with the code above and then consult the documentation for [`impl
> IntoIterator for
> &Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-2)
> and [`impl IntoIterator for Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-1)
> to check your answers.
