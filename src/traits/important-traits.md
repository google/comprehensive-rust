# Important Traits

러스트 표준 라이브러리의 자주 사용하는 몇가지 트레이트 예시입니다:
* `Iterator`와 `IntoIterator` 트레이트는 `for` 루프에서 사용됩니다.
* `From`과 `Into` 트레이트는 타입변환시 사용됩니다.
* `Read`와 `Write` 트레이트는 I/O에 사용됩니다.
* `Add`, `Mul` 등의 트레이트들은 연산자 오버로딩(overloading)에 사용됩니다.
* `Drop` 트레이트는 소멸자 정의에 사용됩니다.
> We will now look at some of the most common traits of the Rust standard library:
> * `Iterator` and `IntoIterator` used in `for` loops,
> * `From` and `Into` used to convert values,
> * `Read` and `Write` used for IO,
> * `Add`, `Mul`, ... used for operator overloading, and
> * `Drop` used for defining destructors.
