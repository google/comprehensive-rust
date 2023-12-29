# Sending Objects

AIDL objects can be sent either as a concrete AIDL type or as the type-erased
`IBinder` interface:

**IBirthdayService.aidl**:

```java
interface IBDayInfoProvider {
    String getName();
    int getYears();
}

interface IBirthdayService {
    String wishHappyBirthday(IBDayInfoProvider provider);
    String wishHbdErased(IBinder anyType);
}
```

**client/src/main.rs**:

```rust,ignore
impl IBDayInfoProvider for MyProvider {
    fn getName(&self) -> String { "Alice".into() }
    fn getYears(&self) -> i32 { 29 }
}

let provider = MyProvider::new();
let msg = service.wishHappyBirthday(MyProvider)?; // TODO: Is this correct?
service.wishHbdErased(MyProvider)?; // TODO: Is this correct?
```
