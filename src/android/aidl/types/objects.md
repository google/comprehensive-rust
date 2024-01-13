# Sending Objects

AIDL objects can be sent either as a concrete AIDL type or as the type-erased
`IBinder` interface:

**birthday_service/aidl/com/example/birthdayservice/IBirthdayInfoProvider.aidl**:

```java
{{#include ../birthday-service/birthday_service/aidl/com/example/birthdayservice/IBirthdayInfoProvider.aidl:IBirthdayInfoProvider}}
```

**birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl**:

```java
import com.example.birthdayservice.IBirthdayInfoProvider;

interface IBirthdayService {
{{#include ../birthday-service/birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:with_info_provider}}
}
```

**birthday_service/src/client.rs**:

```rust,ignore
{{#include ../birthday-service/birthday_service/src/client.rs:InfoProvider}}

fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");
{{#include ../birthday-service/birthday_service/src/client.rs:wish_with_provider}}
}
```

<details>

* Note the usage of `BnBirthdayInfoProvider`. This serves the same purpose as
  `BnBirthdayService` that we saw previously.

</details>
