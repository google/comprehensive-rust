# A Simple GUI Library

트레이트와 트레이트 객체 지식을 활용하여 고전적인 GUI 라이브러리를 설계합니다. 
라이브러리에는 몇가지 위젯이 있습니다: 
* `Window`: `title` 속성과 다른 위젯이 포함됩니다.
* `Button`: `label`위젯과 버튼이 눌렸을때 실행되는 콜백 함수가 있습니다.
* `Label`: `label` 위젯 입니다.
위젯은 `Widget` 트레이트를 구현합니다. 아래 코드를 참조하세요

아래 코드를 <https://play.rust-lang.org/>에 복사하고 누락된 `draw_into`메서드를 채워 넣어 `Widget` 트레이트를 완성해봅시다:
> Let us design a classical GUI library using our new knowledge of traits and
trait objects.
> We will have a number of widgets in our library:
> * `Window`: has a `title` and contains other widgets.
> * `Button`: has a `label` and a callback function which is invoked when the
  button is pressed.
> * `Label`: has a `label`.
> 
> The widgets will implement a `Widget` trait, see below.  
> Copy the code below to <https://play.rust-lang.org/>, fill in the missing
> `draw_into` methods so that you implement the `Widget` trait:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

{{#include simple-gui.rs:setup}}

{{#include simple-gui.rs:Label-width}}
        unimplemented!()
    }

{{#include simple-gui.rs:Label-draw_into}}
        unimplemented!()
    }
}

{{#include simple-gui.rs:Button-width}}
        unimplemented!()
    }

{{#include simple-gui.rs:Button-draw_into}}
        unimplemented!()
    }
}

{{#include simple-gui.rs:Window-width}}
        unimplemented!()
    }

{{#include simple-gui.rs:Window-draw_into}}
        unimplemented!()
    }
}

{{#include simple-gui.rs:main}}
```

위 프로그램의 출력은 아래와 같습니다:
> The output of the above program can be something simple like this:

```text
========
Rust GUI Demo 1.23
========

This is a small text GUI demo.

| Click me! |
```

정력된 글자를 그리기 위해서는 
[fill/alignment](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
를 참조하시기 바랍니다. 
특수 문자(여기서는 '/')로 패딩을 주는 방법과 정렬을 제어하는 방법을 확인하시기 바랍니다.
> If you want to draw aligned text, you can use the
> [fill/alignment](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
> formatting operators. In particular, notice how you can pad with different
> characters (here a `'/'`) and how you can control alignment:

```rust,editable
fn main() {
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
}
```
위의 정렬 트릭을 사용하여 다음과 같은 출력을 생성할 수 있습니다.
> Using such alignment tricks, you can for example produce output like this:

```text
+--------------------------------+
|       Rust GUI Demo 1.23       |
+================================+
| This is a small text GUI demo. |
| +-----------+                  |
| | Click me! |                  |
| +-----------+                  |
+--------------------------------+
```

---
역주
- 추가 구현할 함수는 없고 `impl Widget for ~` 의 두 메서드들만 구현하면 됩니다.
- label에 줄바꿈(개행)문자도 포함될 수 있습니다.`\n`