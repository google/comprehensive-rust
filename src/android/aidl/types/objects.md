<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Sending Objects

AIDL objects can be sent either as a concrete AIDL type or as the type-erased
`IBinder` interface:

_birthday_service/aidl/com/example/birthdayservice/IBirthdayInfoProvider.aidl_:

```java
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayInfoProvider.aidl:IBirthdayInfoProvider}}
```

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
import com.example.birthdayservice.IBirthdayInfoProvider;

interface IBirthdayService {
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:with_info_provider}}
}
```

_birthday_service/src/client.rs_:

```rust,ignore
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../birthday_service/src/client.rs:InfoProvider}}

fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("Failed to connect to BirthdayService");
{{#include ../birthday_service/src/client.rs:wish_with_provider}}
}
```

<details>

- Note the usage of `BnBirthdayInfoProvider`. This serves the same purpose as
  `BnBirthdayService` that we saw previously.

</details>
