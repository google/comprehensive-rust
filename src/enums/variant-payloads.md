# Variant Payloads

다양한 데이터를 가진 `variants`를 열거형에 정의하고, `match` 구문을 이용해 이를 추출할 수 있습니다:
> You can define richer enums where the variants carry data. You can then use the
> `match` statement to extract the data from each variant:

```rust,editable
{{#include ../../third_party/rust-by-example/webevent.rs}}
```
--- 
역주
- 열거형 요소에 값을 포함하도록(예시) 선언해서 match 문에서 열거형의 타입에 따라 동작하도록 만들어진 예시입니다