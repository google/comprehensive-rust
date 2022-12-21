# AIDL Interfaces

You declare the API of your service using an AIDL interface:

*birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl*:

```java
{{#include birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:IBirthdayService}}
```

*birthday_service/aidl/Android.bp*:

```javascript
{{#include birthday_service/aidl/Android.bp}}
```

Add `vendor_available: true` if your AIDL file is used by a binary in the vendor
partition.
