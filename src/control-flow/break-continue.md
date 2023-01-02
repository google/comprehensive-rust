# `break` and `continue`

만약 루프를 중간에 멈추고 십다면 `break`를, 바로 다음 반복으로 넘어가기 위해서는 `continue`를 사용합니다. 두 키워드 모두 중첩 루프에서 label로 표기한 인수를 취하여 제어 할 수 있습니다.
> If you want to exit a loop early, use `break`, if you want to immediately start
> the next iteration use `continue`. Both `continue` and `break` can optionally
> take a label argument which is used to break out of nested loops:

```rust,editable
fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
```

위 예제는 중접루프를 3회 반복한 후 바깥루프('outer' 레이블)을 정지합니다.(종료)
> In this case we break the outer loop after 3 iterations of the inner loop.
