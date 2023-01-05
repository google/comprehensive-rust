# Hello World!

가장 간단한 러스트 프로그램으로 고전적인 Hello World 출력 프로그램을 작성해 보겠습니다.
> Let us jump into the simplest possible Rust program, a classic Hello World
program:

```rust
fn main() {
    println!("Hello 🌍!");
}
```

당신이 볼 수 있는 것: 

* 함수는 `fn`으로 선언됩니다.
* C/C++과 마찬가지로 중괄호`{}`로 블록을 표시합니다. 
* `main` 함수는 프로그램 진입점입니다. 
* 러스트는 위생적인 매크로를 가지고 있습니다. `println!`는 그 예시입니다. 
* 러스트의 문자열은 UTF-8로 인코딩되며 이모지와 같은 유니코드 문자를 포함할 수 있습니다.
> What you see:
> * Functions are introduced with `fn`.
> * Blocks are delimited by curly braces like in C and C++.
> * The `main` function is the entry point of the program.
> * Rust has hygienic macros, `println!` is an example of this.
> * Rust strings are UTF-8 encoded and can contain any Unicode character.


---
역주
- 매크로: 다른 코드를 작성하는 코드입니다(meta-programming). 런타임이 아닌 컴파일 전에 대치 작업이 이뤄지는데 C계열의 #define 문법을 생각하시면 됩니다.
- 위생적인 매크로([hygienic macros, 위키](https://en.wikipedia.org/wiki/Hygienic_macro))는 식별자가 겹치지 않음이 보장되는 매크로... 라는데 일단은 그렇구나하고 진행


* Functions are introduced with `fn`.
* Blocks are delimited by curly braces like in C and C++.
* The `main` function is the entry point of the program.
* Rust has hygienic macros, `println!` is an example of this.
* Rust strings are UTF-8 encoded and can contain any Unicode character.

<details>

This slide tries to make the students comfortable with Rust code. They will see
a ton of it over the next four days so we start small with something familiar.

Key points:

* Rust is very much like other languages in the C/C++/Java tradition. It is
  imperative (not functional) and it doesn't try to reinvent things unless
  absolutely necessary.

* Rust is modern with full support for things like Unicode.

* Rust uses macros for situations where you want to have a variable number of
  arguments (no function [overloading](basic-syntax/functions-interlude.md)).

</details>
