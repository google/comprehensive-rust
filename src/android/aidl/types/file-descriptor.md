# Sending Files

Files can be sent between Binder clients/servers using the
`ParcelFileDescriptor` type:

```java
interface IBirthdayService {
    String wishHappyBirthday(ParcelFileDescriptor infoFile);
}
```

**client/src/main.rs**:

```rust,ignore
let info_file = File::open("info.txt")?;
let msg = service.wishHappyBirthday(info_file)?; // TODO: Is this correct? How do we convert to a `ParcelFileDescriptor`?
```
