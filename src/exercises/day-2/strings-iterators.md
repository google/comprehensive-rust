# Strings and Iterators

이번 훈련은 웹 서버의 라우팅 컴포넌트를 구현합니다. 
서버는 _요청경로(request path)_와 일치하는 여러개의 _경로 접두사(path prefixes)_로 구성되어 있습니다. 경로 접두사에는 와일드카드문자를 포함할 수 있습니다.  아래 테스트 코드를 참조하세요
> In this exercise, you are implementing a routing component of a web server. The
> server is configured with a number of _path prefixes_ which are matched against
> _request paths_. The path prefixes can contain a wildcard character which
> matches a full segment. See the unit tests below.


아래 코드를 <https://play.rust-lang.org/>에 복사하고 테스트를 통과해 보시기 바랍니다. 중간 결과값을 `Vec`에 할당하지 않도록 주의 하시기 바랍니다.
> Copy the following code to <https://play.rust-lang.org/> and make the tests
> pass. Try avoiding allocating a `Vec` for your intermediate results:


```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include strings-iterators.rs:prefix_matches}}
    unimplemented!()
}

{{#include strings-iterators.rs:unit-tests}}
```

---
역주
- 단순 파싱인데 제약조건에 주의 하세요. (Vec 사용금지)
- match 활용인데 개인적으론 좀 낮선 방법이라 익숙해질 필요가 있습니다.
<details>
<summary>힌트</summary>

- [iter 공식 문서 참조](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- solution은 요소를 Option으로 감싸고 마지막은 None으로 두고 match문을 사용하는 로직입니다.
</details>
