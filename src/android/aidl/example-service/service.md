# Service Implementation

We can now implement the AIDL service:

_birthday_service/src/lib.rs_:

```rust,ignore
use com_example_birthdayservice::aidl::com::example::birthdayservice::{
    IBirthdayService::IBirthdayService
};
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
- TODO: What does the `binder::Interface` trait do? Are there methods to
  override? Where source?

</details>
