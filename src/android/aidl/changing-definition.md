# Changing API

Let us extend the API with more functionality: we want to let clients specify a
list of lines for the birthday card:

```java
package com.example.birthdayservice;

/** Birthday service interface. */
interface IBirthdayService {
    /** Generate a Happy Birthday message. */
    String wishHappyBirthday(String name, int years, in String[] text);
}
```

This results in an updated trait definition for `IBirthdayService`:

```rust
trait IBirthdayService {
    fn wishHappyBirthday(&self, name: &str, years: i32, text: &[String]) -> binder::Result<String>;
}
```

<details>

* Note how the `String[]` in the AIDL definition is translated as a `&[String]`
  in Rust, i.e. that idiomatic Rust types are used in the generated bindings
  wherever possible:
  * `in` array arguments are translated to slices.
  * `out` and `inout` args are translated to `&mut Vec<T>`.
  * Return values are translated to returning a `Vec<T>`.

</details>
