# Parcelables

Binder for Rust supports sending parcelables directly:

_birthday_service/aidl/com/example/birthdayservice/BirthdayInfo.aidl_:

```java
{{#include ../birthday_service/aidl/com/example/birthdayservice/BirthdayInfo.aidl}}
```

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
import com.example.birthdayservice.BirthdayInfo;

interface IBirthdayService {
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:with_info}}
}
```

_birthday_service/src/client.rs_:

```rust,ignore
fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");

    let info = BirthdayInfo { name: "Alice".into(), years: 123 };
    service.wishWithInfo(&info)?;
}
```
