# Sending Files

Files can be sent between Binder clients/servers using the
`ParcelFileDescriptor` type:

```java
interface IBirthdayService {
{{#include ../birthday-service/birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:with_file}}
}
```

**birthday_service/src/client.rs**:

```rust,ignore
fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");
{{#include ../birthday-service/birthday_service/src/client.rs:wish_with_file}}
}
```

<details>

* `ParcelFileDescriptor` can be created from a regular `std::fs::File`.

</details>
