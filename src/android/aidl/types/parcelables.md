# Parcelables

Binder for Rust supports sending parcelables directly:

**birthday_service/aidl/com/example/birthdayservice/BirthdayInfo.aidl**:

```java
{{#include ../birthday-service/birthday_service/aidl/com/example/birthdayservice/BirthdayInfo.aidl}}
```

**birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl**:

```java
import com.example.birthdayservice.BirthdayInfo;

interface IBirthdayService {
{{#include ../birthday-service/birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:with_info}}
}
```

**client/src/main.rs**:

```rust,ignore
fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");

{{#include ../birthday-service/birthday_service/src/client.rs:wish_with_info}}
}
```

<details>

* Parcelables work similarly to serde types in the broader Rust ecosystem: You
  get a concrete Rust type to work with in your Rust code, and that type can be
  serializied without you needing to write any serialization logic yourself.

</details>
