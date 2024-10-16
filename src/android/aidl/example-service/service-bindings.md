# Generated Service API

Binder generates a trait for each interface definition.

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:IBirthdayService}}
}
```

_out/soong/.intermediates/.../birthdayservice/IBirthdayService.rs_:

<!-- dprint-ignore-start -->

```rust,ignore
pub trait IBirthdayService: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "com.example.birthdayservice.IBirthdayService" }
  fn r#wishHappyBirthday(&self, _arg_name: &str, _arg_years: i32) -> binder::Result<String>;
  // ... more methods ...
}
```

<!-- dprint-ignore-end -->

Your service will need to implement this trait, and your client will use this
trait to talk to the service.

<details>

- Point out how the generated function signature, specifically the argument and
  return types, correspond the interface definition.
  - `String` for an argument results in a different Rust type than `String` as a
    return type.

</details>
