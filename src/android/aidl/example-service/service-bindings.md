# Generated Service API

Binder generates a trait for each interface definition.

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:IBirthdayService}}
}
```

_out/soong/.intermediates/.../com_example_birthdayservice.rs_:

<!-- The example below is a cleaned up and simplified version of the real code. -->

```rust,ignore
trait IBirthdayService {
    fn wishHappyBirthday(&self, name: &str, years: i32) -> binder::Result<String>;
}
```

Your service will need to implement this trait, and your client will use this
trait to talk to the service.

<details>

- Point out how the generated function signature, specifically the argument and
  return types, correspond the interface definition.
  - `String` for an argument results in a different Rust type than `String` as a
    return type.

</details>
