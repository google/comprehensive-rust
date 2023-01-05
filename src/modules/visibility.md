# Visibility

모듈의 기본 접근자는 private 입니다:
* 모듈의 항목은 기본적으로 private 입니다.(구현에서 숨겨짐)
* 부모와 이웃 항목에서는 접근 가능합니다.

> Modules are a privacy boundary:
> * Module items are private by default (hides implementation details).
> * Parent and sibling items are always visible.

```rust,editable
mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn main() {
    outer::public();
}
```
