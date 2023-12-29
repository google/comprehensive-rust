# Parcelables

Binder for Rust supports sending parcelables directly:

**IBirthdayService.aidl**:

```java
parcelable BirthdayInfo {
    String name;
    int years;
}

interface IBirthdayService {
    String wishHappyBirthday(BirthdayInfo info);
}
```

**client/src/main.rs**:

```rust,ignore
let msg = service.wishHappyBirthday(BirthdayInfo {
    name: String::from("Alice"),
    years: 29,
})?;
```
