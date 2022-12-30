# Pattern Matching

`match`키워드는 C/C++의 `switch`와 유사하게 하나 이상의 패턴과 일치 시킬 수 있습니다.

비교동작은 위에서 아래로 진행되며 처음 일치하는 패턴이 실행됩니다. 

> The `match` keyword let you match a value against one or more _patterns_. The
> comparisons are done from top to bottom and the first match wins.
> 
> The patterns can be simple values, similarly to `switch` in C and C++:

```rust,editable
fn main() {
    let input = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
}
```

`_`패턴은 어떤 패턴과도 매칭되는 와일드 카드입니다.(default)
> The `_` pattern is a wildcard pattern which matches any value.
