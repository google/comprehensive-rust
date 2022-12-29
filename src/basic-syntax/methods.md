# Methods

러스트는 메서드(특정 타입과 관계된 간단한 함수)를 가집니다. 
메서드의 첫번째 인수는 호출부 타입의 인스턴스 입니다.(self, this)
> Rust has methods, they are simply functions that are associated with a particular type. The
> first argument of a method is an instance of the type it is associated with:

```rust,editable
// 구조체 선언입니다.
struct Rectangle {
    width: u32,
    height: u32,
}

// 구조체에 구현된 메서드 들입니다. 
// 구조체 자체에는 데이터만 선언이 되어 impl 키워드로 해당 구조체에 메서드를 선언/연결 해주는 문법 구조입니다.
// 첫번째 인수는 해당 구조체 자신(self)입니다.  
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    // 메서드는 .으로 호출하며 area의 self.width는 10, self.height는 5입니다.
    println!("old area: {}", rect.area()); 
    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
```

* 오늘과 내일 강의에서 더 많은 메서드 사용법을 다룰 것입니다.
> * We will look much more at methods in today's exercise and in tomorrow's class.
