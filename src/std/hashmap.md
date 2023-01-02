# `HashMap`

HashDoS[^역주1] 공격으로부터 보호되는 표준 해시 맵입니다.
> Standard hash map with protection against HashDoS attacks:

```rust,editable
use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!("We've know about {} books, but not Les Misérables.",
                 page_counts.len());
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown.")
        }
    }
}
```

---
역주

[^역주1]: Hash table을 사용하는 웹서버에 파라미터가 많은 POST를 호출하여 Hash table 충돌을 유도하여 CPU 부하를 발생시키는 공격 방법. 
    - POST, GET 요청의 파라메터의 빠른 접근을 위해 웹서버는 파라메터를 Hash table로 관리하는데, POST 요청시 전달할 수 있는 파라메터의 수의 제한이 없다는 점을 이용한 공격 방법