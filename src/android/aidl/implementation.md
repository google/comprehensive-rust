# Service Implementation

We can now implement the AIDL service:

_birthday_service/src/lib.rs_:

```rust,ignore
{{#include birthday_service/src/lib.rs:IBirthdayService}}
```

_birthday_service/Android.bp_:

```javascript
{{#include birthday_service/Android.bp:libbirthdayservice}}
```
