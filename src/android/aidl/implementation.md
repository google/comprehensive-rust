# Service Implementation

We can now implement the AIDL service:

*birthday_service/src/lib.rs*:

```rust,ignore
{{#include birthday_service/src/lib.rs:IBirthdayService}}
```

*birthday_service/Android.bp*:

```javascript
{{#include birthday_service/Android.bp:libbirthdayservice}}
```
