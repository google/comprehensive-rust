# AIDL Server

Finally, we can create a server which exposes the service:

_birthday_service/src/server.rs_:

```rust,ignore
{{#include birthday_service/src/server.rs:main}}
```

_birthday_service/Android.bp_:

```javascript
{{#include birthday_service/Android.bp:birthday_server}}
```
