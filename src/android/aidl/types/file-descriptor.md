# Sending Files

Files can be sent between Binder clients/servers using the
`ParcelFileDescriptor` type:

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
interface IBirthdayService {
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:with_file}}
}
```

_birthday_service/src/client.rs_:

```rust,ignore
fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");
{{#include ../birthday_service/src/client.rs:wish_with_file}}
}
```

_birthday_service/src/lib.rs_:

```rust,ignore
impl IBirthdayService for BirthdayService {
{{#include ../birthday_service/src/lib.rs:wishFromFile}}
}
```

<details>

- `ParcelFileDescriptor` wraps an `OwnedFd`, and so can be created from a `File`
  (or any other type that wraps an `OwnedFd`), and can be used to create a new
  `File` handle on the other side.
- Other types of file descriptors can be wrapped and sent, e.g. TCP, UDP, and
  UNIX sockets.

</details>
