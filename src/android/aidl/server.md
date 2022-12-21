# AIDL Server

Finally, we can create a server which exposes the service:

*birthday_service/src/server.rs*:

```rust,ignore
{{#include birthday_service/src/server.rs:main}}
```

*birthday_service/Android.bp*:

```javascript
{{#include birthday_service/Android.bp:birthday_server}}
```
