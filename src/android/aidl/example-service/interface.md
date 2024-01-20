# AIDL Interfaces

You declare the API of your service using an AIDL interface:

_birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl_:

```java
{{#include ../birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl:IBirthdayService}}
}
```

_birthday_service/aidl/Android.bp_:

```javascript
{{#include ../birthday_service/aidl/Android.bp}}
```

Add `vendor_available: true` if your AIDL file is used by a binary in the vendor
partition.

<details>

- Note that the directory structure under the `aidl/` directory needs to match
  the package name used in the AIDL file, i.e. the package is
  `com.example.birthdayservice` and the file is at
  `aidl/com/example/IBirthdayService.aidl`.

</details>
