<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# AIDL Client

Finally, we can create a Rust client for our new service.

_birthday_service/src/client.rs_:

```rust,ignore
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::IBirthdayService;
use com_example_birthdayservice::binder;

{{#include ../birthday_service/src/client.rs:main}}
}
```

_birthday_service/Android.bp_:

```javascript
{{#include ../birthday_service/Android.bp:birthday_client}}
```

Notice that the client does not depend on `libbirthdayservice`.

Build, push, and run the client on your device:

```shell
{{#include ../../build_all.sh:birthday_client}}
```

```text
Happy Birthday Charlie, congratulations with the 60 years!
```

<details>

- `Strong<dyn IBirthdayService>` is the trait object representing the service
  that the client has connected to.
  - `Strong` is a custom smart pointer type for Binder. It handles both an
    in-process ref count for the service trait object, and the global Binder ref
    count that tracks how many processes have a reference to the object.
  - Note that the trait object that the client uses to talk to the service uses
    the exact same trait that the server implements. For a given Binder
    interface, there is a single Rust trait generated that both client and
    server use.
- Use the same service identifier used when registering the service. This should
  ideally be defined in a common crate that both the client and server can
  depend on.

</details>
