# Ownership

모든 변수 바인딩은 유효한 _범위_를 갖으며, 범위 밖에서 변수 사용은 오류입니다:
> All variable bindings have a _scope_ where they are valid and it is an error to
> use a variable outside its scope:

```rust,editable,compile_fail
struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    } // 스코프 종료지점
    println!("y: {}", p.1);
} // main 함수의 스코프 종료지점
```

* 스코프가 종료되면 변수는 _삭제_되고 데이터 메모리는 해제됩니다.
* 소멸자는 여기(스코프 종료지점)에서 메모리 자원을 해제 합니다. 
* 이것을 두고 변수가 값을 _소유_한다고 표현합니다.

> * At the end of the scope, the variable is _dropped_ and the data is freed.
> * A destructor can run here to free up resources.
> * We say that the variable _owns_ the value.
