# Slices

슬라이스를 사용하면 큰 컬렉션의 부분만 확인할 수 있습니다. 
> A slice gives you a view into a larger collection:

```rust,editable
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}
```

* 슬라이스는 슬라이스 타입으로부터 데이터를 '빌려'옵니다. 
* 질문: `a[3]`으로 수정하면 무슨 일이 있어날까요?
* Slices borrow data from the sliced type.
* Question: What happens if you modify `a[3]`?
