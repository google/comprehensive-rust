# `From` and `Into`

타입은 용이한 형변환을 위해 `From`과 `Into`를 구현합니다:
> Types implement `From` and `Into` to facilitate type conversions:

```rust,editable
fn main() {
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123i16);
    println!("{s}, {addr}, {one}, {bigger}");
}
```
`From`이 구현되면 `Into` 역시 자동으로 구현됩니다:
> `Into` is automatically implemented when `From` is implemented:

```rust,editable
fn main() {
    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = true.into();
    let bigger: i32 = 123i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}
```
