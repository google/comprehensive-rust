# AIDL Client

Finally, we can create a Rust client for our new service.

_birthday_service/src/client.rs_:

```rust,ignore
{{#include birthday_service/src/client.rs:main}}
```

_birthday_service/Android.bp_:

```javascript
{{#include birthday_service/Android.bp:birthday_client}}
```

Notice that the client does not depend on `libbirthdayservice`.

Build, push, and run the client on your device:

```shell
{{#include ../build_all.sh:birthday_client}}
```

```text
Happy Birthday Charlie, congratulations with the 60 years!
```
