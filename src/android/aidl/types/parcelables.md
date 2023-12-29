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

<details>

* Parcelables work similarly to serde types in the broader Rust ecosystem: You
  get a concrete Rust type to work with in your Rust code, and that type can be
  serializied without you needing to write any serialization logic yourself.

</details>
