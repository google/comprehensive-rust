# Arrays and `for` Loops

우리는 배열을 아래와 같이 선언 할 수 있음을 보았습니다:
> We saw that an array can be declared like this:

```rust
let array = [10, 20, 30];
```
배열은 `{:?}` 형태로 표시하여 출력할 수 있습니다.
> You can print such an array by asking for its debug representation with `{:?}`:

```rust,editable
fn main() {
    let array = [10, 20, 30];
    println!("array: {array:?}");
}
```

러스트에서는 `for`키워드를 사용해 배열이나 범위를 반복할 수 있습니다:
> Rust lets you iterate over things like arrays and ranges using the `for` keyword:

```rust,editable
fn main() {
    let array = [10, 20, 30];
    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}
```

위의 예제에서 행렬을 예쁘게 출력하는 `pretty_print`함수와 행렬을 전치(행과 열을 변경)시키는 `transpose`함수를 작성해 보시기 바랍니다.
> Use the above to write a function `pretty_print` which pretty-print a matrix and
> a function `transpose` which will transpose a matrix (turn rows into columns):

```bob
           ⎛⎡1 2 3⎤⎞      ⎡1 4 7⎤
"transpose"⎜⎢4 5 6⎥⎟  "=="⎢2 5 8⎥
           ⎝⎣7 8 9⎦⎠      ⎣3 6 9⎦
```

두 함수 모두 3×3 행렬에서 작동하도록 하드 코딩합니다.
> Hard-code both functions to operate on 3 × 3 matrices.

아래 코드를 <https://play.rust-lang.org/>에 복사해서 구현하시면 됩니다.
> Copy the code below to <https://play.rust-lang.org/> and implement the functions:

```rust,should_panic
// TODO: 구현이 완료되면 아래 줄은 삭제합니다.
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include for-loops.rs:transpose}}
    unimplemented!()
}

{{#include for-loops.rs:pretty_print}}
    unimplemented!()
}

{{#include for-loops.rs:main}}
```

## Bonus Question

하드코딩된 3×3 행렬 입력을 `&[i32]` 슬라이스를 통해 인수와 반환값을 정의가 가능한가요?

예컨데 `&[&[i32]]`는 2차원 슬라이스의 슬라이스 입니다. 가능하다면/하지 않다면 왜 그런가요?
> Could you use `&[i32]` slices instead of hard-coded 3 × 3 matrices for your
> argument and return types? Something like `&[&[i32]]` for a two-dimensional
> slice-of-slices. Why or why not?

프로덕션 품질의 구현에 대해서는 [`ndarray` 크레이트](https://docs.rs/ndarray/)를 참조하시기 바랍니다.
> See the [`ndarray` crate](https://docs.rs/ndarray/) for a production quality
implementation.

---
역주
- C/C++나 js에 대해서 어느정도 지식이 있는걸 전제로 하기때문에 첫 문제부터 난이도가 조금 있지 않을까 싶습니다만..
<details>
<summary>힌트</summary>

  - for를 이용한 출력과 for를 이용해 [i][j] 변환하는 함수입니다.
</details>
