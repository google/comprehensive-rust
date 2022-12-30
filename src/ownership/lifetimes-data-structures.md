# Lifetimes in Data Structures

만약 빌려온 데이터를 저장하는 타입(구조체 같은)인 경우, 반드시 수명 표기를 해야합니다.
> If a data type stores borrowed data, it must be annotated with a lifetime:

```rust,editable
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}
```

---
역주
- Highlight 구조체는 참조문자열(&str)을 저장합니다. 아래 출력문에서 text를 출력해도 동일하게 전부 출력됩니다. 
    - 하지만 중간에 텍스트를 erase로 삭제하려고 하면 빌린(참조)가 설정되 있어서 소유권 이동이 불가하다는 에러가 발생합니다.
    - try it!