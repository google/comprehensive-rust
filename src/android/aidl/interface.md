# AIDL Interfaces

You declare the API of your service using an AIDL interface:

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
{{#include birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:IBirthdayService}}
```

_birthday_service/aidl/Android.bp_:

```javascript
{{#include birthday_service/aidl/Android.bp}}
```

Add `vendor_available: true` if your AIDL file is used by a binary in the vendor
partition.
