<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Service Implementation

We can now implement the AIDL service:

_birthday_service/src/lib.rs_:

```rust,ignore
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
//! Implementation of the `IBirthdayService` AIDL interface.
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::IBirthdayService;
use com_example_birthdayservice::binder;

{{#include ../birthday_service/src/lib.rs:IBirthdayService}}
}
```

_birthday_service/Android.bp_:

```javascript
{{#include ../birthday_service/Android.bp:libbirthdayservice}}
```

<details>

- Point out the path to the generated `IBirthdayService` trait, and explain why
  each of the segments is necessary.
- Note that `wishHappyBirthday` and other AIDL IPC methods take `&self` (instead
  of `&mut self`).
  - This is necessary because Binder responds to incoming requests on a thread
    pool, allowing for multiple requests to be processed in parallel. This
    requires that the service methods only get a shared reference to `self`.
  - Any state that needs to be modified by the service will have to be put in
    something like a `Mutex` to allow for safe mutation.
  - The correct approach for managing service state depends heavily on the
    details of your service.
- TODO: What does the `binder::Interface` trait do? Are there methods to
  override? Where is the source?

</details>
